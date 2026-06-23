use embassy_futures::select::select;

use trouble_host::prelude::*;
use trouble_host::types::gatt_traits::FromGatt;
use trouble_host::BleHostError;
#[expect(unused_imports, reason = "loads AsGatt/FromGatt impls for proto GATT types")]
use edge_protocol::gatt as _;
use edge_protocol::v2::*;
use edge_protocol::v2_proto::{Events, MacAddress, PlantProfileSetting, SyncState, Timestamp};
use esp_hal::rtc_cntl::Rtc;
use log::*;

use crate::state::DeviceStateData;
use crate::utils::rtc::RtcExt;

/// Max number of connections
const CONNECTIONS_MAX: usize = 1;
/// Max number of L2CAP channels.
const L2CAP_CHANNELS_MAX: usize = 2; // Signal + att
#[gatt_service(uuid = STATION_SERVICE_UUID_16)]
struct StationService {
    #[characteristic(uuid = STATION_MAC_ADDR_CHARACTERISTIC_UUID_16, read)]
    address: MacAddress,
    #[characteristic(uuid = STATION_EVENTS_CHARACTERISTIC_UUID_16, read)]
    events: Events,
    #[characteristic(uuid = STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16, read, write)]
    current_profile: PlantProfileSetting,
    #[characteristic(uuid = STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16, read, write)]
    current_time: Timestamp,
    #[characteristic(uuid = STATION_SYNC_STATE_CHARACTERISTIC_UUID_16, read, write)]
    sync_state: SyncState
}

impl StationService {
    fn merge_into(&self, server: &Server<'_>, base: &GattSyncSession) -> Result<GattSyncSession, Error> {
        Ok(GattSyncSession {
            mac: base.mac,
            events: Events::default(),
            current_profile: self.current_profile.get(server)?,
            current_time: self.current_time.get(&server)?,
        })
    }
}

#[gatt_server]
struct Server {
    station_service: StationService
}

/// BLE session snapshot: proto types exposed over GATT for one connection.
#[derive(Debug)]
pub struct GattSyncSession {
    pub mac: [u8; 6],
    pub events: Events,
    /// Written by central over GATT; stored in device state after flush.
    pub current_profile: PlantProfileSetting,
    pub current_time: Timestamp,
}

impl GattSyncSession {
    pub fn init_with_mac(mac: [u8; 6]) -> Self {
        Self {
            mac,
            events: Events::default(),
            current_profile: PlantProfileSetting { profile: None },
            current_time: Timestamp::default(),
        }
    }

    pub fn from_device_state_data(mac: [u8; 6], data: &DeviceStateData) -> Result<Self, ()> {
        Ok(Self {
            mac,
            events: data.to_events()?,
            current_profile: PlantProfileSetting { profile: data.plant_profile.clone() },
            current_time: Timestamp::default(),
        })
    }

    fn to_service(&self, server: &Server<'_>) -> Result<(), Error> {
        let address = mac_address_from_bytes(self.mac).map_err(|_| Error::InvalidValue)?;
        server.station_service.address.set(&server, &address)?;
        server.station_service.events.set(&server, &self.events)?;
        server.station_service.current_profile.set(&server, &self.current_profile)?;
        server
            .station_service
            .current_time
            .set(&server, &self.current_time)?;
        Ok(())
    }
}

/// Run the BLE stack. Pass `Some(rtc)` during `AwaitingTimeSync` to apply central time writes.
pub async fn run<C>(
    controller: C,
    session: &GattSyncSession,
    rtc: Option<&Rtc<'_>>,
) -> Result<GattSyncSession, Error>
where
    C: Controller
{
    let address: Address = Address::random(session.mac);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> = HostResources::new();
    let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
    let Host {
        mut peripheral, runner, ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "Mycelium",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    session.to_service(&server).expect("Unable to set state");

    let res = select(ble_task(runner), async {
        let conn = advertise("Mycelium", &mut peripheral, &server)
            .await
            .map_err(|e| match e {
                BleHostError::BleHost(err) => err,
                _ => Error::InvalidState,
            })?;
        gatt_events_task(&server, &conn, session, rtc).await?;
        server.station_service.merge_into(&server, session)
    })
    .await;

    match res {
        embassy_futures::select::Either::Second(Ok(res)) => Ok(res),
        _ => Err(Error::InvalidState)
    }
}

async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            error!("[ble_task] error: {:?}", e);
        }
    }
}

