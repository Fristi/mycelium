use std::sync::Arc;

use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

use crate::data::types::{EdgeState, StationSyncSummary, SyncSessionRecord};

pub struct SqliteEdgeStateRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteEdgeStateRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_state(&self) -> anyhow::Result<Option<EdgeState>> {
        let row = sqlx::query_as::<_, EdgeStateRow>(
            r#"
            SELECT wifi_ssid, wifi_password, auth0_access_token, auth0_refresh_token, auth0_expires_at
            FROM edge_state
            WHERE id = 1
            "#,
        )
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(row.map(|r| EdgeState {
            wifi_ssid: r.wifi_ssid,
            wifi_password: r.wifi_password,
            auth0_access_token: r.auth0_access_token,
            auth0_refresh_token: r.auth0_refresh_token,
            auth0_expires_at: r.auth0_expires_at,
        }))
    }

    pub async fn set_state(&self, state: &EdgeState) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO edge_state (id, wifi_ssid, wifi_password, auth0_access_token, auth0_refresh_token, auth0_expires_at)
            VALUES (1, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                wifi_ssid = excluded.wifi_ssid,
                wifi_password = excluded.wifi_password,
                auth0_access_token = excluded.auth0_access_token,
                auth0_refresh_token = excluded.auth0_refresh_token,
                auth0_expires_at = excluded.auth0_expires_at
            "#,
        )
        .bind(&state.wifi_ssid)
        .bind(&state.wifi_password)
        .bind(&state.auth0_access_token)
        .bind(&state.auth0_refresh_token)
        .bind(state.auth0_expires_at)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }
}

pub struct SqliteSyncSessionRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteSyncSessionRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn insert_session(&self, record: &SyncSessionRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sync_sessions (
                synced_at,
                station_id,
                mac,
                measurement_count,
                watering_count,
                min_battery,
                time_drift_secs,
                watered_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(record.synced_at)
        .bind(&record.station_id)
        .bind(&record.mac)
        .bind(record.measurement_count)
        .bind(record.watering_count)
        .bind(record.min_battery)
        .bind(record.time_drift_secs)
        .bind(record.watered_at)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn list_station_summaries(&self) -> anyhow::Result<Vec<StationSyncSummary>> {
        let rows = sqlx::query_as::<_, StationSyncSummaryRow>(
            r#"
            SELECT
                station_id,
                MIN(mac) AS mac,
                SUM(measurement_count) AS measurement_count,
                SUM(watering_count) AS watering_count,
                MIN(min_battery) AS min_battery,
                CAST(ROUND(AVG(time_drift_secs)) AS INTEGER) AS avg_time_drift_secs,
                MAX(watered_at) AS max_watered_at,
                MAX(synced_at) AS max_synced_at
            FROM sync_sessions
            GROUP BY station_id
            ORDER BY max_synced_at DESC
            "#,
        )
        .fetch_all(self.pool.as_ref())
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| StationSyncSummary {
                station_id: row.station_id,
                mac: row.mac,
                measurement_count: row.measurement_count,
                watering_count: row.watering_count,
                min_battery: row.min_battery,
                avg_time_drift_secs: row.avg_time_drift_secs,
                max_watered_at: row.max_watered_at,
                max_synced_at: row.max_synced_at,
            })
            .collect())
    }
}

#[derive(Debug, FromRow)]
struct StationSyncSummaryRow {
    station_id: String,
    mac: String,
    measurement_count: i64,
    watering_count: i64,
    min_battery: Option<i64>,
    avg_time_drift_secs: i64,
    max_watered_at: Option<chrono::DateTime<chrono::Utc>>,
    max_synced_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
struct EdgeStateRow {
    wifi_ssid: String,
    wifi_password: String,
    auth0_access_token: String,
    auth0_refresh_token: String,
    auth0_expires_at: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::FromRow;

    #[derive(Debug, FromRow, PartialEq, Eq)]
    struct SyncSessionRow {
        station_id: String,
        mac: String,
        measurement_count: i32,
        watering_count: i32,
        min_battery: Option<i32>,
        time_drift_secs: i64,
    }
    use chrono::Utc;
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
    use std::str::FromStr;

