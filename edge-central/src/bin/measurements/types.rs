use std::pin::Pin;

use chrono::Duration;
use edge_protocol::v2_proto::Events;
use futures::Stream;

#[derive(Debug)]
pub struct PeripheralSyncResult {
    pub address: [u8; 6],
    pub time_drift: Duration,
    pub events: Events,
}

pub trait PeripheralSyncResultStreamProvider {
    fn stream(self: Box<Self>) -> Pin<Box<dyn Stream<Item = Vec<PeripheralSyncResult>>>>;
}
