use esp_hal::peripherals::GPIO34;
use esp_hal::rng::Rng;
use esp_hal::rtc_cntl::Rtc;
use log::info;
use trouble_host::prelude::ExternalController;

use edge_protocol::v2_proto::{Timestamp, WateringEntry};

use crate::ble;
use crate::gauge::Gauge;
use crate::hci_uart::EspUartTransport;
use crate::state::{DeviceState, DeviceStateData};
use crate::utils::rtc::RtcExt;

/// Minimum soil_pf increase between samples to record a watering event.
const WATERING_SOIL_PF_DELTA: f32 = 5.0;

/// Assumed watering duration when inferred from a soil_pf jump.
const WATERING_DURATION_MSEC: u32 = 15_000;

pub struct SimProcessor;

impl SimProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl SimProcessor {
    pub async fn awaiting_time_sync(
        &self,
        rtc: &Rtc<'_>,
        mac: [u8; 6],
        controller: ExternalController<EspUartTransport<'_>, 20>,
    ) -> anyhow::Result<DeviceState> {
        info!("Sim awaiting time sync over HCI UART ...");

        let session = ble::GattSyncSession::init_with_mac(mac);
        let _station_state = ble::run(controller, &session, Some(rtc))
            .await
            .map_err(|e| anyhow::anyhow!("Sim BLE time sync failed: {e:?}"))?;

        embassy_time::Timer::after_millis(300).await;

        Ok(DeviceState::Buffering(DeviceStateData::empty()))
    }

    pub async fn buffering(
        &self,
        state: &DeviceStateData,
        rtc: &Rtc<'_>,
        gauge: &mut Gauge<'_, GPIO34<'_>>,
        _rng: Rng,
    ) -> anyhow::Result<DeviceState> {
        info!(
            "Sim measuring ... {}/{}",
            state.measurements.buckets.len(),
            crate::state::MAX_ENTRIES_MEASUREMENTS
        );

        let sample = gauge.sample().await?;
        let mut data = state.clone();

        if let Some(prev) = data.measurements.buckets.last() {
            let delta = sample.soil_pf - prev.value.soil_pf;
            if delta >= WATERING_SOIL_PF_DELTA {
                let mut watering = WateringEntry::default();
                watering.set_occurred_at(timestamp_from_naive(rtc.now_naivedatetime())?);
                watering.r#duration_msec = WATERING_DURATION_MSEC;
                data.waterings.push(watering).map_err(|_| {
                    anyhow::anyhow!("Watering buffer full")
                })?;
                info!(
                    "Sim watering detected: soil_pf +{:.2} pF (now {:.2})",
                    delta, sample.soil_pf
                );
            }
        }

        data.measurements
            .append_monotonic(rtc.now_naivedatetime(), sample);

        let next_state = if data.is_full() {
            DeviceState::Flush(data)
        } else {
            DeviceState::Buffering(data)
        };

        Ok(next_state)
    }

    pub async fn flushing(
        &self,
        state: &DeviceStateData,
        _rtc: &Rtc<'_>,
        _gauge: &mut Gauge<'_, GPIO34<'_>>,
        mac: [u8; 6],
        controller: ExternalController<EspUartTransport<'_>, 20>,
        _rng: Rng,
    ) -> anyhow::Result<DeviceState> {
        info!(
            "Sim syncing {} measurement bucket(s) and {} watering event(s) over HCI UART BLE",
            state.measurements.buckets.len(),
            state.waterings.len()
        );

        let session = ble::GattSyncSession::from_device_state_data(mac, state)
            .map_err(|_| anyhow::anyhow!("Failed to encode device state for BLE"))?;

        ble::run(controller, &session, None)
            .await
            .map_err(|e| anyhow::anyhow!("Sim BLE sync failed: {e:?}"))?;

        Ok(DeviceState::Buffering(DeviceStateData::empty()))
    }
}

fn timestamp_from_naive(
    dt: chrono::NaiveDateTime,
) -> anyhow::Result<Timestamp> {
    let ts = dt.and_utc().timestamp();
    if ts < 0 {
        anyhow::bail!("RTC timestamp before unix epoch");
    }
    Ok(Timestamp {
        timestamp: ts as u32,
    })
}
