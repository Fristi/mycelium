#![cfg_attr(not(test), no_std)]

pub mod translate;
pub mod v2_proto;
pub mod v2;
pub mod wire;

#[cfg(feature = "gatt")]
pub mod gatt;

use chrono::NaiveDateTime;
use timeseries::Deviate;

#[derive(Debug, Clone, Copy)]
pub struct SerieEntry<T> {
    pub timestamp: NaiveDateTime,
    pub value: T,
}

pub type MeasurementSerieEntry = SerieEntry<Measurement>;

#[derive(Debug, Clone, Copy)]
pub struct Watering {
    pub duration_msec: u32,
}

pub type WateringSerieEntry = SerieEntry<Watering>;

#[derive(Clone, Copy, Debug)]
pub struct Measurement {
    pub battery: u8,
    pub lux: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub soil_pf: f32,
}

impl Deviate for Measurement {
    fn deviate(&self, other: &Self, max_deviation: &Self) -> bool {
        (self.temperature - other.temperature).abs() > max_deviation.temperature
            || (self.humidity - other.humidity).abs() > max_deviation.humidity
            || self.battery.abs_diff(other.battery) > max_deviation.battery
            || (self.lux - other.lux).abs() > max_deviation.lux
    }
}

impl Measurement {
    pub const MAX_DEVIATION: Self = Self {
        battery: 1,
        lux: 100.0,
        temperature: 0.1,
        humidity: 0.1,
        soil_pf: 0.1,
    };
}
