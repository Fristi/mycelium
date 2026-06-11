use std::sync::Arc;

use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::data::types::EdgeState;

#[derive(Debug, sqlx::FromRow)]
pub struct EdgeStateRow {
    pub id: i64,
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub auth0_access_token: String,
    pub auth0_refresh_token: String,
    pub auth0_expires_at: NaiveDateTime,
}

impl EdgeStateRow {
    pub fn from_edge_state(state: &crate::data::types::EdgeState) -> Self {
        EdgeStateRow {
            // We only store one edge state, so the identifier is hard-coded to 1
            id: 1,
            wifi_ssid: state.wifi_ssid.clone(),
            wifi_password: state.wifi_password.clone(),
            auth0_access_token: state.auth0_access_token.clone(),
            auth0_refresh_token: state.auth0_refresh_token.clone(),
            auth0_expires_at: state.auth0_expires_at,
        }
    }

    pub fn to_edge_state(&self) -> crate::data::types::EdgeState {
        crate::data::types::EdgeState {
            wifi_ssid: self.wifi_ssid.clone(),
            wifi_password: self.wifi_password.clone(),
            auth0_access_token: self.auth0_access_token.clone(),
            auth0_refresh_token: self.auth0_refresh_token.clone(),
            auth0_expires_at: self.auth0_expires_at,
        }
    }
}

pub struct SqliteEdgeStateRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteEdgeStateRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_state(&self) -> anyhow::Result<Option<EdgeState>> {
        let row: Option<EdgeStateRow> = sqlx::query_as(
            "
            SELECT id, wifi_ssid, wifi_password, auth0_access_token, auth0_refresh_token, auth0_expires_at
            FROM edge_state
            LIMIT 1
            "
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(row.map(|r| r.to_edge_state()))
    }

    pub async fn set_state(&self, state: &EdgeState) -> anyhow::Result<u64> {
        let row = EdgeStateRow::from_edge_state(&state);

        let res = sqlx::query(
            "
            INSERT INTO edge_state (id, wifi_ssid, wifi_password, auth0_access_token, auth0_refresh_token, auth0_expires_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(id) DO UPDATE SET
                wifi_ssid = excluded.wifi_ssid,
                wifi_password = excluded.wifi_password,
                auth0_access_token = excluded.auth0_access_token,
                auth0_refresh_token = excluded.auth0_refresh_token,
                auth0_expires_at = excluded.auth0_expires_at
            "
        )
        .bind(row.id)
        .bind(row.wifi_ssid)
        .bind(row.wifi_password)
        .bind(row.auth0_access_token)
        .bind(row.auth0_refresh_token)
        .bind(row.auth0_expires_at)
        .execute(&*self.pool)
        .await?;

        Ok(res.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_edge_state_get_none() {
        let pool = Arc::new(
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:?cache=shared")
                .await
                .expect("Failed to create pool")
        );

        // Run migrations to create the edge_state table
        sqlx::migrate!()
            .run(&*pool)
            .await
            .expect("Failed to run migrations");

        let repo = SqliteEdgeStateRepository::new(pool.clone());

        // There should be no record yet
        let result = repo.get_state().await.expect("Unable to get");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_edge_state_set_and_get_some() {
        let pool = Arc::new(
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:?cache=shared")
                .await
                .expect("Failed to create pool")
        );

        sqlx::migrate!()
            .run(&*pool)
            .await
            .expect("Failed to run migrations");

        let repo = SqliteEdgeStateRepository::new(pool.clone());

        let state = EdgeState {
            wifi_ssid: "ssid1".to_string(),
            wifi_password: "pass1".to_string(),
            auth0_access_token: "token1".to_string(),
            auth0_refresh_token: "refresh1".to_string(),
            auth0_expires_at: Utc.timestamp_opt(1_700_000_000, 0).unwrap().naive_utc(),
        };
        // Set the state
        let affected = repo.set_state(&state).await.expect("Unable to set state");
        assert_eq!(affected, 1);

        // Get the state
        let result = repo.get_state().await.expect("Unable to get state");
        assert!(result.is_some());
        let loaded = result.unwrap();
        assert_eq!(loaded.wifi_ssid, state.wifi_ssid);
        assert_eq!(loaded.wifi_password, state.wifi_password);
        assert_eq!(loaded.auth0_access_token, state.auth0_access_token);
        assert_eq!(loaded.auth0_refresh_token, state.auth0_refresh_token);
        assert_eq!(loaded.auth0_expires_at, state.auth0_expires_at);
    }

    #[tokio::test]
    async fn test_edge_state_set_overwrites() {
        let pool = Arc::new(
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:?cache=shared")
                .await
                .expect("Failed to create pool")
        );

        sqlx::migrate!()
            .run(&*pool)
            .await
            .expect("Failed to run migrations");

        let repo = SqliteEdgeStateRepository::new(pool.clone());

        let state1 = EdgeState {
            wifi_ssid: "ssid1".to_string(),
            wifi_password: "pass1".to_string(),
            auth0_access_token: "token1".to_string(),
            auth0_refresh_token: "refresh1".to_string(),
            auth0_expires_at: Utc.timestamp_opt(1_700_000_000, 0).unwrap().naive_utc(),
        };

        let state2 = EdgeState {
            wifi_ssid: "ssid1".to_string(),
            wifi_password: "pass2".to_string(),
            auth0_access_token: "token2".to_string(),
            auth0_refresh_token: "refresh2".to_string(),
            auth0_expires_at: Utc.timestamp_opt(1_800_000_000, 0).unwrap().naive_utc(),
        };

        // Set the first state
        let affected1 = repo.set_state(&state1).await.expect("Unable to set state");
        assert_eq!(affected1, 1);

        // Set the second state (should update)
        repo.set_state(&state2).await.expect("Unable to set state");

        // Get the state and check it's the updated one
        let result = repo.get_state().await.expect("Unable to get state");
        assert!(result.is_some());
        let loaded = result.unwrap();
        assert_eq!(loaded.wifi_ssid, state2.wifi_ssid);
        assert_eq!(loaded.wifi_password, state2.wifi_password);
        assert_eq!(loaded.auth0_access_token, state2.auth0_access_token);
        assert_eq!(loaded.auth0_refresh_token, state2.auth0_refresh_token);
        assert_eq!(loaded.auth0_expires_at, state2.auth0_expires_at);
}
}
