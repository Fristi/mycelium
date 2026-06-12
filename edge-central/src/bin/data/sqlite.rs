use std::sync::Arc;

use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

use crate::data::types::{EdgeState, SyncSessionRecord};

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
}
