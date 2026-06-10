//! Domain ↔ wire translations. All conversions between `lib.rs` types and `v2_proto` types live here.

use crate::v2_proto::{
    Event, Event_, Events, MacAddress, Measurement as ProtoMeasurement, MeasurementEntry,
    Timestamp, WateringEntry,
};
use crate::{Measurement, MeasurementSerieEntry, WateringSerieEntry};
use chrono::NaiveDateTime;
use heapless::Vec as HVec;
use timeseries::Series;

pub const MAX_EVENTS: usize = 16;

/// Build a wire `MacAddress` from six raw bytes (including zeros).
pub fn mac_address_from_bytes(value: [u8; 6]) -> Result<MacAddress, ()> {
    let mac_address = heapless::Vec::from_slice(&value).map_err(|_| ())?;
    Ok(MacAddress { mac_address })
}

/// Extract six raw bytes from a wire `MacAddress`.
pub fn mac_address_to_bytes(value: &MacAddress) -> Result<[u8; 6], ()> {
    value
        .r#mac_address()
        .clone()
        .into_array()
        .map_err(|_| ())
}

/// Flat domain view of a decoded `Events` payload.
pub struct DecodedEvents<const M: usize, const W: usize> {
    pub measurements: HVec<MeasurementSerieEntry, M>,
    pub waterings: HVec<WateringSerieEntry, W>,
}

fn lib_measurement_to_proto(m: Measurement) -> ProtoMeasurement {
    ProtoMeasurement {
        r#battery: m.battery as u32,
        r#lux: m.lux,
        r#temperature: m.temperature,
        r#humidity: m.humidity,
        r#soil_pf: m.soil_pf,
    }
}

fn proto_measurement_to_lib(m: &ProtoMeasurement) -> Measurement {
    Measurement {
        battery: m.r#battery as u8,
        lux: m.r#lux,
        temperature: m.r#temperature,
        humidity: m.r#humidity,
        soil_pf: m.r#soil_pf,
    }
}

fn naive_to_timestamp(dt: NaiveDateTime) -> Result<Timestamp, ()> {
    let ts = dt.and_utc().timestamp();
    if ts < 0 || ts > u32::MAX as i64 {
        return Err(());
    }
    Ok(Timestamp {
        timestamp: ts as u32,
    })
}

fn timestamp_to_naive(ts: u32) -> Result<NaiveDateTime, ()> {
    chrono::DateTime::from_timestamp(ts as i64, 0)
        .ok_or(())
        .map(|dt| dt.naive_utc())
}

fn event_timestamp(event: &Event) -> Result<u32, ()> {
    match &event.event {
        Some(Event_::Event::Measurement(e)) => Ok(e.start.timestamp),
        Some(Event_::Event::Watering(e)) => e
            .r#occurred_at()
            .map(|t| t.timestamp)
            .ok_or(()),
        None => Err(()),
    }
}

/// Encode measurement buckets as protobuf measurement events.
pub fn encode_measurement_series<const N: usize>(
    measurements: &Series<N, NaiveDateTime, Measurement>,
) -> Result<HVec<Event, MAX_EVENTS>, ()> {
    let mut events = HVec::<Event, MAX_EVENTS>::new();

    for se in &measurements.buckets {
        let mut entry = MeasurementEntry::default();
        entry.set_start(naive_to_timestamp(se.range.start)?);
        if let Some(end) = se.range.end {
            entry.set_end(naive_to_timestamp(end)?);
        }
        entry.set_measurement(lib_measurement_to_proto(se.value));

        events
            .push(Event {
                event: Some(Event_::Event::Measurement(entry)),
            })
            .map_err(|_| ())?;
    }

    Ok(events)
}

/// Encode watering entries as protobuf watering events.
pub fn encode_watering_vec<const W: usize>(
    waterings: &HVec<WateringSerieEntry, W>,
) -> Result<HVec<Event, MAX_EVENTS>, ()> {
    let mut events = HVec::<Event, MAX_EVENTS>::new();

    for w in waterings {
        let mut entry = WateringEntry::default();
        entry.set_occurred_at(naive_to_timestamp(w.timestamp)?);
        entry.set_duration_msec(w.value.duration_msec);

        events
            .push(Event {
                event: Some(Event_::Event::Watering(entry)),
            })
            .map_err(|_| ())?;
    }

    Ok(events)
}

