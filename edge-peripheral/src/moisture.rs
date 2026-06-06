use embassy_time::Timer;
use embedded_hal::{i2c::{I2c, SevenBitAddress}};
use crate::utils::anyhow::ResultAny;

const ADDR: u8 = 0x55;

/// Soil sensor state
pub struct SoilSensor<I2C> {
    i2c: I2C
}

impl<I2C> SoilSensor<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()>
    where
        I2C: I2c<SevenBitAddress>
    {
        self.i2c
            .write(ADDR, &[0x10 | 0x01, 0x01])
            .with_anyhow("Unable to start soil conversion")?;

        Ok(())
    }

    pub async fn read(&mut self) -> anyhow::Result<f32>
    where
        I2C: I2c<SevenBitAddress>
    {
        self.i2c
            .write(ADDR, &[0x10 | 0x02])
            .with_anyhow("Unable to trigger soil read")?;

        Timer::after_micros(150).await;

        let mut buf = [0u8; 3];
        self.i2c
            .read(ADDR, &mut buf)
            .with_anyhow("Unable to read soil data")?;

        let d0 = buf[0];
        let d1 = buf[1];
        let d2 = buf[2];

        let pf = d0 as f32 + (d1 as f32 * 256.0) + (d2 as f32 / 256.0);
        
        Ok(pf)
    }
}
