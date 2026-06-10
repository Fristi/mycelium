#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub mod i2c;
pub mod noop;

use anyhow::Result;

pub struct OnboardingDisplay {
    pub line1: String,
    pub line2: Option<String>,
}

pub trait Status: Send {
    fn show_onboarding(&mut self, display: &OnboardingDisplay) -> Result<()> {
        let _ = display;
        Ok(())
    }
}

pub fn make_status() -> Result<Box<dyn Status>> {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        let status = i2c::I2cStatus::new("/dev/i2c-3")?;
        return Ok(Box::new(status));
    }

    Ok(Box::new(noop::NoopStatus::new()))
}