use chrono::NaiveDateTime;
use edge_protocol::v2_proto::{
    Event, Event_, Events, Measurement, MeasurementRange, PlantProfile, Timestamp, WateringEntry,
};
use esp_hal::ram;
use heapless::Vec as HVec;
use timeseries::Series;

pub const MAX_ENTRIES_MEASUREMENTS: usize = 8;
pub const MAX_ENTRIES_WATERINGS: usize = 2;

pub type Measurements = Series<MAX_ENTRIES_MEASUREMENTS, NaiveDateTime, Measurement>;
pub type Waterings = HVec<WateringEntry, MAX_ENTRIES_WATERINGS>;

#[derive(Debug, Clone)]
pub struct DeviceStateData {
    pub measurements: Measurements,
    pub waterings: Waterings,
    pub plant_profile: Option<PlantProfile>
}

impl DeviceStateData {
    pub fn empty() -> Self {
        Self {
            measurements: Series::new(Measurement::MAX_DEVIATION),
            waterings: HVec::new(),
            plant_profile: None
        }
    }

    pub fn is_full(&self) -> bool {
        self.measurements.is_full() || self.waterings.is_full()
    }

    pub fn to_events(&self) -> Result<Events, ()> {
        let mut events = Events::default();

        for bucket in &self.measurements.buckets {
            let mut range = MeasurementRange::default();
            range.set_start(naive_to_timestamp(bucket.range.start)?);
            if let Some(end) = bucket.range.end {
                range.set_end(naive_to_timestamp(end)?);
            }
            range.set_measurement(bucket.value.clone());

            events
                .r#events
                .push(Event {
                    r#event: Some(Event_::Event::Measurement(range)),
                })
                .map_err(|_| ())?;
        }

        for watering in &self.waterings {
            events
                .r#events
                .push(Event {
                    r#event: Some(Event_::Event::Watering(watering.clone())),
                })
                .map_err(|_| ())?;
        }

        Ok(events)
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
    unsafe { &*core::ptr::addr_of!(STATE) }
}

pub fn set_device_state(state: DeviceState) {
    unsafe {
        STATE = state;
    }
}

fn naive_to_timestamp(dt: NaiveDateTime) -> Result<Timestamp, ()> {
    let ts = dt.and_utc().timestamp();
    if ts < 0 {
        return Err(());
    }
    Ok(Timestamp {
        timestamp: ts as u32,
    })
}

