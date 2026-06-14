use std::collections::HashMap;
use std::sync::Arc;

use bluer::{
    adv::Advertisement,
    gatt::local::{
        Application, Characteristic, CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod,
        ReqError, Service,
    },
};
use edge_protocol::v2::{
    STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16, STATION_EVENTS_CHARACTERISTIC_UUID_16,
    STATION_MAC_ADDR_CHARACTERISTIC_UUID_16, STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16,
    STATION_SERVICE_UUID_16, STATION_SYNC_STATE_CHARACTERISTIC_UUID_16,
};
use futures::FutureExt;
use tokio::sync::{mpsc, Mutex};
use tracing::{info, warn};
use uuid::Uuid;

use crate::acl::{acl_packet, att_read_request, att_write_request, l2cap_att, parse_att_read_response};
use crate::controller::ControllerState;

const ADV_NAME: &str = "Mycelium";

fn uuid16(short: u16) -> Uuid {
    Uuid::parse_str(&format!("0000{short:04x}-0000-1000-8000-00805f9b34fb")).expect("valid uuid16")
}

async fn ensure_esp_connection(
    state: &Arc<Mutex<ControllerState>>,
    event_tx: &mpsc::Sender<Vec<u8>>,
) -> Result<(), ReqError> {
    let needs = state.lock().await.connection_handle.is_none();
    if needs {
        let evt = state
            .lock()
            .await
            .connect_esp([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        event_tx.send(evt).await.map_err(|_| ReqError::Failed)?;
    }
    Ok(())
}

pub async fn run_radio(
    state: Arc<Mutex<ControllerState>>,
    event_tx: mpsc::Sender<Vec<u8>>,
    acl_to_host_tx: mpsc::Sender<Vec<u8>>,
    mut acl_from_host_rx: mpsc::Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let service_uuid = uuid16(STATION_SERVICE_UUID_16);
    let handles = Arc::new(Mutex::new(HashMap::from([
        (uuid16(STATION_MAC_ADDR_CHARACTERISTIC_UUID_16), 0x0003),
        (uuid16(STATION_EVENTS_CHARACTERISTIC_UUID_16), 0x0005),
        (uuid16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16), 0x0007),
        (uuid16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16), 0x0009),
        (uuid16(STATION_SYNC_STATE_CHARACTERISTIC_UUID_16), 0x000B),
    ])));

    let pending_acl: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    let adv_state = state.clone();
    let adapter_adv = adapter.clone();
    tokio::spawn(async move {
        loop {
            let advertising = adv_state.lock().await.advertising;
            if advertising {
                let adv = Advertisement {
                    advertisement_type: bluer::adv::Type::Peripheral,
                    service_uuids: vec![service_uuid],
                    local_name: Some(ADV_NAME.to_string()),
                    discoverable: Some(true),
                    ..Default::default()
                };
                match adapter_adv.advertise(adv).await {
                    Ok(_handle) => {
                        info!("Advertising {ADV_NAME} via BlueZ");
                        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    }
                    Err(err) => {
                        warn!(?err, "advertise failed, retrying");
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        }
    });

    let read_char = |uuid: Uuid| {
        let acl_to_host_tx = acl_to_host_tx.clone();
        let pending_acl = pending_acl.clone();
        let state = state.clone();
        let handles = handles.clone();
        let event_tx = event_tx.clone();
        CharacteristicRead {
            read: true,
            fun: Box::new(move |_req| {
                let acl_to_host_tx = acl_to_host_tx.clone();
                let pending_acl = pending_acl.clone();
                let state = state.clone();
                let handles = handles.clone();
                let event_tx = event_tx.clone();
                async move {
                    ensure_esp_connection(&state, &event_tx).await?;
                    let esp_handle = *handles.lock().await.get(&uuid).ok_or(ReqError::Failed)?;
                    let conn = state.lock().await.connection_handle.ok_or(ReqError::Failed)?;
                    let att = att_read_request(esp_handle);
                    let acl = acl_packet(conn, &l2cap_att(&att));
                    acl_to_host_tx
                        .send(acl)
                        .await
                        .map_err(|_| ReqError::Failed)?;

                    for _ in 0..50 {
                        if let Some(resp) = pending_acl.lock().await.take() {
                            if let Some(value) = parse_att_read_response(&resp) {
                                return Ok(value);
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                    }
                    Err(ReqError::Failed)
                }
                .boxed()
            }),
            ..Default::default()
        }
    };

    let write_char = |uuid: Uuid| {
        let acl_to_host_tx = acl_to_host_tx.clone();
        let state = state.clone();
        let handles = handles.clone();
        let event_tx = event_tx.clone();
        CharacteristicWrite {
            write: true,
            write_without_response: false,
            method: CharacteristicWriteMethod::Fun(Box::new(move |new_value, _req| {
                let acl_to_host_tx = acl_to_host_tx.clone();
                let state = state.clone();
                let handles = handles.clone();
                let event_tx = event_tx.clone();
                async move {
                    ensure_esp_connection(&state, &event_tx).await?;
                    let esp_handle = *handles.lock().await.get(&uuid).ok_or(ReqError::Failed)?;
                    let conn = state.lock().await.connection_handle.ok_or(ReqError::Failed)?;
                    let att = att_write_request(esp_handle, &new_value);
                    let acl = acl_packet(conn, &l2cap_att(&att));
                    acl_to_host_tx
                        .send(acl)
                        .await
                        .map_err(|_| ReqError::Failed)?;
                    Ok(())
                }
                .boxed()
            })),
            ..Default::default()
        }
    };

    let app = Application {
        services: vec![Service {
            uuid: service_uuid,
            primary: true,
            characteristics: vec![
                Characteristic {
                    uuid: uuid16(STATION_MAC_ADDR_CHARACTERISTIC_UUID_16),
                    read: Some(read_char(uuid16(STATION_MAC_ADDR_CHARACTERISTIC_UUID_16))),
                    ..Default::default()
                },
                Characteristic {
                    uuid: uuid16(STATION_EVENTS_CHARACTERISTIC_UUID_16),
                    read: Some(read_char(uuid16(STATION_EVENTS_CHARACTERISTIC_UUID_16))),
                    ..Default::default()
                },
                Characteristic {
                    uuid: uuid16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16),
                    read: Some(read_char(uuid16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16))),
                    write: Some(write_char(uuid16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16))),
                    ..Default::default()
                },
                Characteristic {
                    uuid: uuid16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16),
                    read: Some(read_char(uuid16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16))),
                    write: Some(write_char(uuid16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16))),
                    ..Default::default()
                },
                Characteristic {
                    uuid: uuid16(STATION_SYNC_STATE_CHARACTERISTIC_UUID_16),
                    read: Some(read_char(uuid16(STATION_SYNC_STATE_CHARACTERISTIC_UUID_16))),
                    write: Some(write_char(uuid16(STATION_SYNC_STATE_CHARACTERISTIC_UUID_16))),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    };

    let _app_handle = adapter.serve_gatt_application(app).await?;

    while let Some(acl) = acl_from_host_rx.recv().await {
        *pending_acl.lock().await = Some(acl);
    }

    Ok(())
}
