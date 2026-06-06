#![no_std]
#![no_main]

mod processor;
mod battery;
mod moisture;
mod gauge;
mod state;
mod utils;
mod ble;

use embassy_executor::Spawner;
use crate::{processor::{process, DebugProcessor}, state::get_device_state, state::set_device_state};
use {esp_alloc as _, esp_backtrace as _};
use esp_hal::rtc_cntl::sleep::{RtcSleepConfig, TimerWakeupSource};
use log::{error};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_s: Spawner) {
    let state = get_device_state();
    let mut cfg = RtcSleepConfig::deep();
    cfg.set_rtc_fastmem_pd_en(false);
    let wakeup_source = TimerWakeupSource::new(core::time::Duration::from_secs(1));

    match process(state, DebugProcessor::new()).await {
        Ok(result) => {
            set_device_state(result.next_state);
            let mut rtc = result.rtc;
            rtc.sleep(&cfg, &[&wakeup_source]);
        },
        Err(err) => {
            error!("Process crashed! {}", err);
        }
    }
}
