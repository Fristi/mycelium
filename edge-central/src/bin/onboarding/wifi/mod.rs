mod system;

use async_trait::async_trait;
use edge_onboarding_ble::WifiConfig;

use crate::cfg::WifiProvisionerMode;

pub use system::SystemWifiProvisioner;

/// Joins the hub to a WiFi network during BLE onboarding.
#[async_trait]
pub trait WifiProvisioner: Send + Sync {
    async fn connect(&self, wifi: &WifiConfig) -> anyhow::Result<()>;
}

/// Accepts credentials but does not change system WiFi (e.g. DietPi already configured).
pub struct NoopWifiProvisioner;

#[async_trait]
impl WifiProvisioner for NoopWifiProvisioner {
    async fn connect(&self, wifi: &WifiConfig) -> anyhow::Result<()> {
        tracing::info!(
            ssid = %wifi.ssid,
            "Skipping WiFi provisioning; hub network is configured externally"
        );
        Ok(())
    }
}

pub fn make_wifi_provisioner(
    mode: WifiProvisionerMode,
    interface: Option<String>,
) -> Box<dyn WifiProvisioner> {
    match mode {
        WifiProvisionerMode::Noop => Box::new(NoopWifiProvisioner),
        WifiProvisionerMode::System => Box::new(SystemWifiProvisioner::new(
            interface.unwrap_or_else(|| "wlan0".into()),
        )),
    }
}
