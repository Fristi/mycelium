use chrono::{DateTime, NaiveDateTime, Utc};

pub trait RtcExt {
    fn now_naivedatetime(&self) -> NaiveDateTime;
    fn set_unix_timestamp(&self, secs: u32);
}

impl<'a> RtcExt for esp_hal::rtc_cntl::Rtc<'a> {
    fn now_naivedatetime(&self) -> NaiveDateTime {
        let now_us = self.current_time_us() as i64;
        let secs = now_us / 1_000_000;
        let nsecs = (now_us % 1_000_000) * 1_000;

        DateTime::<Utc>::from_timestamp(secs, nsecs as u32)
            .map(|dt| dt.naive_utc())
            .unwrap_or_default()
    }

    fn set_unix_timestamp(&self, secs: u32) {
        self.set_current_time_us(secs as u64 * 1_000_000);
    }
}