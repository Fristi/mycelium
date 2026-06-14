#[cfg(target_os = "linux")]
mod linux;

use tokio::sync::mpsc;

use crate::controller::ControllerState;

#[cfg(target_os = "linux")]
pub async fn run_radio(
    state: std::sync::Arc<tokio::sync::Mutex<ControllerState>>,
    event_tx: mpsc::Sender<Vec<u8>>,
    acl_to_host_tx: mpsc::Sender<Vec<u8>>,
    acl_from_host_rx: mpsc::Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    linux::run_radio(state, event_tx, acl_to_host_tx, acl_from_host_rx).await
}

#[cfg(not(target_os = "linux"))]
pub async fn run_radio(
    _state: std::sync::Arc<tokio::sync::Mutex<ControllerState>>,
    _event_tx: mpsc::Sender<Vec<u8>>,
    _acl_to_host_tx: mpsc::Sender<Vec<u8>>,
    _acl_from_host_rx: mpsc::Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    anyhow::bail!("edge-hci-bridge requires Linux (BlueZ + bluer)")
}
