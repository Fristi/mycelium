use std::process::Command;
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use edge_onboarding_ble::WifiConfig;
use tracing::info;

use super::WifiProvisioner;

pub struct SystemWifiProvisioner {
    interface: String,
}

impl SystemWifiProvisioner {
    pub fn new(interface: String) -> Self {
        Self { interface }
    }

    fn connect_blocking(&self, ssid: &str, password: &str) -> anyhow::Result<()> {
        if nmcli_available() {
            info!(interface = %self.interface, ssid, "Connecting to WiFi via nmcli");
            match connect_nmcli(&self.interface, ssid, password) {
                Ok(true) => return Ok(()),
                Ok(false) => {
                    tracing::warn!(ssid, "nmcli could not activate network, trying wpa_cli")
                }
                Err(error) => tracing::warn!(?error, "nmcli failed, trying wpa_cli"),
            }
        }

        info!(interface = %self.interface, ssid, "Connecting to WiFi via wpa_cli");
        connect_wpa_cli(&self.interface, ssid, password)
    }
}

#[async_trait]
impl WifiProvisioner for SystemWifiProvisioner {
    async fn connect(&self, wifi: &WifiConfig) -> anyhow::Result<()> {
        let interface = self.interface.clone();
        let ssid = wifi.ssid.clone();
        let password = wifi.password.clone();
        tokio::task::spawn_blocking(move || {
            SystemWifiProvisioner { interface }.connect_blocking(&ssid, &password)
        })
        .await?
    }
}

fn nmcli_available() -> bool {
    Command::new("nmcli")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn connect_nmcli(interface: &str, ssid: &str, password: &str) -> anyhow::Result<bool> {
    let output = Command::new("nmcli")
        .args([
            "device",
            "wifi",
            "connect",
            ssid,
            "password",
            password,
            "ifname",
            interface,
        ])
        .output()
        .map_err(|e| anyhow::anyhow!("nmcli failed to run: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("successfully activated"))
}

fn connect_wpa_cli(interface: &str, ssid: &str, password: &str) -> anyhow::Result<()> {
    let _ = Command::new("ip")
        .args(["link", "set", interface, "up"])
        .status();

    let network_id = run_wpa_cli(interface, &["add_network"])?.trim().to_string();
    if network_id == "FAIL" {
        anyhow::bail!("wpa_cli add_network failed");
    }

    let ssid_arg = format!("\"{ssid}\"");
    let psk_arg = format!("\"{password}\"");

    run_wpa_cli(interface, &["set_network", &network_id, "ssid", &ssid_arg])?;
    run_wpa_cli(interface, &["set_network", &network_id, "psk", &psk_arg])?;
    run_wpa_cli(
        interface,
        &["set_network", &network_id, "key_mgmt", "WPA-PSK"],
    )?;
    run_wpa_cli(interface, &["enable_network", &network_id])?;
    run_wpa_cli(interface, &["select_network", &network_id])?;

    for _ in 0..45 {
        let status = run_wpa_cli(interface, &["status"])?;
        if status.contains("wpa_state=COMPLETED") {
            return Ok(());
        }
        thread::sleep(Duration::from_secs(1));
    }

    anyhow::bail!("WiFi connection to {ssid} timed out")
}

fn run_wpa_cli(interface: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("wpa_cli")
        .arg("-i")
        .arg(interface)
        .args(args)
        .output()
        .map_err(|e| {
            anyhow::anyhow!(
                "wpa_cli not found ({e}). Install wpa_supplicant on the hub, or use NetworkManager."
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("wpa_cli {:?} failed: {stdout}{stderr}", args);
    }

    Ok(stdout)
}
