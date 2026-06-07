use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

use crate::proto::{OnboardingStatus, WifiConfig};

/// A hub discovered during onboarding scan.
#[derive(Debug, Clone)]
pub struct OnboardingDevice {
    pub id: String,
    pub name: Option<String>,
    pub rssi: Option<i16>,
}

/// BLE central (Tauri app) role during onboarding.
#[async_trait]
pub trait OnboardingBleCentral: Send {
    /// Scan for peripherals advertising the onboarding service.
    async fn scan_onboarding_devices(&self) -> anyhow::Result<Vec<OnboardingDevice>>;

    /// Connect to a device by ID returned from [`Self::scan_onboarding_devices`].
    async fn connect(&self, device_id: &str) -> anyhow::Result<()>;

    /// Write WiFi credentials to the hub.
    async fn write_wifi_config(&self, config: &WifiConfig) -> anyhow::Result<()>;

    /// Subscribe to onboarding status notifications from the hub.
    async fn watch_status(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = OnboardingStatus> + Send>>>;

    async fn disconnect(&self) -> anyhow::Result<()>;
}