    async fn test_pool() -> Arc<SqlitePool> {
        let opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePool::connect_with(opts).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        Arc::new(pool)
    }

    #[tokio::test]
    async fn insert_sync_session_round_trip() {
        let pool = test_pool().await;
        let repo = SqliteSyncSessionRepository::new(pool.clone());

        let record = SyncSessionRecord {
            synced_at: Utc::now(),
            station_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            mac: "40:f5:20:b7:85:40".to_string(),
            measurement_count: 3,
            watering_count: 1,
            min_battery: Some(72),
            time_drift_secs: -4,
            watered_at: chrono::DateTime::<Utc>::from_timestamp(1_700_000_600, 0),
        };

        repo.insert_session(&record).await.unwrap();

        let row = sqlx::query_as::<_, SyncSessionRow>(
            r#"
            SELECT station_id, mac, measurement_count, watering_count, min_battery, time_drift_secs
            FROM sync_sessions
            WHERE station_id = ?
            "#,
        )
        .bind(&record.station_id)
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

        assert_eq!(
            row,
            SyncSessionRow {
                station_id: record.station_id,
                mac: record.mac,
                measurement_count: record.measurement_count,
                watering_count: record.watering_count,
                min_battery: record.min_battery,
                time_drift_secs: record.time_drift_secs,
            }
        );
    }

    #[tokio::test]
    async fn list_station_summaries_aggregates_by_station_id() {
        let pool = test_pool().await;
        let repo = SqliteSyncSessionRepository::new(pool);

        let station_a = "550e8400-e29b-41d4-a716-446655440000";
        let station_b = "660e8400-e29b-41d4-a716-446655440001";

        let sessions = [
            SyncSessionRecord {
                synced_at: chrono::DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
                station_id: station_a.to_string(),
                mac: "40:f5:20:b7:85:40".to_string(),
                measurement_count: 2,
                watering_count: 1,
                min_battery: Some(80),
                time_drift_secs: -4,
                watered_at: chrono::DateTime::<Utc>::from_timestamp(1_700_000_100, 0),
            },
            SyncSessionRecord {
                synced_at: chrono::DateTime::<Utc>::from_timestamp(1_700_001_000, 0).unwrap(),
                station_id: station_a.to_string(),
                mac: "40:f5:20:b7:85:40".to_string(),
                measurement_count: 3,
                watering_count: 0,
                min_battery: Some(72),
                time_drift_secs: -2,
                watered_at: None,
            },
            SyncSessionRecord {
                synced_at: chrono::DateTime::<Utc>::from_timestamp(1_700_002_000, 0).unwrap(),
                station_id: station_b.to_string(),
                mac: "aa:bb:cc:dd:ee:ff".to_string(),
                measurement_count: 5,
                watering_count: 2,
                min_battery: Some(90),
                time_drift_secs: 1,
                watered_at: chrono::DateTime::<Utc>::from_timestamp(1_700_001_500, 0),
            },
        ];

        for record in &sessions {
            repo.insert_session(record).await.unwrap();
        }

        let summaries = repo.list_station_summaries().await.unwrap();
        assert_eq!(summaries.len(), 2);

        let a = summaries
            .iter()
            .find(|s| s.station_id == station_a)
            .expect("station a summary");
        assert_eq!(a.mac, "40:f5:20:b7:85:40");
        assert_eq!(a.measurement_count, 5);
        assert_eq!(a.watering_count, 1);
        assert_eq!(a.min_battery, Some(72));
        assert_eq!(a.avg_time_drift_secs, -3);
        assert_eq!(
            a.max_watered_at,
            chrono::DateTime::<Utc>::from_timestamp(1_700_000_100, 0)
        );
        assert_eq!(
            a.max_synced_at,
            chrono::DateTime::<Utc>::from_timestamp(1_700_001_000, 0).unwrap()
        );

        let b = summaries
            .iter()
            .find(|s| s.station_id == station_b)
            .expect("station b summary");
        assert_eq!(b.measurement_count, 5);
        assert_eq!(b.watering_count, 2);
        assert_eq!(b.avg_time_drift_secs, 1);
        assert_eq!(
            b.max_synced_at,
            chrono::DateTime::<Utc>::from_timestamp(1_700_002_000, 0).unwrap()
        );
    }
}
