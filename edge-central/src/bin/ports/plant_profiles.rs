use std::sync::{Arc, RwLock};
use std::time::Duration;

use edge_client_backend::{
    apis::{configuration::Configuration, default_api},
    models::{PlantProfile, PlantProfileVariables, StationPlantProfile},
};
use edge_protocol::v2_proto::{Interval, PlantProfile as ProtoPlantProfile};

pub trait PlantProfilePort: Send + Sync {
    fn current_profiles(&self) -> Vec<StationPlantProfile>;
    fn profile_for_mac(&self, mac: &[u8; 6]) -> Option<PlantProfile>;
}

#[derive(Debug)]
pub struct CachedPlantProfilePort {
    profiles: Arc<RwLock<Vec<StationPlantProfile>>>,
}

impl CachedPlantProfilePort {
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn update(&self, profiles: Vec<StationPlantProfile>) {
        *self.profiles.write().unwrap() = profiles;
    }
}

impl Default for CachedPlantProfilePort {
    fn default() -> Self {
        Self::new()
    }
}

impl PlantProfilePort for CachedPlantProfilePort {
    fn current_profiles(&self) -> Vec<StationPlantProfile> {
        self.profiles.read().unwrap().clone()
    }

    fn profile_for_mac(&self, mac: &[u8; 6]) -> Option<PlantProfile> {
        let mac_str = format_mac(mac);
        self.profiles
            .read()
            .unwrap()
            .iter()
            .find(|p| normalize_mac(&p.mac) == mac_str)
            .map(|p| (*p.profile).clone())
    }
}

pub async fn run_profile_sync(
    configuration: Configuration,
    store: Arc<CachedPlantProfilePort>,
    interval: Duration,
) {
    loop {
        match default_api::get_profiles(&configuration).await {
            Ok(profiles) => {
                tracing::info!("Fetched {} plant profile(s)", profiles.len());
                store.update(profiles);
            }
            Err(err) => tracing::warn!(?err, "Failed to fetch plant profiles"),
        }
        tokio::time::sleep(interval).await;
    }
}

pub fn format_mac(mac: &[u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

fn normalize_mac(s: &str) -> String {
    s.to_lowercase().replace('-', ":")
}

fn interval_to_proto(interval: &edge_client_backend::models::IntervalA) -> Interval {
    Interval {
        r#start: interval.start,
        r#end: interval.end,
    }
}

pub fn api_profile_to_proto(variables: &PlantProfileVariables) -> ProtoPlantProfile {
    ProtoPlantProfile::default()
        .init_light_mmol(interval_to_proto(&variables.light_mmol))
        .init_light_lux(interval_to_proto(&variables.light_lux))
        .init_temperature(interval_to_proto(&variables.temperature))
        .init_humidity(interval_to_proto(&variables.humidity))
        .init_soil_moisture(interval_to_proto(&variables.soil_moisture))
        .init_soil_ec(interval_to_proto(&variables.soil_ec))
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_client_backend::models::IntervalA;

    fn sample_profile(name: &str) -> PlantProfile {
        let interval = IntervalA::new(1, 2);
        PlantProfile::new(
            name.to_string(),
            PlantProfileVariables::new(
                interval.clone(),
                interval.clone(),
                interval.clone(),
                interval.clone(),
                interval.clone(),
                interval,
            ),
        )
    }

    #[test]
    fn profile_for_mac_matches_colon_separated_address() {
        let mac = [0x40, 0xf5, 0x20, 0xb7, 0x85, 0x40];
        let store = CachedPlantProfilePort::new();
        store.update(vec![StationPlantProfile::new(
            uuid::Uuid::nil(),
            "40:f5:20:b7:85:40".to_string(),
            sample_profile("Schefflera"),
        )]);

        let profile = store.profile_for_mac(&mac).expect("profile should match");
        assert_eq!(profile.name, "Schefflera");
    }
}
