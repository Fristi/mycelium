use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use edge_onboarding_ble::{
    decode_proto, encode_proto, OnboardingBleCentral, OnboardingDevice, OnboardingStatus,
    WifiConfig, HUB_ADVERTISE_NAME, ONBOARDING_SERVICE_UUID,
};
use futures::{Stream, StreamExt};
use tokio::sync::Mutex;
use uuid::Uuid;

const ONBOARDING_SERVICE: Uuid = Uuid::from_bytes(ONBOARDING_SERVICE_UUID);

pub struct BtleplugOnboardingCentral {
    adapter: Arc<Adapter>,
    peripheral: Arc<Mutex<Option<Peripheral>>>,
}

impl BtleplugOnboardingCentral {
    pub async fn new() -> anyhow::Result<Self> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let adapter = adapters
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No Bluetooth adapter found"))?;
        Ok(Self {
            adapter: Arc::new(adapter),
            peripheral: Arc::new(Mutex::new(None)),
        })
    }

    async fn connected_peripheral(&self) -> anyhow::Result<Peripheral> {
        self.peripheral
            .lock()
            .await
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Not connected to a hub device"))
    }

    async fn find_characteristic(
        peripheral: &Peripheral,
        uuid: Uuid,
    ) -> anyhow::Result<btleplug::api::Characteristic> {
        peripheral.discover_services().await?;
        let services = peripheral.services();
        let service = services
            .iter()
            .find(|s| s.uuid == ONBOARDING_SERVICE)
            .ok_or_else(|| anyhow::anyhow!("Onboarding service not found"))?;
        service
            .characteristics
            .iter()
            .find(|c| c.uuid == uuid)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Characteristic {uuid} not found"))
    }
}

#[async_trait]
impl OnboardingBleCentral for BtleplugOnboardingCentral {
    async fn scan_onboarding_devices(&self) -> anyhow::Result<Vec<OnboardingDevice>> {
        // Scan without a service filter: macOS CoreBluetooth is strict about UUID matching and
        // our onboarding service uses a custom 128-bit UUID (not the Bluetooth base UUID).
        self.adapter.start_scan(ScanFilter::default()).await?;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let peripherals = self.adapter.peripherals().await?;
        let _ = self.adapter.stop_scan().await;

        let mut devices = Vec::new();
        for peripheral in peripherals {
            let properties = peripheral.properties().await?;
            let Some(props) = properties else {
                continue;
            };
            let name_matches = props.local_name.as_deref() == Some(HUB_ADVERTISE_NAME);
            let service_matches = props.services.iter().any(|uuid| *uuid == ONBOARDING_SERVICE);
            if !name_matches && !service_matches {
                continue;
            }
            devices.push(OnboardingDevice {
                id: peripheral.id().to_string(),
                name: props
                    .local_name
                    .clone()
                    .or_else(|| Some(HUB_ADVERTISE_NAME.to_string())),
                rssi: props.rssi,
            });
        }
        Ok(devices)
    }

    async fn connect(&self, device_id: &str) -> anyhow::Result<()> {
        let peripherals = self.adapter.peripherals().await?;
        let peripheral = peripherals
            .into_iter()
            .find(|p| p.id().to_string() == device_id)
            .ok_or_else(|| anyhow::anyhow!("Device {device_id} not found"))?;

        peripheral.connect().await?;
        *self.peripheral.lock().await = Some(peripheral);
        Ok(())
    }

    async fn write_wifi_config(&self, config: &WifiConfig) -> anyhow::Result<()> {
        let peripheral = self.connected_peripheral().await?;
        let wifi_char = Self::find_characteristic(
            &peripheral,
            edge_onboarding_ble::wifi_characteristic_uuid(),
        )
        .await?;

        let mut buf = [0u8; 256];
        let len = encode_proto(config, &mut buf).map_err(|_| anyhow::anyhow!("encode wifi"))?;
        peripheral
            .write(&wifi_char, &buf[..len], WriteType::WithResponse)
            .await?;
        Ok(())
    }

    async fn watch_status(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = OnboardingStatus> + Send>>> {
        let peripheral = self.connected_peripheral().await?;
        let status_char = Self::find_characteristic(
            &peripheral,
            edge_onboarding_ble::status_characteristic_uuid(),
        )
        .await?;

        peripheral.subscribe(&status_char).await?;
        let notifications = peripheral.notifications().await?;
        let status_uuid = status_char.uuid;

        let stream = futures::stream::unfold(notifications, move |mut notifications| async move {
            loop {
                match notifications.next().await {
                    Some(notification) if notification.uuid == status_uuid => {
                        if let Ok(status) = decode_proto::<OnboardingStatus>(&notification.value) {
                            return Some((status, notifications));
                        }
                    }
                    Some(_) => continue,
                    None => return None,
                }
            }
        });

        Ok(Box::pin(stream))
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        if let Some(peripheral) = self.peripheral.lock().await.take() {
            peripheral.disconnect().await?;
        }
        Ok(())
    }
}