/// Encode measurements and waterings as a single chronologically merged `Events` payload.
pub fn encode_device_buffer<const N: usize, const W: usize>(
    measurements: &Series<N, NaiveDateTime, Measurement>,
    waterings: &HVec<WateringSerieEntry, W>,
) -> Result<Events, ()> {
    let measurement_events = encode_measurement_series(measurements)?;
    let watering_events = encode_watering_vec(waterings)?;

    if measurement_events.len() + watering_events.len() > MAX_EVENTS {
        return Err(());
    }

    let mut merged = HVec::<Event, MAX_EVENTS>::new();
    let mut mi = 0;
    let mut wi = 0;

    while mi < measurement_events.len() || wi < watering_events.len() {
        let take_measurement = if mi >= measurement_events.len() {
            false
        } else if wi >= watering_events.len() {
            true
        } else {
            event_timestamp(&measurement_events[mi])? <= event_timestamp(&watering_events[wi])?
        };

        if take_measurement {
            merged
                .push(measurement_events[mi].clone())
                .map_err(|_| ())?;
            mi += 1;
        } else {
            merged
                .push(watering_events[wi].clone())
                .map_err(|_| ())?;
            wi += 1;
        }
    }

    Ok(Events { events: merged })
}

/// Encode buffered measurement series as protobuf events (measurements only).
pub fn encode_measurements_only<const N: usize>(
    measurements: &Series<N, NaiveDateTime, Measurement>,
) -> Result<Events, ()> {
    let events = encode_measurement_series(measurements)?;
    Ok(Events { events })
}

