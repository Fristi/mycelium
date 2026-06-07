use crate::cfg::{AppConfig, OnboardingStrategy};
use crate::onboarding::local::LocalOnboarding;
use crate::onboarding::types::Onboarding;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
use crate::status::make_status;

pub mod ble;
pub mod local;
pub mod types;
pub mod wifi;

pub async fn make_onboarding(cfg: &AppConfig) -> anyhow::Result<Box<dyn Onboarding>> {
    match cfg.onboarding_strategy {
        OnboardingStrategy::Ble => make_ble_onboarding(cfg).await,
        OnboardingStrategy::Local => {
            let wifi = cfg
                .wifi
                .clone()
                .ok_or_else(|| anyhow::anyhow!("APP.WIFI is required for local onboarding"))?;
            let onboarding = LocalOnboarding::new(cfg.auth0.clone(), wifi);
            Ok(Box::new(onboarding))
        }
    }
}

async fn make_ble_onboarding(cfg: &AppConfig) -> anyhow::Result<Box<dyn Onboarding>> {
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        let peripheral = ble::BluerOnboardingPeripheral::new().await?;
        let status = make_status()?;
        let wifi = wifi::make_wifi_provisioner(
            cfg.wifi_provisioner.clone(),
            cfg.wifi_interface.clone(),
        );
        return Ok(Box::new(ble::BleOnboarding::new(
            cfg.auth0.clone(),
            wifi,
            peripheral,
            status,
        )));
    }

    #[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
    {
        let _ = cfg;
        anyhow::bail!("BLE onboarding is only supported on Linux aarch64")
    }
}
