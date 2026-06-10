use core::cell::RefCell;

use esp_hal::analog::adc::{Adc, AdcConfig};
use esp_hal::clock::CpuClock;
use esp_hal::efuse::{self, InterfaceMacAddress};
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::peripherals::GPIO34;
use esp_hal::rng::Rng;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;

use log::info;
use trouble_host::prelude::ExternalController;

use crate::battery::BatteryMeasurement;
use crate::gauge::Gauge;
use crate::state::{DeviceState, DeviceStateData};
use crate::utils::anyhow::ResultAny;
use crate::utils::rtc::RtcExt;
use crate::{ble, state};

pub struct ProcessorResult {
    pub next_state: DeviceState,
    pub rtc: esp_hal::rtc_cntl::Rtc<'static>,
}

pub trait Processor {
    async fn awaiting_time_sync(
        &self,
        rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        mac: [u8; 6],
        controller: trouble_host::prelude::ExternalController<BleConnector<'_>, 20>,
    ) -> anyhow::Result<DeviceState>;

    async fn buffering(
        &self,
        state: &DeviceStateData,
        rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        gauge: &mut crate::gauge::Gauge<'_, GPIO34<'_>>,
        rng: esp_hal::rng::Rng,
    ) -> anyhow::Result<DeviceState>;

    async fn flushing(
        &self,
        state: &DeviceStateData,
        rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        gauge: &mut crate::gauge::Gauge<'_, GPIO34<'_>>,
        mac: [u8; 6],
        controller: trouble_host::prelude::ExternalController<BleConnector<'_>, 20>,
        rng: esp_hal::rng::Rng,
    ) -> anyhow::Result<DeviceState>;
}

pub struct DebugProcessor;

impl DebugProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Processor for DebugProcessor {
    async fn awaiting_time_sync(
        &self,
        rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        mac: [u8; 6],
        controller: trouble_host::prelude::ExternalController<
            esp_radio::ble::controller::BleConnector<'_>,
            20,
        >,
    ) -> anyhow::Result<state::DeviceState> {
        info!("Awaiting time sync ... ");

        let session = ble::GattSyncSession::init_with_mac(mac);
        let station_state = ble::run(controller, &session, Some(rtc))
            .await
            .map_err(|e| anyhow::anyhow!("BLE time sync failed: {e:?}"))?;

        if station_state.current_time.timestamp != 0 {
            info!(
                "Time synced to unix {}",
                station_state.current_time.timestamp
            );
        } else {
            info!("BLE session ended without time write");
        }

        embassy_time::Timer::after_millis(300).await;

        Ok(state::DeviceState::Buffering(
            state::DeviceStateData::empty(),
        ))
    }

    async fn buffering(
        &self,
        state: &state::DeviceStateData,
        rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        gauge: &mut crate::gauge::Gauge<'_, GPIO34<'_>>,
        _rng: esp_hal::rng::Rng,
    ) -> anyhow::Result<state::DeviceState> {
        info!("Measuring ... {}/6 ", &state.measurements.buckets.len());

        let sample = gauge.sample().await?;
        let mut data = state.clone();

        data.measurements
            .append_monotonic(rtc.now_naivedatetime(), sample);

        let next_state = if data.is_full() {
            state::DeviceState::Flush(data)
        } else {
            state::DeviceState::Buffering(data)
        };

        Ok(next_state)
    }

    async fn flushing(
        &self,
        state: &state::DeviceStateData,
        _rtc: &esp_hal::rtc_cntl::Rtc<'_>,
        _gauge: &mut crate::gauge::Gauge<'_, GPIO34<'_>>,
        mac: [u8; 6],
        controller: trouble_host::prelude::ExternalController<
            esp_radio::ble::controller::BleConnector<'_>,
            20,
        >,
        _rng: esp_hal::rng::Rng,
    ) -> anyhow::Result<state::DeviceState> {
        info!(
            "Syncing {} measurement bucket(s) and {} watering event(s) over BLE",
            state.measurements.buckets.len(),
            state.waterings.len()
        );

        let session = ble::GattSyncSession::from_device_state_data(mac, state)
            .map_err(|_| anyhow::anyhow!("Failed to encode device state for BLE"))?;

        ble::run(controller, &session, None)
            .await
            .map_err(|e| anyhow::anyhow!("BLE sync failed: {e:?}"))?;

        Ok(state::DeviceState::Buffering(
            state::DeviceStateData::empty(),
        ))
    }
}

