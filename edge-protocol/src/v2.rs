use crate::v2_proto::{
    Event, Event_, Events, MacAddress, Measurement as ProtoMeasurement, MeasurementEntry,
    Timestamp,
};
use crate::{Measurement, MeasurementSerieEntry};
use chrono::NaiveDateTime;
use heapless::Vec as HVec;
use core::convert::Infallible;
use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use timeseries::{Deviate, Series};

pub const STATION_SERVICE_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xAA];
pub const STATION_MAC_ADDR_CHARACTERISTIC_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xBA];
pub const STATION_PLANT_PROFILE_CHARACTERISTIC_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xBB];
pub const STATION_EVENTS_CHARACTERISTIC_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xBC];
pub const STATION_CURRENT_TIME_CHARACTERISTIC_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xBD];
pub const STATION_SYNC_STATE_CHARACTERISTIC_UUID: [u8; 16] =
    [0x01, 0x14, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xBE];

pub const STATION_SERVICE_UUID_16: u16 = uuid16(STATION_SERVICE_UUID);
pub const STATION_MAC_ADDR_CHARACTERISTIC_UUID_16: u16 = uuid16(STATION_MAC_ADDR_CHARACTERISTIC_UUID);
pub const STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16: u16 =
    uuid16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID);
pub const STATION_EVENTS_CHARACTERISTIC_UUID_16: u16 = uuid16(STATION_EVENTS_CHARACTERISTIC_UUID);
pub const STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16: u16 =
    uuid16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID);
pub const STATION_SYNC_STATE_CHARACTERISTIC_UUID_16: u16 = uuid16(STATION_SYNC_STATE_CHARACTERISTIC_UUID);

const fn uuid16(uuid: [u8; 16]) -> u16 {
    u16::from_le_bytes([uuid[14], uuid[15]])
}

/// Encode a protobuf message into `buf`. Returns the number of bytes written.
pub fn encode_proto<T: MessageEncode>(msg: &T, buf: &mut [u8]) -> Result<usize, ()> {
    let size = MessageEncode::compute_size(msg);
    if size > buf.len() {
        return Err(());
    }
    let mut writer = &mut buf[..size];
    let mut encoder = PbEncoder::new(&mut writer);
    msg.encode(&mut encoder).map_err(|_| ())?;
    Ok(size)
}

/// Decode a protobuf message from a BLE characteristic value.
///
/// Uses the exact byte length returned by the ATT read; do not pad or trim the buffer.
pub fn decode_proto<T: MessageDecode + Default>(
    data: &[u8],
) -> Result<T, DecodeError<Infallible>> {
    if data.is_empty() {
        return Ok(T::default());
    }
    let mut message = T::default();
    let mut decoder = PbDecoder::new(data);
    message.decode(&mut decoder, data.len())?;
    Ok(message)
}

/// Decode a 6-byte station MAC from a MAC-address characteristic value.
pub fn decode_mac_address(data: &[u8]) -> Result<[u8; 6], DecodeError<Infallible>> {
    if data.len() == 6 {
        let mut mac = [0u8; 6];
        mac.copy_from_slice(data);
        return Ok(mac);
    }

    let mac_addr: MacAddress = decode_proto(data)?;
    mac_addr
        .r#mac_address
        .clone()
        .into_array()
        .map_err(|_| DecodeError::UnexpectedEof)
}

const MAX_DEVIATION_MEASUREMENT: ProtoMeasurement = ProtoMeasurement {
    r#battery: 1,
    r#lux: 0.1,
    r#temperature: 0.1,
    r#humidity: 0.1,
    r#soil_pf: 0.1,
};

