use core::cell::RefCell;

use bh1730fvc::{blocking::BH1730FVC};
use embassy_time::{Delay, Timer};
use embedded_hal_bus::i2c::RefCellDevice;
use embedded_hal_compat::ReverseCompat;
use esp_hal::{analog::adc::AdcChannel, gpio::Output, i2c::master::I2c, Blocking};
use edge_protocol::v2_proto::Measurement;

use crate::battery::BatteryMeasurement;
use crate::soil_calibration::pf_to_percent;
use crate::utils::anyhow::ResultAny;
use crate::moisture::SoilSensor;

pub struct Gauge<'a, P : AdcChannel> {
    i2c_pcb: RefCell<I2c<'a, Blocking>>,
    i2c_ext: RefCell<I2c<'a, Blocking>>,
    pcb_pwr: Output<'a>,
    bm: BatteryMeasurement<'a, P>
}

impl <'a, P : AdcChannel> Gauge<'a, P> {
    pub fn new(
        i2c_pcb: RefCell<I2c<'a, Blocking>>, 
        i2c_ext: RefCell<I2c<'a, Blocking>>, 
        pcb_pwr: Output<'a>, 
        bm: BatteryMeasurement<'a, P>) -> Self {
        Self {
            i2c_pcb,
            i2c_ext,
            pcb_pwr,
            bm
        }
    }

    pub async fn sample(&mut self) -> anyhow::Result<Measurement> {
        self.pcb_pwr.set_high();

        Timer::after_millis(100).await;

        let mut delay = Delay;

        let mut i2c_pcb_bh1730fvc = RefCellDevice::new(&self.i2c_pcb).reverse();
        let mut i2c_ext_moisture = RefCellDevice::new(&self.i2c_ext);

        let mut soil = SoilSensor::new(&mut i2c_ext_moisture);

        let _ = soil.start().with_anyhow("Failed to start reading soil");

        Timer::after_millis(15).await;

        let soil_pf_raw = soil.read().await.with_anyhow("Unable to read soil")?;
        let soil_moisture = pf_to_percent(soil_pf_raw) as f32;

        let mut bh1730fvc = BH1730FVC::new(&mut delay, &mut i2c_pcb_bh1730fvc)
            .with_anyhow("BH1730FVC init failed")?;

        let mut sht = shtcx2::shtc3(RefCellDevice::new(&self.i2c_pcb));

        sht.start_measurement(shtcx2::PowerMode::NormalMode)
            .with_anyhow("SHT start measurement failed")?;

        bh1730fvc.set_mode(bh1730fvc::Mode::SingleShot, &mut i2c_pcb_bh1730fvc)
            .with_anyhow("BH1730FVC set mode failed")?;

        Timer::after_millis(150).await;

        let lux = bh1730fvc.read_ambient_light_intensity(&mut i2c_pcb_bh1730fvc).with_anyhow("BH1730FVC read failed")?;

        let battery = self.bm.sample();
        
        let measurement = sht.get_measurement_result().with_anyhow("SHT read failed")?;

        self.pcb_pwr.set_low();

        let measurement = Measurement {
            battery: battery as u32,
            lux,
            temperature: measurement.temperature.as_degrees_celsius(),
            humidity: measurement.humidity.as_percent(),
            soil_moisture,
        };

        Ok(measurement)
    }
}

