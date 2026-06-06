//! GATT encoding for v2 protobuf types (trouble-host integration).

use core::cell::UnsafeCell;

use crate::v2_proto::*;
use micropb::{MessageDecode, MessageEncode, PbDecoder, PbEncoder};
use trouble_host::types::gatt_traits::{AsGatt, FromGatt, FromGattError};

const MAX_BUFFER_SIZE: usize = 1024;

struct SyncUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T> Sync for SyncUnsafeCell<T> {}

static GATT_BUFFER: SyncUnsafeCell<[u8; MAX_BUFFER_SIZE]> =
    SyncUnsafeCell(UnsafeCell::new([0; MAX_BUFFER_SIZE]));

macro_rules! as_gatt {
    ($($t:ty),*) => {
        $(
            impl AsGatt for $t
            where
                $t: MessageEncode
            {
                const MIN_SIZE: usize = 0;
                const MAX_SIZE: usize = match <$t as MessageEncode>::MAX_SIZE {
                    Some(n) => n,
                    None => MAX_BUFFER_SIZE
                };

                fn as_gatt(&self) -> &'static [u8] {
                    let buffer: &mut [u8; MAX_BUFFER_SIZE] = unsafe { &mut *GATT_BUFFER.0.get() };
                    let len = MessageEncode::compute_size(self);
                    if len == 0 {
                        return &[];
                    }
                    let ptr = buffer.as_ptr();
                    let mut writer = &mut buffer[..len];
                    let mut encoder = PbEncoder::new(&mut writer);
                    self.encode(&mut encoder).expect("Encoding failed");
                    unsafe { core::slice::from_raw_parts(ptr, len) }
                }
            }
        )*
    };
}

macro_rules! from_gatt {
    ($($t:ty),*) => {
        $(
            impl FromGatt for $t
            where
                $t: MessageDecode + Default,
            {
                fn from_gatt(data: &[u8]) -> Result<Self, FromGattError> {
                    if data.is_empty() {
                        return Ok(Self::default());
                    }
                    let mut message = Self::default();
                    let mut decoder = PbDecoder::new(data);
                    message
                        .decode(&mut decoder, data.len())
                        .map_err(|_| FromGattError::InvalidLength)?;
                    Ok(message)
                }
            }
        )*
    };
}

static GATT_BUFFER_SYNCSTATE: SyncUnsafeCell<[u8; 1]> =
    SyncUnsafeCell(UnsafeCell::new([0; 1]));

impl AsGatt for SyncState {
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: usize = 1;

    fn as_gatt(&self) -> &[u8] {
        let buffer: &mut [u8; 1] = unsafe { &mut *GATT_BUFFER_SYNCSTATE.0.get() };
        let ptr = buffer.as_ptr();
        buffer[0] = self.0 as u8;
        unsafe { core::slice::from_raw_parts(ptr, SyncState::MAX_SIZE) }
    }
}

impl FromGatt for SyncState {
    fn from_gatt(data: &[u8]) -> Result<Self, FromGattError> {
        Ok(SyncState::from(data[0] as i8))
    }
}

as_gatt!(PlantProfile, Timestamp, Events, MacAddress);
from_gatt!(PlantProfile, Timestamp, Events, MacAddress);
