#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub mod i2c;
pub mod display_loop;
pub mod format;
pub mod noop;

use anyhow::Result;

use crate::cfg::StatusStrategy;

pub struct OnboardingDisplay {
    pub line1: String,
    pub line2: Option<String>,
}

pub struct SyncSummaryDisplay {
    pub lines: [String; 3],
    pub page: Option<(usize, usize)>,
}

pub trait Status: Send {
    fn show_onboarding(&mut self, display: &OnboardingDisplay) -> Result<()> {
        let _ = display;
        Ok(())
    }

    fn show_sync_summary(&mut self, display: &SyncSummaryDisplay) -> Result<()> {
        let _ = display;
        Ok(())
    }
}

pub fn make_status(strategy: StatusStrategy) -> Result<Box<dyn Status>> {
    match strategy {
        StatusStrategy::Noop => Ok(Box::new(noop::NoopStatus::new())),
        StatusStrategy::I2c => make_i2c_status(),
    }
}

fn make_i2c_status() -> Result<Box<dyn Status>> {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        let status = i2c::I2cStatus::new("/dev/i2c-3")?;
        Ok(Box::new(status))
    }

    #[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
    {
        anyhow::bail!("APP.STATUS_STRATEGY=i2c requires Linux aarch64")
    }
}
