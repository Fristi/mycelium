use async_trait::async_trait;

use crate::proto::{OnboardingStatus, WifiConfig};

/// BLE peripheral (hub) role during onboarding.
#[async_trait]
pub trait OnboardingBlePeripheral: Send {
    /// Start advertising `MyceliumHub` and the onboarding GATT service.
    async fn advertise_and_accept(&mut self) -> anyhow::Result<()>;

    /// Wait for the central to write `WifiConfig` on the wifi characteristic.
    async fn receive_wifi_config(&mut self) -> anyhow::Result<WifiConfig>;

    /// Push status updates to the connected central (notify + readable value).
    async fn notify_status(&mut self, status: &OnboardingStatus) -> anyhow::Result<()>;

    /// Stop advertising and release the GATT application.
    async fn shutdown(&mut self) -> anyhow::Result<()>;
}
