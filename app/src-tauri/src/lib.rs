mod onboarding_ble;

use std::sync::Arc;

use edge_onboarding_ble::{OnboardingBleCentral, OnboardingPhase, OnboardingStatus, WifiConfig};
use futures::StreamExt;
use onboarding_ble::BtleplugOnboardingCentral;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppBleState {
    central: Arc<Mutex<BtleplugOnboardingCentral>>,
}

impl AppBleState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            central: Arc::new(Mutex::new(BtleplugOnboardingCentral::new().await?)),
        })
    }
}

#[derive(Serialize)]
pub struct OnboardingDeviceDto {
    pub id: String,
    pub name: Option<String>,
    pub rssi: Option<i16>,
}

#[derive(Serialize, Clone)]
pub struct OnboardingStatusDto {
    pub phase: String,
    pub user_code: String,
    pub verification_uri_complete: String,
    pub error: String,
}

impl From<&OnboardingStatus> for OnboardingStatusDto {
    fn from(status: &OnboardingStatus) -> Self {
        let phase = match status.phase {
            OnboardingPhase::AwaitingWifi => "AwaitingSettings",
            OnboardingPhase::ProvisioningWifi => "ProvisioningWifi",
            OnboardingPhase::AwaitingAuth => "AwaitingAuthorization",
            OnboardingPhase::Complete => "Complete",
            OnboardingPhase::Failed => "Failed",
            _ => "AwaitingSettings",
        };
        Self {
            phase: phase.to_string(),
            user_code: status.user_code.clone(),
            verification_uri_complete: status.verification_uri_complete.clone(),
            error: status.error.clone(),
        }
    }
}

#[tauri::command]
async fn scan_onboarding_devices(
    state: State<'_, AppBleState>,
) -> Result<Vec<OnboardingDeviceDto>, String> {
    let central = state.central.lock().await;
    let devices = central
        .scan_onboarding_devices()
        .await
        .map_err(|e| e.to_string())?;
    Ok(devices
        .into_iter()
        .map(|d| OnboardingDeviceDto {
            id: d.id,
            name: d.name,
            rssi: d.rssi,
        })
        .collect())
}

#[tauri::command]
async fn provision_hub_device(
    state: State<'_, AppBleState>,
    app: AppHandle,
    device_id: String,
    ssid: String,
    password: String,
) -> Result<(), String> {
    let central = state.central.clone();
    let device_id = device_id.clone();

    tokio::spawn(async move {
        let result = async {
            let central = central.lock().await;
            central.connect(&device_id).await?;
            central
                .write_wifi_config(&WifiConfig { ssid, password })
                .await?;

            let mut status_stream = central.watch_status().await?;
            while let Some(status) = status_stream.next().await {
                let dto = OnboardingStatusDto::from(&status);
                app.emit("onboarding-status", dto)?;
                if matches!(
                    status.phase,
                    OnboardingPhase::Complete | OnboardingPhase::Failed
                ) {
                    break;
                }
            }
            central.disconnect().await?;
            anyhow::Ok(())
        }
        .await;

        if let Err(error) = result {
            let _ = app.emit(
                "onboarding-status",
                OnboardingStatusDto {
                    phase: "Failed".into(),
                    user_code: String::new(),
                    verification_uri_complete: String::new(),
                    error: error.to_string(),
                },
            );
        }
    });

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            match tauri::async_runtime::block_on(AppBleState::new()) {
                Ok(state) => {
                    app.manage(state);
                }
                Err(error) => {
                    eprintln!("Failed to initialize BLE central: {error}");
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_onboarding_devices,
            provision_hub_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
