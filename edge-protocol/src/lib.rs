#![cfg_attr(not(test), no_std)]

#[cfg(feature = "sim-gatt")]
extern crate trouble_host_sim as trouble_host;

use timeseries::Deviate;

use crate::v2_proto::Measurement;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_parens
)]
pub mod v2_proto;
pub mod v2;
pub mod wire;

#[cfg(any(feature = "gatt", feature = "sim-gatt"))]
pub mod gatt;

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
