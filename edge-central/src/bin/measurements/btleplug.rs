use std::pin::Pin;
use std::sync::Arc;

use anyhow::anyhow;
use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use chrono::{DateTime, Duration, Utc};
use edge_protocol::v2::{
    decode_proto, encode_proto, STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16,
    STATION_EVENTS_CHARACTERISTIC_UUID_16, STATION_MAC_ADDR_CHARACTERISTIC_UUID_16,
    STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16, STATION_SERVICE_UUID_16,
};
use edge_protocol::v2_proto::{Events, MacAddress, Timestamp};
use futures::Stream;
use tokio::time::{sleep, timeout, Duration as TokioDuration};
use tracing::info;
use uuid::Uuid;

use crate::measurements::types::{PeripheralSyncResult, PeripheralSyncResultStreamProvider};
use crate::ports::plant_profiles::{api_profile_to_proto, PlantProfilePort};

const CONNECT_TIMEOUT: TokioDuration = TokioDuration::from_secs(10);
const READ_RETRY_DELAY: TokioDuration = TokioDuration::from_millis(200);
const READ_RETRIES: usize = 5;

// trouble-host registers GATT services/chars with 16-bit UUIDs; btleplug expects the
// Bluetooth base UUID form (0000XXXX-0000-1000-8000-00805f9b34fb), not the custom 128-bit IDs.
const STATION_SERVICE: Uuid = uuid_from_u16(STATION_SERVICE_UUID_16);
const STATION_MAC_ADDR_CHARACTERISTIC: Uuid = uuid_from_u16(STATION_MAC_ADDR_CHARACTERISTIC_UUID_16);
const STATION_EVENTS_CHARACTERISTIC: Uuid = uuid_from_u16(STATION_EVENTS_CHARACTERISTIC_UUID_16);
const STATION_CURRENT_TIME_CHARACTERISTIC: Uuid =
    uuid_from_u16(STATION_CURRENT_TIME_CHARACTERISTIC_UUID_16);
const STATION_PLANT_PROFILE_CHARACTERISTIC: Uuid =
    uuid_from_u16(STATION_PLANT_PROFILE_CHARACTERISTIC_UUID_16);

pub struct BtleplugPeripheralSyncResultStreamProvider {
    adapter: Arc<Adapter>,
    plant_profiles: Arc<dyn PlantProfilePort>,
}

impl BtleplugPeripheralSyncResultStreamProvider {
    pub async fn new(plant_profiles: Arc<dyn PlantProfilePort>) -> anyhow::Result<Self> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let adapter = adapters
            .into_iter()
            .next()
            .ok_or(anyhow!("No adapter found"))?;

        Ok(BtleplugPeripheralSyncResultStreamProvider {
            adapter: Arc::new(adapter),
            plant_profiles,
        })
    }
}

impl PeripheralSyncResultStreamProvider for BtleplugPeripheralSyncResultStreamProvider {
    fn stream(self: Box<Self>) -> Pin<Box<dyn Stream<Item = Vec<PeripheralSyncResult>>>> {
        let adapter = self.adapter.clone();
        let plant_profiles = self.plant_profiles.clone();
        let stream = futures::stream::unfold((adapter, plant_profiles), |(adapter, plant_profiles)| async move {
            if let Err(err) = adapter
                .start_scan(ScanFilter {
                    services: vec![STATION_SERVICE],
                })
                .await
            {
                tracing::error!(?err, "Btleplug error occurred");
                return None;
            }

            sleep(TokioDuration::from_secs(1)).await;

            let peripherals = adapter.peripherals().await.ok()?;
            let mut results = vec![];

            if let Err(err) = adapter.stop_scan().await {
                info!("Error occurred while stop scanning: {:?}", err);
            }

            tracing::info!("Found {:?} peripherals", peripherals);

            for peripheral in peripherals {
                let now = Utc::now();
                match sync(peripheral, now, plant_profiles.as_ref()).await {
                    Err(err) => tracing::warn!(?err, "Sync error occurred"),
                    Ok(result) => results.push(result),
                }
            }

            Some((results, (adapter, plant_profiles)))
        });

        Box::pin(stream)
    }
}

