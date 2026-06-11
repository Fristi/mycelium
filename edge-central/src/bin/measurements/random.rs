use std::{pin::Pin, time::Duration};

use chrono::TimeDelta;
use edge_protocol::v2_proto::Events;
use futures::{stream, Stream};
use tokio::time::sleep;

use crate::measurements::types::{PeripheralSyncResult, PeripheralSyncResultStreamProvider};

pub struct RandomPeripheralSyncResultStreamProvider {
    pub mac: [u8; 6],
    pub delay: TimeDelta,
}

impl RandomPeripheralSyncResultStreamProvider {
    pub fn new(mac: [u8; 6], delay: TimeDelta) -> Self {
        Self { mac, delay }
    }
}

impl PeripheralSyncResultStreamProvider for RandomPeripheralSyncResultStreamProvider {
    fn stream(self: Box<Self>) -> Pin<Box<dyn Stream<Item = Vec<PeripheralSyncResult>>>> {
        let delay = Duration::from_millis(self.delay.num_milliseconds() as u64);
        let mac = self.mac;
        let stream = stream::unfold((delay, mac), |(delay, mac)| async move {

            let result = PeripheralSyncResult {
                address: mac,
                time_drift: TimeDelta::zero(),
                events: Events::default(),
            };

            sleep(delay).await;

            Some((vec![result], (delay, mac)))
        });

        Box::pin(stream)
    }
}