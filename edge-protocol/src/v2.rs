//! v2 BLE protocol — re-exports wire codec and translate layer.
//!
//! Prefer importing from `edge_protocol::wire` or `edge_protocol::translate` in new code.

pub use crate::translate::{
    decode_events, encode_device_buffer, encode_measurement_series, encode_measurements_only,
    encode_watering_vec, mac_address_from_bytes, mac_address_to_bytes, DecodedEvents, MAX_EVENTS,
};
pub use crate::wire::{
    decode_proto, encode_proto, STATION_CURRENT_TIME_CHARACTERISTIC_UUID,
    STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16,
    STATION_EVENTS_CHARACTERISTIC_UUID, STATION_EVENTS_CHARACTERISTIC_UUID_16,
    STATION_MAC_ADDR_CHARACTERISTIC_UUID, STATION_MAC_ADDR_CHARACTERISTIC_UUID_16,
    STATION_PLANT_PROFILE_CHARACTERISTIC_UUID, STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16,
    STATION_SERVICE_UUID, STATION_SERVICE_UUID_16, STATION_SYNC_STATE_CHARACTERISTIC_UUID,
    STATION_SYNC_STATE_CHARACTERISTIC_UUID_16,
};

// Backward-compatible aliases
pub use crate::translate::encode_device_buffer as device_state_data_to_events;
pub use crate::translate::encode_measurement_series as measurement_events_from_series;
pub use crate::translate::encode_measurements_only as measurements_to_events;
pub use crate::translate::encode_watering_vec as watering_events_from_vec;

/// Decode measurement entries from wire events (direct 1:1 mapping).
pub fn events_to_measurement_entries<const N: usize>(
    events: crate::v2_proto::Events,
) -> Result<heapless::Vec<crate::MeasurementSerieEntry, N>, ()> {
    Ok(decode_events::<N, N>(events)?.measurements)
}

/// Decode watering entries from wire events (direct 1:1 mapping).
pub fn events_to_watering_entries<const N: usize>(
    events: crate::v2_proto::Events,
) -> Result<heapless::Vec<crate::WateringSerieEntry, N>, ()> {
    Ok(decode_events::<N, N>(events)?.waterings)
}
