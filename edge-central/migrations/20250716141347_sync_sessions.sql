DROP TABLE IF EXISTS measurements;

CREATE TABLE sync_sessions (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    synced_at         DATETIME NOT NULL,
    station_id        TEXT NOT NULL,
    mac               TEXT NOT NULL,
    measurement_count INTEGER NOT NULL,
    watering_count    INTEGER NOT NULL,
    min_battery       INTEGER,
    time_drift_secs   INTEGER NOT NULL,
    watered_at        DATETIME
);

CREATE INDEX idx_sync_sessions_mac_synced_at ON sync_sessions (mac, synced_at DESC);
CREATE INDEX idx_sync_sessions_station_id ON sync_sessions (station_id, synced_at DESC);
