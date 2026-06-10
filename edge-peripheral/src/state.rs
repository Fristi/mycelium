use chrono::NaiveDateTime;
use edge_protocol::translate::encode_device_buffer;
use edge_protocol::v2_proto::Events;
use edge_protocol::{Measurement, WateringSerieEntry};
use esp_hal::ram;
use heapless::Vec as HVec;
use timeseries::Series;

pub const MAX_ENTRIES: usize = 6;

pub type Measurements = Series<MAX_ENTRIES, NaiveDateTime, Measurement>;
pub type Waterings = HVec<WateringSerieEntry, MAX_ENTRIES>;

#[derive(Debug, Clone)]
pub struct DeviceStateData {
    pub measurements: Measurements,
    pub waterings: Waterings,
}

impl DeviceStateData {
    pub fn empty() -> Self {
        Self {
            measurements: Series::new(Measurement::MAX_DEVIATION),
            waterings: HVec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.measurements.is_full() || self.waterings.is_full()
    }

    pub fn to_events(&self) -> Result<Events, ()> {
        encode_device_buffer(&self.measurements, &self.waterings)
    }
}

#[derive(Debug, Clone)]
pub enum DeviceState {
    AwaitingTimeSync,
    Buffering(DeviceStateData),
    Flush(DeviceStateData),
}

// TODO: This is a hack to get the state of the device across the different states.
// It is not thread safe and should be replaced with a more robust solution.
// see: https://stackoverflow.com/questions/79177001/esp-no-std-rust-persist-data-during-deep-sleeps
#[ram(unstable(rtc_fast))]
static mut STATE: DeviceState = DeviceState::AwaitingTimeSync;

pub fn get_device_state() -> &'static DeviceState {
    return unsafe { &STATE };
}

pub fn set_device_state(state: DeviceState) {
    unsafe {
        STATE = state;
    }
}
