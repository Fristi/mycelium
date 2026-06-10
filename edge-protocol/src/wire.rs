//! BLE wire codec: protobuf encode/decode and GATT UUID constants.

use core::convert::Infallible;
use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

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

