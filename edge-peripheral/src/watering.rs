use embassy_time::Duration;
use log::info;

#[cfg(feature = "watering-gpio")]
use embassy_time::Timer;
#[cfg(feature = "watering-gpio")]
use esp_hal::gpio::{Level, Output};

pub trait Watering {
    async fn water_for(&mut self, duration: Duration) -> anyhow::Result<()>;
}

pub struct NoopWatering;

impl Watering for NoopWatering {
    async fn water_for(&mut self, duration: Duration) -> anyhow::Result<()> {
        info!(
            "NoopWatering: would water for {} ms",
            duration.as_millis()
        );
        Ok(())
    }
}

#[cfg(feature = "watering-gpio")]
pub struct GpioWatering<'a> {
    motor: Output<'a>,
}

#[cfg(feature = "watering-gpio")]
impl<'a> GpioWatering<'a> {
    pub fn new(motor: Output<'a>) -> Self {
        Self { motor }
    }
}

#[cfg(feature = "watering-gpio")]
impl Watering for GpioWatering<'_> {
    async fn water_for(&mut self, duration: Duration) -> anyhow::Result<()> {
        info!("GpioWatering: watering for {} ms", duration.as_millis());
        self.motor.set_high();
        Timer::after(duration).await;
        self.motor.set_low();
        Ok(())
    }
}

#[cfg(feature = "watering-gpio")]
pub fn new_gpio_watering<'a>(
    pin: esp_hal::peripherals::GPIO19<'a>,
    config: esp_hal::gpio::OutputConfig,
) -> GpioWatering<'a> {
    let motor = Output::new(pin, Level::Low, config);
    GpioWatering::new(motor)
}
