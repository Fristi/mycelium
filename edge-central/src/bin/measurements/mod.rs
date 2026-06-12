use std::sync::Arc;

use chrono::TimeDelta;
use crate::cfg::PeripheralSyncMode;
use crate::measurements::random::RandomPeripheralSyncResultStreamProvider;
use crate::measurements::types::PeripheralSyncResultStreamProvider;
use crate::ports::plant_profiles::PlantProfilePort;

pub mod btleplug;
pub mod checkin;
pub mod random;
pub mod sync_metrics;
pub mod types;

pub async fn make_peripheral_sync_stream_provider(
    mode: &PeripheralSyncMode,
    plant_profiles: Arc<dyn PlantProfilePort>,
) -> anyhow::Result<Box<dyn PeripheralSyncResultStreamProvider>> {
    match mode {
        PeripheralSyncMode::Ble => {
            {
                let provider =
                    btleplug::BtleplugPeripheralSyncResultStreamProvider::new(plant_profiles)
                        .await?;

                anyhow::Ok(Box::new(provider))
            }

        }
        PeripheralSyncMode::Random => {
            let provider = RandomPeripheralSyncResultStreamProvider::new(
                [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa],
                TimeDelta::seconds(2),
            );

            anyhow::Ok(Box::new(provider))
        }
    }
}