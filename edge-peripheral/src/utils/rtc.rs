use chrono::NaiveDateTime;

pub trait RtcExt {
    fn now_naivedatetime(&self) -> NaiveDateTime;
    fn set_unix_timestamp(&self, secs: u32);
}

impl<'a> RtcExt for esp_hal::rtc_cntl::Rtc<'a> {
    fn now_naivedatetime(&self) -> NaiveDateTime {
        let now_us = self.current_time_us() as i64;
        let secs = now_us / 1_000_000;
        let nsecs = (now_us % 1_000_000) * 1_000;

        NaiveDateTime::from_timestamp(secs, nsecs as u32)
    }

    fn set_unix_timestamp(&self, secs: u32) {
        self.set_current_time_us(secs as u64 * 1_000_000);
    }
}