async fn advertise<'values, 'server, C: Controller>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let service_uuid = STATION_SERVICE_UUID_16.to_le_bytes();

    let mut adv_data = [0u8; 31];
    let adv_len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[service_uuid]),
        ],
        &mut adv_data[..],
    )?;

    let mut scan_data = [0u8; 31];
    let scan_len = AdStructure::encode_slice(
        &[
            AdStructure::CompleteLocalName(name.as_bytes()),
            AdStructure::ServiceUuids16(&[service_uuid]),
        ],
        &mut scan_data[..],
    )?;

    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &adv_data[..adv_len],
                scan_data: &scan_data[..scan_len],
            },
        )
        .await?;
    info!("[adv] advertising as {name} (service 0x{STATION_SERVICE_UUID_16:04x})");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    session: &GattSyncSession,
    rtc: Option<&Rtc<'_>>,
) -> Result<(), Error> {
    let address = mac_address_from_bytes(session.mac).map_err(|_| Error::InvalidValue)?;
    server.station_service.address.set(server, &address)?;
    server.station_service.events.set(server, &session.events)?;

    if rtc.is_some() {
        server
            .station_service
            .sync_state
            .set(server, &SyncState::Ready)?;
    }

    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                let reply = match event {
                    GattEvent::Write(write) => {
                        handle_gatt_write(server, rtc, &write)?;
                        write.accept()?
                    }
                    GattEvent::Read(read) => {
                        if let Some(rtc) = rtc {
                            if read.handle() == server.station_service.current_time.handle {
                                refresh_current_time(server, rtc)?;
                            }
                        }
                        read.accept()?
                    }
                    other => other.accept()?,
                };
                reply.send().await;
            }
            _ => {}
        }
    };
    info!("[gatt] disconnected: {:?}", reason);
    Ok(())
}

fn refresh_current_time(server: &Server<'_>, rtc: &Rtc<'_>) -> Result<(), Error> {
    let secs = (rtc.current_time_us() / 1_000_000) as u32;
    server
        .station_service
        .current_time
        .set(server, &Timestamp { timestamp: secs })
}

fn handle_gatt_write<P: PacketPool>(
    server: &Server<'_>,
    rtc: Option<&Rtc<'_>>,
    write: &WriteEvent<'_, '_, P>,
) -> Result<(), Error> {
    if write.handle() == server.station_service.current_profile.handle {
        let profile_setting: PlantProfileSetting = write
            .value(&server.station_service.current_profile)
            .map_err(|_| Error::InvalidValue)?;
        server
            .station_service
            .current_profile
            .set(server, &profile_setting)?;
        info!("[gatt] plant profile written");
        return Ok(());
    }

    let Some(rtc) = rtc else {
        if write.handle() == server.station_service.sync_state.handle {
            let state: SyncState = write
                .value(&server.station_service.sync_state)
                .map_err(|_| Error::InvalidValue)?;
            info!("[gatt] sync_state write: {}", state.0);
        }
        return Ok(());
    };

    if write.handle() == server.station_service.current_time.handle {
        let ts: Timestamp = write
            .value(&server.station_service.current_time)
            .map_err(|_| Error::InvalidValue)?;

        rtc.set_unix_timestamp(ts.timestamp);
                server
                    .station_service
                    .sync_state
                    .set(server, &SyncState::Done)?;

        return Ok(());
    }

    if write.handle() == server.station_service.sync_state.handle {
        let state: SyncState = write
            .value(&server.station_service.sync_state)
            .map_err(|_| Error::InvalidValue)?;
        info!("[gatt] sync_state write: {}", state.0);
    }

    Ok(())
}

/// Build a wire `MacAddress` from six raw bytes (including zeros).
fn mac_address_from_bytes(value: [u8; 6]) -> Result<MacAddress, ()> {
    let mac_address = heapless::Vec::from_slice(&value).map_err(|_| ())?;
    Ok(MacAddress { mac_address })
}