impl Deviate for ProtoMeasurement {
    fn deviate(&self, other: &Self, max_deviation: &Self) -> bool {
        (self.r#temperature - other.r#temperature).abs() > max_deviation.r#temperature
            || (self.r#humidity - other.r#humidity).abs() > max_deviation.r#humidity
            || self.r#battery.abs_diff(other.r#battery) > max_deviation.r#battery
            || (self.r#lux - other.r#lux).abs() > max_deviation.r#lux
            || (self.r#soil_pf - other.r#soil_pf).abs() > max_deviation.r#soil_pf
    }
}

impl TryFrom<MacAddress> for [u8; 6] {
    type Error = heapless::Vec<u8, 6>;

    fn try_from(value: MacAddress) -> Result<Self, Self::Error> {
        value.mac_address.into_array()
    }
}

impl TryFrom<[u8; 6]> for MacAddress {
    type Error = ();

    fn try_from(value: [u8; 6]) -> Result<Self, Self::Error> {
        let mac_address = heapless::Vec::from_slice(&value)?;
        Ok(MacAddress { mac_address })
    }
}

impl From<u32> for Timestamp {
    fn from(value: u32) -> Self {
        Timestamp { timestamp: value }
    }
}

impl From<Timestamp> for u32 {
    fn from(value: Timestamp) -> Self {
        value.timestamp
    }
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

fn naive_to_timestamp(dt: NaiveDateTime) -> Result<Timestamp, ()> {
    let ts = dt.and_utc().timestamp();
    if ts < 0 || ts > u32::MAX as i64 {
        return Err(());
    }
    Ok(Timestamp {
        timestamp: ts as u32,
    })
}

/// Encode buffered measurement series as protobuf events for BLE sync.
pub fn measurements_to_events<const N: usize>(
    measurements: &Series<N, NaiveDateTime, Measurement>,
) -> Result<Events, ()> {
    let mut events = heapless::Vec::<Event, 16>::new();

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

    Ok(Events { events })
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

impl TryFrom<Events> for Series<16, u32, ProtoMeasurement> {
    type Error = ();

    fn try_from(value: Events) -> Result<Self, Self::Error> {
        let mut series: Series<16, u32, ProtoMeasurement> = Series::new(MAX_DEVIATION_MEASUREMENT);

        for m in value.events {
            if let Some(Event_::Event::Measurement(e)) = m.event {
                series.append_monotonic(e.start.timestamp, e.measurement.clone());

                if let Some(end) = e.r#end() {
                    series.append_monotonic(end.timestamp, e.measurement.clone());
                }
            }
        }

        Ok(series)
    }
}

impl TryFrom<Series<16, u32, ProtoMeasurement>> for Events {
    type Error = ProtoMeasurement;

    fn try_from(value: Series<16, u32, ProtoMeasurement>) -> Result<Self, Self::Error> {
        let mut events = heapless::Vec::<Event, 16>::new();
        for se in &value.buckets {
            let mut me = MeasurementEntry::default();

            me.set_start(se.range.start.into());

            if let Some(end) = se.range.end {
                me.set_end(end.into());
            }

            me.set_measurement(se.value.clone());

            events
                .push(Event {
                    event: Some(Event_::Event::Measurement(me)),
                })
                .expect("Should be able to convert");
        }

        Ok(Events { events })
    }
}

pub fn events_to_measurement_entries<const N: usize>(
    events: Events,
) -> Result<HVec<MeasurementSerieEntry, N>, ()> {
    let series: Series<16, u32, ProtoMeasurement> = events.try_into()?;
    let mut out = HVec::new();

    for se in &series.buckets {
        let timestamp = chrono::DateTime::from_timestamp(se.range.start as i64, 0)
            .ok_or(())?
            .naive_utc();
        out.push(MeasurementSerieEntry {
            timestamp,
            measurement: proto_measurement_to_lib(&se.value),
        })
        .map_err(|_| ())?;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    use micropb::{MessageEncode, PbEncoder};

    #[test]
    fn measurements_to_events_roundtrip() {
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

        let events = measurements_to_events(&measurements).unwrap();
        let entries = events_to_measurement_entries::<6>(events).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp, t0);
        assert_eq!(entries[0].measurement.battery, 90);
    }

    #[test]
    fn mac_address_decode_from_ble_read() {
        let data = [10, 6, 64, 245, 32, 183, 133, 64];
        let mac = decode_mac_address(&data).unwrap();
        assert_eq!(mac, [64, 245, 32, 183, 133, 64]);
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

    #[test]
    fn series_events_conversion() {
        let mut series: Series<16, u32, ProtoMeasurement> = Series::new(MAX_DEVIATION_MEASUREMENT);

        series.append_monotonic(
            0,
            ProtoMeasurement {
                r#battery: 100,
                r#lux: 32.0,
                r#temperature: 32.0,
                r#humidity: 32.0,
                r#soil_pf: 32.0,
            },
        );
        series.append_monotonic(
            100,
            ProtoMeasurement {
                r#battery: 100,
                r#lux: 32.0,
                r#temperature: 32.0,
                r#humidity: 32.0,
                r#soil_pf: 32.0,
            },
        );
        series.append_monotonic(
            200,
            ProtoMeasurement {
                r#battery: 100,
                r#lux: 31.0,
                r#temperature: 32.0,
                r#humidity: 32.0,
                r#soil_pf: 32.0,
            },
        );

        let events: Events = series.clone().try_into().expect("Should convert");
        let converted_series: Series<16, u32, ProtoMeasurement> =
            events.try_into().expect("Should convert");

        assert_eq!(converted_series, series);
    }
}
