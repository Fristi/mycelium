#![no_std]
#![no_main]

extern crate alloc;

mod processor;
mod battery;
mod moisture;
mod gauge;
mod state;
mod utils;

#[cfg(any(feature = "hardware", feature = "sim"))]
mod ble;

#[cfg(feature = "sim")]
mod hci_uart;

#[cfg(feature = "sim")]
mod sim_processor;

use embassy_executor::Spawner;
use crate::state::{get_device_state, set_device_state};
use {esp_alloc as _, esp_backtrace as _};
use esp_hal::rtc_cntl::sleep::{RtcSleepConfig, TimerWakeupSource};
use log::error;

#[cfg(not(feature = "sim"))]
use crate::processor::{process, DebugProcessor};

#[cfg(feature = "sim")]
use crate::processor::process_sim;

#[cfg(feature = "sim")]
use crate::sim_processor::SimProcessor;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_s: Spawner) {
    let state = get_device_state();
    let mut cfg = RtcSleepConfig::deep();
    cfg.set_rtc_fastmem_pd_en(false);

    #[cfg(feature = "sim")]
    let wakeup_source = TimerWakeupSource::new(core::time::Duration::from_millis(500));

    #[cfg(not(feature = "sim"))]
    let wakeup_source = TimerWakeupSource::new(core::time::Duration::from_secs(1));

    #[cfg(not(feature = "sim"))]
    let result = process(state, DebugProcessor::new()).await;

    #[cfg(feature = "sim")]
    let result = process_sim(state, SimProcessor::new()).await;

    match result {
        Ok(result) => {
            set_device_state(result.next_state);
            let mut rtc = result.rtc;
            rtc.sleep(&cfg, &[&wakeup_source]);
        }
        Err(err) => {
            error!("Process crashed! {}", err);
        }
    }
}
