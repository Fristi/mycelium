use edge_protocol::Measurement;
use timeseries::Series;
use chrono::NaiveDateTime;

use esp_hal::ram;

pub type Measurements = Series<6, NaiveDateTime, Measurement>;

#[derive(Debug)]
pub enum DeviceState {
    AwaitingTimeSync,
    Buffering(Measurements),
    Flush(Measurements)
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