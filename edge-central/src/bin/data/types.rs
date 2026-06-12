use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeState {
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub auth0_access_token: String,
    pub auth0_refresh_token: String,
    pub auth0_expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncSessionRecord {
    pub synced_at: DateTime<Utc>,
    pub station_id: String,
    pub mac: String,
    pub measurement_count: i32,
    pub watering_count: i32,
    pub min_battery: Option<i32>,
    pub time_drift_secs: i64,
    pub watered_at: Option<DateTime<Utc>>,
}
