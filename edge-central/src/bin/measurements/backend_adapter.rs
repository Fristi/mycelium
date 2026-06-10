use edge_client_backend::models::StationMeasurement;
use edge_protocol::translate::decode_events;
use edge_protocol::v2_proto::Events;
use edge_protocol::{MeasurementSerieEntry, WateringSerieEntry};

/// Domain view of peripheral events, materialized for backend and status display.
pub struct SyncPayload {
    pub measurements: Vec<MeasurementSerieEntry>,
    pub waterings: Vec<WateringSerieEntry>,
}

impl SyncPayload {
    pub fn from_events(events: Events) -> anyhow::Result<Self> {
        let decoded = decode_events::<32, 32>(events)
            .map_err(|_| anyhow::anyhow!("Failed to decode wire events"))?;

        Ok(Self {
            measurements: decoded.measurements.into_iter().collect(),
            waterings: decoded.waterings.into_iter().collect(),
        })
    }

    pub fn to_station_measurements(&self) -> Vec<StationMeasurement> {
        self.measurements
            .iter()
            .map(|measurement| StationMeasurement {
                on: measurement
                    .timestamp
                    .format("%Y-%m-%dT%H:%M:%SZ")
                    .to_string(),
                battery_voltage: 0_f64,
                temperature: measurement.value.temperature as f64,
                humidity: measurement.value.humidity as f64,
                lux: measurement.value.lux as f64,
                soil_pf: measurement.value.soil_pf as f64,
                tank_pf: 0_f64,
            })
            .collect()
    }
}
