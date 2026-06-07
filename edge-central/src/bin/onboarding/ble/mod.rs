#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod bluer;

use async_trait::async_trait;
use chrono::Utc;
use edge_onboarding_ble::{OnboardingBlePeripheral, OnboardingPhase, OnboardingStatus};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::{
    auth::auth0::{TokenResult, TokenStatus},
    cfg::Auth0Config,
    data::types::EdgeState,
    onboarding::types::Onboarding,
    onboarding::wifi::WifiProvisioner,
    status::{OnboardingDisplay, Status},
};

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use bluer::BluerOnboardingPeripheral;

pub struct BleOnboarding<P> {
    auth0: Auth0Config,
    wifi: Box<dyn WifiProvisioner>,
    peripheral: Mutex<P>,
    status: Mutex<Box<dyn Status>>,
}

impl<P> BleOnboarding<P>
where
    P: OnboardingBlePeripheral,
{
    pub fn new(
        auth0: Auth0Config,
        wifi: Box<dyn WifiProvisioner>,
        peripheral: P,
        status: Box<dyn Status>,
    ) -> Self {
        Self {
            auth0,
            wifi,
            peripheral: Mutex::new(peripheral),
            status: Mutex::new(status),
        }
    }

    async fn show_onboarding(&self, display: OnboardingDisplay) -> anyhow::Result<()> {
        self.status
            .lock()
            .await
            .show_onboarding(&display)
    }
}

#[async_trait]
impl<P> Onboarding for BleOnboarding<P>
where
    P: OnboardingBlePeripheral,
{
    async fn process(&self) -> anyhow::Result<EdgeState> {
        let mut peripheral = self.peripheral.lock().await;

        peripheral.advertise_and_accept().await?;
        self.show_onboarding(OnboardingDisplay {
            line1: "Waiting WiFi".into(),
            line2: None,
        })
        .await?;

        let wifi = loop {
            let wifi = peripheral.receive_wifi_config().await?;

            peripheral
                .notify_status(&OnboardingStatus {
                    phase: OnboardingPhase::ProvisioningWifi,
                    user_code: String::new(),
                    verification_uri_complete: String::new(),
                    error: String::new(),
                })
                .await?;

            self.show_onboarding(OnboardingDisplay {
                line1: "Connecting".into(),
                line2: Some("WiFi...".into()),
            })
            .await?;

            match self.wifi.connect(&wifi).await {
                Ok(()) => break wifi,
                Err(error) => {
                    tracing::error!(?error, ssid = %wifi.ssid, "WiFi connect failed");
                    peripheral
                        .notify_status(&OnboardingStatus {
                            phase: OnboardingPhase::Failed,
                            user_code: String::new(),
                            verification_uri_complete: String::new(),
                            error: error.to_string(),
                        })
                        .await?;
                    peripheral.advertise_and_accept().await?;
                    self.show_onboarding(OnboardingDisplay {
                        line1: "Waiting WiFi".into(),
                        line2: None,
                    })
                    .await?;
                }
            }
        };

        let device_code = crate::auth::auth0::request_device_code(&self.auth0).await?;

        info!("Verification code: {}", device_code.user_code);
        info!(
            "Verification url: {}",
            device_code.verification_uri_complete
        );

        self.show_onboarding(OnboardingDisplay {
            line1: "Authorize".into(),
            line2: Some(device_code.user_code.clone()),
        })
        .await?;

        peripheral
            .notify_status(&OnboardingStatus {
                phase: OnboardingPhase::AwaitingAuth,
                user_code: device_code.user_code.clone(),
                verification_uri_complete: device_code.verification_uri_complete.clone(),
                error: String::new(),
            })
            .await?;

        loop {
            match crate::auth::auth0::poll_token(&self.auth0, device_code.device_code.as_str())
                .await
            {
                Ok(TokenResult::Full {
                    access_token,
                    refresh_token,
                    expires_in,
                }) => {
                    let expires_at = Utc::now() + Duration::from_secs(expires_in);

                    peripheral
                        .notify_status(&OnboardingStatus {
                            phase: OnboardingPhase::Complete,
                            user_code: String::new(),
                            verification_uri_complete: String::new(),
                            error: String::new(),
                        })
                        .await?;

                    self.show_onboarding(OnboardingDisplay {
                        line1: "Device".into(),
                        line2: Some("onboarded".into()),
                    })
                    .await?;

                    peripheral.shutdown().await?;

                    return Ok(EdgeState {
                        wifi_ssid: wifi.ssid,
                        wifi_password: wifi.password,
                        auth0_access_token: access_token,
                        auth0_refresh_token: refresh_token,
                        auth0_expires_at: expires_at.naive_utc(),
                    });
                }
                Ok(TokenResult::AccessToken { .. }) => {
                    info!("Received access token without refresh token, skipping");
                }
                Ok(TokenResult::Error { error }) => match error {
                    TokenStatus::ExpiredToken
                    | TokenStatus::AccessDenied
                    | TokenStatus::InvalidGrant => {
                        peripheral
                            .notify_status(&OnboardingStatus {
                                phase: OnboardingPhase::Failed,
                                user_code: String::new(),
                                verification_uri_complete: String::new(),
                                error: format!("{:?}", error),
                            })
                            .await?;
                        anyhow::bail!("Failed with {:?}", error);
                    }
                    TokenStatus::AuthorizationPending | TokenStatus::SlowDown => {
                        info!("Auth0 status: {:?}", error);
                    }
                },
                Err(error) => {
                    anyhow::bail!("Failed with {}", error);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}
