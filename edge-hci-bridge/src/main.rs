use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use tokio::sync::mpsc;
use tokio_serial::SerialPortBuilderExt;
use tracing::{error, info};

mod acl;
mod controller;
mod hci;
mod radio;

use controller::ControllerState;
use hci::{command_opcode, HostPacket, H4Reader, H4Writer};

const OPCODE_LE_SET_ADV_ENABLE: u16 = 0x200A;

#[derive(Parser, Debug)]
#[command(name = "edge-hci-bridge", about = "Linux HCI controller emulator for Wokwi sim E2E")]
struct Args {
    /// Serial device path (PTY from socat or RFC2217 tty)
    #[arg(long, default_value = "/tmp/mycelium-hci")]
    port: String,

    /// HCI UART baud rate (must match edge-peripheral sim firmware)
    #[arg(long, default_value_t = 115_200)]
    baud: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    info!(port = %args.port, baud = args.baud, "Starting edge-hci-bridge");

    let port = tokio_serial::new(&args.port, args.baud)
        .open_native_async()
        .with_context(|| format!("open serial port {}", args.port))?;

    let (reader, writer) = tokio::io::split(port);
    let state = Arc::new(tokio::sync::Mutex::new(ControllerState::new()));

    let (event_tx, mut event_rx) = mpsc::channel::<Vec<u8>>(32);
    let (acl_to_host_tx, mut acl_to_host_rx) = mpsc::channel::<Vec<u8>>(32);
    let (acl_from_host_tx, acl_from_host_rx) = mpsc::channel::<Vec<u8>>(32);

    let read_state = state.clone();
    let read_event_tx = event_tx.clone();
    let read_acl_from_host_tx = acl_from_host_tx.clone();
    tokio::spawn(async move {
        let mut h4 = H4Reader::new(reader);
        loop {
            match h4.read_packet().await {
                Ok(HostPacket::Command(packet)) => {
                    let mut guard = read_state.lock().await;
                    let opcode = command_opcode(&packet);
                    let event = guard.handle_command(&packet);
                    if read_event_tx.send(event).await.is_err() {
                        break;
                    }
                    if opcode == OPCODE_LE_SET_ADV_ENABLE && guard.advertising {
                        info!("ESP host enabled LE advertising");
                    }
                }
                Ok(HostPacket::Acl(acl)) => {
                    read_state.lock().await.note_acl_from_host(&acl);
                    if read_acl_from_host_tx.send(acl).await.is_err() {
                        break;
                    }
                }
                Err(err) => {
                    error!(?err, "serial read failed");
                    break;
                }
            }
        }
    });

    let write_task = tokio::spawn(async move {
        let mut h4 = H4Writer::new(writer);
        loop {
            tokio::select! {
                Some(event) = event_rx.recv() => {
                    if h4.write_event(&event).await.is_err() {
                        break;
                    }
                }
                Some(acl) = acl_to_host_rx.recv() => {
                    if h4.write_acl_to_host(&acl).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    radio::run_radio(
        state,
        event_tx,
        acl_to_host_tx,
        acl_from_host_rx,
    )
    .await?;
    write_task.await.ok();
    Ok(())
}
