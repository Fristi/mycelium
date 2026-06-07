use std::sync::Arc;

use async_trait::async_trait;
use bluer::{
    adv::Advertisement,
    gatt::local::{
        Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead,
        CharacteristicWrite, CharacteristicWriteMethod, ReqError, Service,
    },
};
use edge_onboarding_ble::{
    decode_proto, encode_proto, service_uuid, status_characteristic_uuid, wifi_characteristic_uuid,
    HUB_ADVERTISE_NAME, OnboardingBlePeripheral, OnboardingPhase, OnboardingStatus, WifiConfig,
};
use futures::FutureExt;
use tokio::sync::{mpsc, Mutex};

pub struct BluerOnboardingPeripheral {
    wifi_rx: mpsc::Receiver<WifiConfig>,
    status_value: Arc<Mutex<Vec<u8>>>,
    status_notify_tx: Arc<Mutex<Option<mpsc::Sender<Vec<u8>>>>>,
    _adv_handle: bluer::adv::AdvertisementHandle,
    _app_handle: bluer::gatt::local::ApplicationHandle,
}

impl BluerOnboardingPeripheral {
    pub async fn new() -> anyhow::Result<Self> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        let (wifi_tx, wifi_rx) = mpsc::channel(1);
        let status_value = Arc::new(Mutex::new(Vec::new()));
        let status_notify_tx: Arc<Mutex<Option<mpsc::Sender<Vec<u8>>>>> =
            Arc::new(Mutex::new(None));

        let status_value_read = status_value.clone();
        let status_value_notify = status_value.clone();
        let status_notify_tx_notify = status_notify_tx.clone();
        let wifi_tx_write = wifi_tx.clone();

        let app = Application {
            services: vec![Service {
                uuid: service_uuid(),
                primary: true,
                characteristics: vec![
                    Characteristic {
                        uuid: wifi_characteristic_uuid(),
                        write: Some(CharacteristicWrite {
                            write: true,
                            write_without_response: false,
                            method: CharacteristicWriteMethod::Fun(Box::new(
                                move |new_value, _req| {
                                    let wifi_tx = wifi_tx_write.clone();
                                    async move {
                                        let config: WifiConfig =
                                            decode_proto(&new_value).map_err(|_| ReqError::Failed)?;
                                        wifi_tx
                                            .send(config)
                                            .await
                                            .map_err(|_| ReqError::Failed)?;
                                        Ok(())
                                    }
                                    .boxed()
                                },
                            )),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Characteristic {
                        uuid: status_characteristic_uuid(),
                        read: Some(CharacteristicRead {
                            read: true,
                            fun: Box::new({
                                let status_value = status_value_read.clone();
                                move |_req| {
                                    let status_value = status_value.clone();
                                    async move {
                                        Ok(status_value.lock().await.clone())
                                    }
                                    .boxed()
                                }
                            }),
                            ..Default::default()
                        }),
                        notify: Some(CharacteristicNotify {
                            notify: true,
                            method: CharacteristicNotifyMethod::Fun(Box::new(move |mut notifier| {
                                let status_value = status_value_notify.clone();
                                let status_notify_tx = status_notify_tx_notify.clone();
                                async move {
                                    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(8);
                                    {
                                        let mut guard = status_notify_tx.lock().await;
                                        *guard = Some(tx);
                                    }
                                    tokio::spawn(async move {
                                        let initial = status_value.lock().await.clone();
                                        if !initial.is_empty() {
                                            let _ = notifier.notify(initial).await;
                                        }
                                        while let Some(value) = rx.recv().await {
                                            if notifier.notify(value).await.is_err() {
                                                break;
                                            }
                                        }
                                    });
                                }
                                .boxed()
                            })),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        let app_handle = adapter.serve_gatt_application(app).await?;

        let le_advertisement = Advertisement {
            service_uuids: vec![service_uuid()].into_iter().collect(),
            discoverable: Some(true),
            local_name: Some(HUB_ADVERTISE_NAME.to_string()),
            ..Default::default()
        };
        let adv_handle = adapter.advertise(le_advertisement).await?;

        Ok(Self {
            wifi_rx,
            status_value,
            status_notify_tx,
            _adv_handle: adv_handle,
            _app_handle: app_handle,
        })
    }

    async fn set_status_bytes(&self, bytes: Vec<u8>) -> anyhow::Result<()> {
        {
            let mut value = self.status_value.lock().await;
            *value = bytes.clone();
        }
        if let Some(tx) = self.status_notify_tx.lock().await.as_ref() {
            tx.send(bytes).await.ok();
        }
        Ok(())
    }
}

#[async_trait]
impl OnboardingBlePeripheral for BluerOnboardingPeripheral {
    async fn advertise_and_accept(&mut self) -> anyhow::Result<()> {
        let status = OnboardingStatus {
            phase: OnboardingPhase::AwaitingWifi,
            user_code: String::new(),
            verification_uri_complete: String::new(),
            error: String::new(),
        };
        let mut buf = [0u8; 512];
        let len = encode_proto(&status, &mut buf).map_err(|_| anyhow::anyhow!("encode status"))?;
        self.set_status_bytes(buf[..len].to_vec()).await
    }

    async fn receive_wifi_config(&mut self) -> anyhow::Result<WifiConfig> {
        self.wifi_rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("BLE connection closed before WiFi config was received"))
    }

    async fn notify_status(&mut self, status: &OnboardingStatus) -> anyhow::Result<()> {
        let mut buf = [0u8; 512];
        let len = encode_proto(status, &mut buf).map_err(|_| anyhow::anyhow!("encode status"))?;
        self.set_status_bytes(buf[..len].to_vec()).await
    }

    async fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
