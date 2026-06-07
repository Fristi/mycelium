pub mod central;
pub mod peripheral;
pub mod proto;

pub use central::{OnboardingBleCentral, OnboardingDevice};
pub use peripheral::OnboardingBlePeripheral;
pub use proto::{OnboardingStatus, WifiConfig};
pub use proto::OnboardingStatus_::Phase as OnboardingPhase;

use core::convert::Infallible;
use micropb::{DecodeError, MessageDecode, MessageEncode, PbDecoder, PbEncoder};

pub const HUB_ADVERTISE_NAME: &str = "MyceliumHub";

pub const ONBOARDING_SERVICE_UUID: [u8; 16] =
    [0x02, 0x24, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xAA];
pub const ONBOARDING_WIFI_CHARACTERISTIC_UUID: [u8; 16] =
    [0x02, 0x24, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xAB];
pub const ONBOARDING_STATUS_CHARACTERISTIC_UUID: [u8; 16] =
    [0x02, 0x24, 0x64, 0xF3, 0xB0, 0x00, 0x40, 0x42, 0x50, 0xBA, 0x05, 0xCA, 0x45, 0xBF, 0x8A, 0xAC];

pub const ONBOARDING_SERVICE_UUID_16: u16 = uuid16(ONBOARDING_SERVICE_UUID);
pub const ONBOARDING_WIFI_CHARACTERISTIC_UUID_16: u16 =
    uuid16(ONBOARDING_WIFI_CHARACTERISTIC_UUID);
pub const ONBOARDING_STATUS_CHARACTERISTIC_UUID_16: u16 =
    uuid16(ONBOARDING_STATUS_CHARACTERISTIC_UUID);

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

/// Decode a protobuf message from bytes.
pub fn decode_proto<T>(data: &[u8]) -> Result<T, DecodeError<Infallible>>
where
    T: MessageDecode + Default,
{
    let mut message = T::default();
    let mut decoder = PbDecoder::new(data);
    message.decode(&mut decoder, data.len())?;
    Ok(message)
}

pub fn service_uuid() -> uuid::Uuid {
    uuid::Uuid::from_bytes(ONBOARDING_SERVICE_UUID)
}

pub fn wifi_characteristic_uuid() -> uuid::Uuid {
    uuid::Uuid::from_bytes(ONBOARDING_WIFI_CHARACTERISTIC_UUID)
}

pub fn status_characteristic_uuid() -> uuid::Uuid {
    uuid::Uuid::from_bytes(ONBOARDING_STATUS_CHARACTERISTIC_UUID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wifi_config_round_trip() {
        let msg = WifiConfig {
            ssid: "test-ssid".into(),
            password: "secret".into(),
        };
        let mut buf = [0u8; 256];
        let len = encode_proto(&msg, &mut buf).unwrap();
        let decoded: WifiConfig = decode_proto(&buf[..len]).unwrap();
        assert_eq!(decoded.ssid, "test-ssid");
        assert_eq!(decoded.password, "secret");
    }

    #[test]
    fn onboarding_status_round_trip() {
        let msg = OnboardingStatus {
            phase: OnboardingPhase::AwaitingAuth,
            user_code: "ABCD-EFGH".into(),
            verification_uri_complete: "https://example.com".into(),
            error: String::new(),
        };
        let mut buf = [0u8; 512];
        let len = encode_proto(&msg, &mut buf).unwrap();
        let decoded: OnboardingStatus = decode_proto(&buf[..len]).unwrap();
        assert_eq!(decoded.phase, OnboardingPhase::AwaitingAuth);
        assert_eq!(decoded.user_code, "ABCD-EFGH");
    }
}