pub async fn process<P: Processor>(
    state: &DeviceState,
    processor: P,
) -> anyhow::Result<ProcessorResult> {
    let cpu_clock = match state {
        DeviceState::AwaitingTimeSync | DeviceState::Flush(_) => CpuClock::max(),
        DeviceState::Buffering(_) => CpuClock::_80MHz,
    };

    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(cpu_clock));
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    let mac_addr = efuse::interface_mac_address(InterfaceMacAddress::Bluetooth);
    let mac: [u8; 6] = mac_addr
        .as_bytes()
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid MAC address length"))?;
    let mut rtc = Rtc::new(peripherals.LPWR);
    let rng = Rng::new();

    match state {
        DeviceState::AwaitingTimeSync => {
            let bluetooth = peripherals.BT;
            let connector = BleConnector::new(bluetooth, Default::default())
                .map_err(|e| anyhow::anyhow!("Failed to init BLE: {e:?}"))?;
            let controller = ExternalController::new(connector);

            let next_state = processor.awaiting_time_sync(&rtc, mac, controller).await?;
            let processor_result = ProcessorResult { next_state, rtc };

            Ok(processor_result)
        }
        DeviceState::Buffering(data) => {
            let adc_pin = peripherals.GPIO34;
            let mut adc_config = AdcConfig::new();
            let pin = adc_config.enable_pin(adc_pin, esp_hal::analog::adc::Attenuation::_11dB);
            let adc = Adc::new(peripherals.ADC1, adc_config);
            let output_config_pcb = OutputConfig::default();

            let pcb_pwr = Output::new(
                peripherals.GPIO23,
                esp_hal::gpio::Level::High,
                output_config_pcb,
            );

            let i2c_pcb = esp_hal::i2c::master::I2c::new(
                peripherals.I2C0,
                esp_hal::i2c::master::Config::default(),
            )
            .with_anyhow("Failed to init i2c pcb")?
            .with_sda(peripherals.GPIO21)
            .with_scl(peripherals.GPIO22);

            let i2c_pcb_refcell = RefCell::new(i2c_pcb);

            let output_config_ext = OutputConfig::default()
                .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain)
                .with_pull(esp_hal::gpio::Pull::Up);

            let i2c_ext = esp_hal::i2c::master::I2c::new(
                peripherals.I2C1,
                esp_hal::i2c::master::Config::default(),
            )
            .with_anyhow("Failed to init i2c ext")?
            .with_sda(peripherals.GPIO27)
            .with_scl(peripherals.GPIO26);

            let i2c_ext_refcell = RefCell::new(i2c_ext);

            let battery = BatteryMeasurement::new(adc, pin);
            let mut gauge = Gauge::new(i2c_pcb_refcell, i2c_ext_refcell, pcb_pwr, battery);

            let next_state = processor.buffering(&data, &rtc, &mut gauge, rng).await?;
            let processor_result = ProcessorResult { next_state, rtc };

            Ok(processor_result)
        }
        DeviceState::Flush(data) => {
            let adc_pin = peripherals.GPIO34;
            let mut adc_config = AdcConfig::new();
            let pin = adc_config.enable_pin(adc_pin, esp_hal::analog::adc::Attenuation::_11dB);
            let adc = Adc::new(peripherals.ADC1, adc_config);
            let output_config_pcb = OutputConfig::default();

            let pcb_pwr = Output::new(
                peripherals.GPIO23,
                esp_hal::gpio::Level::High,
                output_config_pcb,
            );

            let i2c_pcb = esp_hal::i2c::master::I2c::new(
                peripherals.I2C0,
                esp_hal::i2c::master::Config::default(),
            )
            .with_anyhow("Failed to init i2c pcb")?
            .with_sda(peripherals.GPIO21)
            .with_scl(peripherals.GPIO22);

            let i2c_pcb_refcell = RefCell::new(i2c_pcb);

            let output_config_ext = OutputConfig::default()
                .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain)
                .with_pull(esp_hal::gpio::Pull::Up);

            let i2c_ext = esp_hal::i2c::master::I2c::new(
                peripherals.I2C1,
                esp_hal::i2c::master::Config::default(),
            )
            .with_anyhow("Failed to init i2c ext")?
            .with_sda(peripherals.GPIO27)
            .with_scl(peripherals.GPIO26);

            let i2c_ext_refcell = RefCell::new(i2c_ext);

            let battery = BatteryMeasurement::new(adc, pin);
            let mut gauge = Gauge::new(i2c_pcb_refcell, i2c_ext_refcell, pcb_pwr, battery);

            let bluetooth = peripherals.BT;
            let connector = BleConnector::new(bluetooth, Default::default())
                .map_err(|e| anyhow::anyhow!("Failed to init BLE: {e:?}"))?;
            let controller = ExternalController::new(connector);

            let next_state = processor
                .flushing(&data, &rtc, &mut gauge, mac, controller, rng)
                .await?;
            let processor_result = ProcessorResult { next_state, rtc };

            Ok(processor_result)
        }
    }
}
