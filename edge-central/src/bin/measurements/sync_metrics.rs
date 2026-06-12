use chrono::{DateTime, Utc};
use edge_protocol::v2_proto::{Event_, Events, MeasurementRange, WateringEntry, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncSessionMetrics {
    pub measurement_count: u32,
    pub watering_count: u32,
    pub min_battery: Option<u8>,
    pub watered_at: Option<DateTime<Utc>>,
}

pub fn extract_sync_session_metrics(events: &Events) -> Option<SyncSessionMetrics> {
    let mut measurement_count = 0u32;
    let mut watering_count = 0u32;
    let mut min_battery = None::<u8>;
    let mut watered_at = None::<DateTime<Utc>>;

    for event in &events.r#events {
        match &event.r#event {
            Some(Event_::Event::Measurement(range)) => {
                measurement_count += 1;
                if let Some(battery) = measurement_battery(range) {
                    min_battery = Some(min_battery.map(|b| b.min(battery)).unwrap_or(battery));
                }
            }
            Some(Event_::Event::Watering(watering)) => {
                watering_count += 1;
                if let Some(occurred_at) = watering_occurred_at(watering) {
                    watered_at = Some(
                        watered_at
                            .map(|existing| existing.max(occurred_at))
                            .unwrap_or(occurred_at),
                    );
                }
            }
            None => {}
        }
    }

    if measurement_count == 0 && watering_count == 0 {
        return None;
    }

    Some(SyncSessionMetrics {
        measurement_count,
        watering_count,
        min_battery,
        watered_at,
    })
}

fn measurement_battery(range: &MeasurementRange) -> Option<u8> {
    let battery = range.r#measurement.as_ref()?.r#battery;
    u8::try_from(battery).ok()
}

fn watering_occurred_at(watering: &WateringEntry) -> Option<DateTime<Utc>> {
    proto_timestamp_to_datetime(watering.r#occurred_at.as_ref()?)
}

fn proto_timestamp_to_datetime(ts: &Timestamp) -> Option<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(ts.timestamp as i64, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_protocol::v2_proto::{
        Event, Measurement as ProtoMeasurement,
    };

    fn sample_measurement_range(battery: u32) -> MeasurementRange {
        let mut range = MeasurementRange::default();
        range.set_start(Timestamp { timestamp: 1_700_000_000 });
        range.set_measurement(ProtoMeasurement {
            battery,
            ..ProtoMeasurement::default()
        });
        range
    }

    fn sample_watering_entry(timestamp: u32) -> WateringEntry {
        let mut watering = WateringEntry::default();
        watering.set_occurred_at(Timestamp { timestamp });
        watering
    }

    #[test]
    fn empty_events_returns_none() {
        assert!(extract_sync_session_metrics(&Events::default()).is_none());
    }

    #[test]
    fn counts_measurements_and_waterings() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range(85))),
            })
            .unwrap();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Watering(sample_watering_entry(1_700_000_600))),
            })
            .unwrap();

        let metrics = extract_sync_session_metrics(&events).unwrap();
        assert_eq!(metrics.measurement_count, 1);
        assert_eq!(metrics.watering_count, 1);
        assert_eq!(metrics.min_battery, Some(85));
        assert_eq!(
            metrics.watered_at,
            DateTime::<Utc>::from_timestamp(1_700_000_600, 0)
        );
    }

    #[test]
    fn min_battery_across_ranges() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range(90))),
            })
            .unwrap();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range(72))),
            })
            .unwrap();

        let metrics = extract_sync_session_metrics(&events).unwrap();
        assert_eq!(metrics.min_battery, Some(72));
    }

    #[test]
    fn watered_at_is_most_recent() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Watering(sample_watering_entry(1_700_000_100))),
            })
            .unwrap();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Watering(sample_watering_entry(1_700_000_900))),
            })
            .unwrap();

        let metrics = extract_sync_session_metrics(&events).unwrap();
        assert_eq!(
            metrics.watered_at,
            DateTime::<Utc>::from_timestamp(1_700_000_900, 0)
        );
    }

    #[test]
    fn skips_empty_event_variants() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range(80))),
            })
            .unwrap();
        events.r#events.push(Event::default()).unwrap();

        let metrics = extract_sync_session_metrics(&events).unwrap();
        assert_eq!(metrics.measurement_count, 1);
    }
}