/// Decode wire events into flat domain entries (1:1 per event; no re-bucketing).
pub fn decode_events<const M: usize, const W: usize>(
    events: Events,
) -> Result<DecodedEvents<M, W>, ()> {
    let mut measurements = HVec::new();
    let mut waterings = HVec::new();

    for event in events.events {
        match event.event {
            Some(Event_::Event::Measurement(e)) => {
                measurements
                    .push(MeasurementSerieEntry {
                        timestamp: timestamp_to_naive(e.start.timestamp)?,
                        value: proto_measurement_to_lib(&e.measurement),
                    })
                    .map_err(|_| ())?;
            }
            Some(Event_::Event::Watering(e)) => {
                let occurred_at = e.r#occurred_at().ok_or(())?;
                waterings
                    .push(WateringSerieEntry {
                        timestamp: timestamp_to_naive(occurred_at.timestamp)?,
                        value: crate::Watering {
                            duration_msec: e.r#duration_msec,
                        },
                    })
                    .map_err(|_| ())?;
            }
            None => {}
        }
    }

    Ok(DecodedEvents {
        measurements,
        waterings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::decode_proto;
    use micropb::{MessageEncode, PbEncoder};

    #[test]
    fn encode_measurements_roundtrip() {
        let mut measurements: Series<6, NaiveDateTime, Measurement> =
            Series::new(Measurement::MAX_DEVIATION);
        let t0 = NaiveDateTime::from_timestamp_opt(1_700_000_000, 0).unwrap();
        measurements.append_monotonic(
            t0,
            Measurement {
                battery: 90,
                lux: 100.0,
                temperature: 21.0,
                humidity: 55.0,
                soil_pf: 800.0,
            },
        );

        let events = encode_measurements_only(&measurements).unwrap();
        let decoded = decode_events::<6, 4>(events).unwrap();
        assert_eq!(decoded.measurements.len(), 1);
        assert_eq!(decoded.measurements[0].timestamp, t0);
        assert_eq!(decoded.measurements[0].value.battery, 90);
    }

    #[test]
    fn watering_roundtrip() {
        let t0 = NaiveDateTime::from_timestamp_opt(1_700_000_100, 0).unwrap();
        let mut waterings = HVec::<WateringSerieEntry, 4>::new();
        waterings
            .push(WateringSerieEntry {
                timestamp: t0,
                value: crate::Watering {
                    duration_msec: 5000,
                },
            })
            .unwrap();

        let events = encode_watering_vec(&waterings).unwrap();
        let decoded = decode_events::<4, 4>(Events { events }).unwrap();
        assert_eq!(decoded.waterings.len(), 1);
        assert_eq!(decoded.waterings[0].timestamp, t0);
        assert_eq!(decoded.waterings[0].value.duration_msec, 5000);
    }

    #[test]
    fn mixed_events_roundtrip() {
        let mut measurements: Series<6, NaiveDateTime, Measurement> =
            Series::new(Measurement::MAX_DEVIATION);
        let t_meas = NaiveDateTime::from_timestamp_opt(1_700_000_000, 0).unwrap();
        measurements.append_monotonic(
            t_meas,
            Measurement {
                battery: 90,
                lux: 100.0,
                temperature: 21.0,
                humidity: 55.0,
                soil_pf: 800.0,
            },
        );

        let mut waterings = HVec::<WateringSerieEntry, 4>::new();
        let t_water = NaiveDateTime::from_timestamp_opt(1_700_000_200, 0).unwrap();
        waterings
            .push(WateringSerieEntry {
                timestamp: t_water,
                value: crate::Watering {
                    duration_msec: 3000,
                },
            })
            .unwrap();

        let events = encode_device_buffer(&measurements, &waterings).unwrap();
        assert_eq!(events.events.len(), 2);
        assert!(matches!(
            events.events[0].event,
            Some(Event_::Event::Measurement(_))
        ));
        assert!(matches!(
            events.events[1].event,
            Some(Event_::Event::Watering(_))
        ));

        let decoded = decode_events::<6, 4>(events).unwrap();
        assert_eq!(decoded.measurements.len(), 1);
        assert_eq!(decoded.waterings.len(), 1);
        assert_eq!(decoded.waterings[0].value.duration_msec, 3000);
    }

    #[test]
    fn encode_overflow() {
        let mut measurements: Series<6, NaiveDateTime, Measurement> =
            Series::new(Measurement::MAX_DEVIATION);
        for i in 0..6 {
            let t = NaiveDateTime::from_timestamp_opt(1_700_000_000 + i * 60, 0).unwrap();
            measurements.append_monotonic(
                t,
                Measurement {
                    battery: 90,
                    lux: 100.0,
                    temperature: 21.0 + i as f32,
                    humidity: 55.0,
                    soil_pf: 800.0,
                },
            );
        }

        let mut waterings = HVec::<WateringSerieEntry, 16>::new();
        for i in 0..11 {
            let t = NaiveDateTime::from_timestamp_opt(1_700_001_000 + i * 60, 0).unwrap();
            waterings
                .push(WateringSerieEntry {
                    timestamp: t,
                    value: crate::Watering {
                        duration_msec: 1000,
                    },
                })
                .unwrap();
        }

        assert!(encode_device_buffer(&measurements, &waterings).is_err());
    }

    #[test]
    fn mac_address_roundtrip_preserves_zero_bytes() {
        use crate::wire::{decode_proto, encode_proto};

        let mac = [0x00, 0x00, 0xf5, 0x20, 0xb7, 0x85];
        let proto = mac_address_from_bytes(mac).unwrap();
        let mut buf = [0u8; 16];
        let len = encode_proto(&proto, &mut buf).unwrap();
        assert_eq!(len, 8, "protobuf frames length-delimited bytes");

        let decoded = decode_proto::<MacAddress>(&buf[..len]).unwrap();
        assert_eq!(mac_address_to_bytes(&decoded).unwrap(), mac);
    }

    #[test]
    fn events_encode_decode_roundtrip() {
        let events = Events::default();
        let mut buf = [0u8; 256];
        let mut writer = buf.as_mut_slice();
        let mut encoder = PbEncoder::new(&mut writer);
        events.encode(&mut encoder).unwrap();
        let len = 256 - writer.len();
        let decoded = decode_proto::<Events>(&buf[..len]).unwrap();
        assert_eq!(events, decoded);
    }
}