async fn sync(
    peripheral: Peripheral,
    now: DateTime<Utc>,
    plant_profiles: &dyn PlantProfilePort,
) -> anyhow::Result<PeripheralSyncResult> {
    if peripheral.properties().await?.and_then(|p| p.local_name).as_deref() != Some("Mycelium")
    {
        return Err(anyhow!("Skipping non-Mycelium peripheral"));
    }

    timeout(CONNECT_TIMEOUT, peripheral.connect())
        .await
        .map_err(|_| anyhow!("Connect timed out after {CONNECT_TIMEOUT:?}"))??;

    peripheral.discover_services().await?;

    let services = peripheral.services();
    info!("Services: {:?}", services);

    let service = services
        .iter()
        .find(|s| s.uuid == STATION_SERVICE)
        .ok_or_else(|| anyhow!("Device does not have {STATION_SERVICE} service"))?;

    let mac_addr_char = service
        .characteristics
        .iter()
        .find(|c| c.uuid == STATION_MAC_ADDR_CHARACTERISTIC)
        .ok_or_else(|| {
            anyhow!("Device does not have {STATION_MAC_ADDR_CHARACTERISTIC} characteristic")
        })?;

    let events_char = service
        .characteristics
        .iter()
        .find(|c| c.uuid == STATION_EVENTS_CHARACTERISTIC)
        .ok_or_else(|| {
            anyhow!("Device does not have {STATION_EVENTS_CHARACTERISTIC} characteristic")
        })?;

    let time_char = service
        .characteristics
        .iter()
        .find(|c| c.uuid == STATION_CURRENT_TIME_CHARACTERISTIC)
        .ok_or_else(|| {
            anyhow!("Device does not have {STATION_CURRENT_TIME_CHARACTERISTIC} characteristic")
        })?;

    let profile_char = service
        .characteristics
        .iter()
        .find(|c| c.uuid == STATION_PLANT_PROFILE_CHARACTERISTIC)
        .ok_or_else(|| {
            anyhow!("Device does not have {STATION_PLANT_PROFILE_CHARACTERISTIC} characteristic")
        })?;

    let mac_addr_data = read_characteristic(&peripheral, mac_addr_char).await?;
    let mac_address: MacAddress = decode_proto(&mac_addr_data)
        .map_err(|e| anyhow!("Failed to decode MacAddress protobuf: {e:?}"))?;
    let address = mac_address_to_bytes(&mac_address)
        .map_err(|_| anyhow!("MacAddress characteristic did not contain 6 bytes"))?;

    info!(
        "Found device {:?} (station mac {:02x?})",
        peripheral.address(),
        address
    );

    let time_drift = write_current_time(&peripheral, time_char, now).await?;

    if let Some(api_profile) = plant_profiles.profile_for_mac(&address) {
        sync_plant_profile(&peripheral, profile_char, &api_profile).await?;
    } else {
        info!(
            "No plant profile found for station mac {:02x?}",
            address
        );
    }

    let events_data = read_characteristic(&peripheral, events_char).await?;
    let events: Events = decode_proto(&events_data)
        .map_err(|e| anyhow!("Failed to decode Events protobuf: {e:?}"))?;

    peripheral.disconnect().await?;

    Ok(PeripheralSyncResult {
        address,
        time_drift,
        events,
    })
}

async fn sync_plant_profile(
    peripheral: &Peripheral,
    profile_char: &btleplug::api::Characteristic,
    api_profile: &edge_client_backend::models::PlantProfile,
) -> anyhow::Result<()> {
    use btleplug::api::Peripheral as _;

    let proto_profile = api_profile_to_proto(&api_profile.variables);
    let mut buf = [0u8; 64];
    let len = encode_proto(&proto_profile, &mut buf)
        .map_err(|_| anyhow!("Failed to encode PlantProfile"))?;

    peripheral
        .write(profile_char, &buf[..len], WriteType::WithResponse)
        .await?;

    info!(
        "Wrote plant profile {:?} ({} bytes)",
        api_profile.name, len
    );

    Ok(())
}

async fn write_current_time(
    peripheral: &Peripheral,
    time_char: &btleplug::api::Characteristic,
    now: DateTime<Utc>,
) -> anyhow::Result<Duration> {
    use btleplug::api::Peripheral as _;

    let central_secs = now.timestamp();
    if central_secs < 0 {
        return Ok(Duration::zero());
    }

    let timestamp = Timestamp {
        timestamp: central_secs as u32,
    };
    let mut buf = [0u8; 8];
    let len = encode_proto(&timestamp, &mut buf).map_err(|_| anyhow!("Failed to encode Timestamp"))?;

    peripheral
        .write(time_char, &buf[..len], WriteType::WithResponse)
        .await?;

    let read_back = read_characteristic(peripheral, time_char).await?;
    let station_time: Timestamp = decode_proto(&read_back)
        .map_err(|e| anyhow!("Failed to decode Timestamp after write: {e:?}"))?;

    let drift_secs = station_time.timestamp as i64 - central_secs;
    info!(
        "Wrote unix time {central_secs}, station reports {} (drift {drift_secs}s)",
        station_time.timestamp
    );

    Ok(Duration::seconds(drift_secs))
}

async fn read_characteristic(
    peripheral: &Peripheral,
    characteristic: &btleplug::api::Characteristic,
) -> anyhow::Result<Vec<u8>> {
    use btleplug::api::Peripheral as _;

    for attempt in 0..READ_RETRIES {
        let data = peripheral.read(characteristic).await?;
        if !data.is_empty() {
            return Ok(data);
        }
        if attempt + 1 < READ_RETRIES {
            sleep(READ_RETRY_DELAY).await;
        }
    }
    Ok(vec![])
}

fn mac_address_to_bytes(value: &MacAddress) -> Result<[u8; 6], ()> {
    value
        .r#mac_address()
        .clone()
        .into_array()
        .map_err(|_| ())
}
