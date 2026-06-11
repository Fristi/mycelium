-- Replace generic event log with typed watering table
DROP TABLE station_log;

CREATE TABLE station_waterings (
    station_id    UUID NOT NULL REFERENCES stations (id) ON DELETE CASCADE,
    occurred_at   TIMESTAMPTZ NOT NULL,
    duration_msec INTEGER NOT NULL
);

-- Align measurements with v2.proto MeasurementRange + Measurement
ALTER TABLE station_measurements
    ADD COLUMN ended_on TIMESTAMPTZ,
    DROP COLUMN tank_pf;

ALTER TABLE station_measurements
    RENAME COLUMN battery_voltage TO battery;

ALTER TABLE station_measurements
    ALTER COLUMN battery TYPE INTEGER USING battery::integer;
