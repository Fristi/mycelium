use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use edge_client_backend::apis::{default_api::CheckinStationError, Error as ApiError};
use edge_client_backend::models::{
    self, CheckinEvent,
    measurement::Type as MeasurementType,
    watering::Type as WateringType,
};
use edge_protocol::v2_proto::{Event_, Events, MeasurementRange, Timestamp, WateringEntry};

type ApiMeasurement = models::Measurement;

pub fn events_to_checkin(events: &Events) -> Result<Vec<CheckinEvent>> {
    let mut checkin_events = Vec::with_capacity(events.r#events.len());

    for event in &events.r#events {
        match &event.r#event {
            Some(Event_::Event::Measurement(range)) => {
                checkin_events.push(CheckinEvent::Measurement(Box::new(
                    measurement_range_to_api(range).with_context(|| "measurement event")?,
                )));
            }
            Some(Event_::Event::Watering(watering)) => {
                checkin_events.push(CheckinEvent::Watering(Box::new(
                    watering_entry_to_api(watering).with_context(|| "watering event")?,
                )));
            }
            None => {
                tracing::warn!("skipping event with empty oneof variant");
            }
        }
    }

    Ok(checkin_events)
}

pub fn log_checkin_station_error(
    err: &ApiError<CheckinStationError>,
    station_id: &uuid::Uuid,
    mac: &str,
    event_count: usize,
) {
    match err {
        ApiError::ResponseError(response) => {
            tracing::error!(
                %station_id,
                %mac,
                event_count,
                status = %response.status,
                body = %response.content,
                entity = ?response.entity,
                "station checkin request failed"
            );
        }
        _ => {
            tracing::error!(
                %station_id,
                %mac,
                event_count,
                error = %err,
                "station checkin request failed"
            );
        }
    }
}

fn measurement_range_to_api(range: &MeasurementRange) -> Result<ApiMeasurement> {
    let start = range
        .r#start
        .as_ref()
        .context("measurement missing start timestamp")?;
    let measurement = range
        .r#measurement
        .as_ref()
        .context("measurement missing sensor values")?;

    Ok(ApiMeasurement {
        start: proto_timestamp_to_rfc3339(start)?,
        end: range
            .r#end
            .as_ref()
            .map(proto_timestamp_to_rfc3339)
            .transpose()?,
        battery: i32::try_from(measurement.r#battery)
            .context("measurement battery out of range")?,
        lux: f64::from(measurement.r#lux),
        temperature: f64::from(measurement.r#temperature),
        humidity: f64::from(measurement.r#humidity),
        soil_moisture: f64::from(measurement.r#soil_moisture),
        _type: MeasurementType::Measurement,
    })
}

fn watering_entry_to_api(watering: &WateringEntry) -> Result<models::Watering> {
    let occurred_at = watering
        .r#occurred_at
        .as_ref()
        .context("watering missing occurred_at timestamp")?;

    Ok(models::Watering {
        occurred_at: proto_timestamp_to_rfc3339(occurred_at)?,
        duration_msec: i64::from(watering.r#duration_msec),
        _type: WateringType::Watering,
    })
}

fn proto_timestamp_to_rfc3339(ts: &Timestamp) -> Result<String> {
    DateTime::<Utc>::from_timestamp(ts.timestamp as i64, 0)
        .ok_or_else(|| anyhow!("invalid proto timestamp {}", ts.timestamp))
        .map(|dt| dt.to_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_client_backend::models::CheckinEvent;
    use edge_protocol::v2_proto::{
        Event, Measurement as ProtoMeasurement,
    };

    fn sample_measurement_range() -> MeasurementRange {
        let mut range = MeasurementRange::default();
        range.set_start(Timestamp { timestamp: 1_700_000_000 });
        range.set_end(Timestamp {
            timestamp: 1_700_000_300,
        });
        range.set_measurement(ProtoMeasurement {
            battery: 85,
            lux: 1200.5,
            temperature: 22.3,
            humidity: 55.0,
            soil_moisture: 35.0,
        });
        range
    }

    fn sample_watering_entry() -> WateringEntry {
        let mut watering = WateringEntry::default();
        watering.set_occurred_at(Timestamp {
            timestamp: 1_700_000_600,
        });
        watering.r#duration_msec = 15_000;
        watering
    }

    #[test]
    fn checkin_event_serializes_with_type_discriminator() {
        let events = events_to_checkin(&{
            let mut events = Events::default();
            events
                .r#events
                .push(Event {
                    r#event: Some(Event_::Event::Measurement(sample_measurement_range())),
                })
                .unwrap();
            events
        })
        .unwrap();

        let value = serde_json::to_value(&events).unwrap();
        let item = value.as_array().unwrap().first().unwrap();
        assert_eq!(item.get("_type").and_then(|v| v.as_str()), Some("Measurement"));
        assert!(item.get("soilMoisture").is_some());
        assert!(item.get("Measurement").is_none());
    }

    #[test]
    fn empty_events_returns_empty_vec() {
        let events = Events::default();
        let checkin = events_to_checkin(&events).unwrap();
        assert!(checkin.is_empty());
    }

    #[test]
    fn converts_measurement_and_watering() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range())),
            })
            .unwrap();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Watering(sample_watering_entry())),
            })
            .unwrap();

        let checkin = events_to_checkin(&events).unwrap();
        assert_eq!(checkin.len(), 2);

        let CheckinEvent::Measurement(measurement) = &checkin[0] else {
            panic!("expected measurement event");
        };
        assert_eq!(measurement.start, "2023-11-14T22:13:20+00:00");
        assert_eq!(
            measurement.end.as_deref(),
            Some("2023-11-14T22:18:20+00:00")
        );
        assert_eq!(measurement.battery, 85);
        assert!((measurement.lux - 1200.5).abs() < 1e-3);
        assert!((measurement.temperature - 22.3).abs() < 1e-3);
        assert!((measurement.humidity - 55.0).abs() < 1e-3);
        assert!((measurement.soil_moisture - 35.0).abs() < 1e-3);

        let CheckinEvent::Watering(watering) = &checkin[1] else {
            panic!("expected watering event");
        };
        assert_eq!(watering.occurred_at, "2023-11-14T22:23:20+00:00");
        assert_eq!(watering.duration_msec, 15_000);
    }

    #[test]
    fn missing_measurement_start_returns_error() {
        let mut range = MeasurementRange::default();
        range.set_measurement(ProtoMeasurement::default());

        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(range)),
            })
            .unwrap();

        assert!(events_to_checkin(&events).is_err());
    }

    #[test]
    fn skips_event_with_empty_variant() {
        let mut events = Events::default();
        events
            .r#events
            .push(Event {
                r#event: Some(Event_::Event::Measurement(sample_measurement_range())),
            })
            .unwrap();
        events.r#events.push(Event::default()).unwrap();

        let checkin = events_to_checkin(&events).unwrap();
        assert_eq!(checkin.len(), 1);
    }
}
