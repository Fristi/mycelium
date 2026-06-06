use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingStrategy {
    Ble,
    Local,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PeripheralSyncMode {
    Ble,
    Random,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub scope: String,
    pub audience: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
}

fn default_plant_profiles_sync_interval_secs() -> u64 {
    60
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub backend_url: String,
    pub database_url: String,
    pub onboarding_strategy: OnboardingStrategy,
    pub peripheral_sync_mode: PeripheralSyncMode,
    #[serde(default = "default_plant_profiles_sync_interval_secs")]
    pub plant_profiles_sync_interval_secs: u64,
    pub auth0: Auth0Config,
    pub wifi: WifiConfig,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<AppConfig> {
        let config = Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .list_separator(",")
                    .separator("."),
            )
            .build()?;

        let app: AppConfig = config.try_deserialize()?;

        Ok(app)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use std::env;

    #[test]
    #[serial]
    fn test_from_env() {
        // Set environment variables for testing
        env::set_var("APP.BACKEND_URL", "http://localhost:8080/api");
        env::set_var("APP.DATABASE_URL", "postgres://localhost/test");
        env::set_var("APP.ONBOARDING_STRATEGY", "ble");
        env::set_var("APP.PERIPHERAL_SYNC_MODE", "random");
        env::set_var("APP.AUTH0.DOMAIN", "test.auth0.com");
        env::set_var("APP.AUTH0.CLIENT_ID", "test-client-id");
        env::set_var("APP.AUTH0.SCOPE", "openid profile");
        env::set_var("APP.AUTH0.AUDIENCE", "test-audience");
        env::set_var("APP.WIFI.SSID", "test-wifi");
        env::set_var("APP.WIFI.PASSWORD", "test-password");
        env::set_var("APP.PLANT_PROFILES_SYNC_INTERVAL_SECS", "120");

        // Load configuration from environment
        let config = AppConfig::from_env().unwrap();

        // Verify the configuration was loaded correctly
        assert_eq!(config.database_url, "postgres://localhost/test");
        assert_eq!(config.plant_profiles_sync_interval_secs, 120);
        match config.onboarding_strategy {
            OnboardingStrategy::Ble => {}
            _ => panic!("Expected OnboardingStrategy::Ble"),
        }
        match config.peripheral_sync_mode {
            PeripheralSyncMode::Random => {}
            _ => panic!("Expected PeripherhalSyncMode::Random"),
        }
        assert_eq!(config.auth0.domain, "test.auth0.com");
        assert_eq!(config.auth0.client_id, "test-client-id");
        assert_eq!(config.auth0.scope, "openid profile");
        assert_eq!(config.auth0.audience, "test-audience");
        assert_eq!(config.wifi.ssid, "test-wifi");
        assert_eq!(config.wifi.password, "test-password");

        // Clean up environment variables
        env::remove_var("APP.BACKEND_URL");
        env::remove_var("APP.DATABASE_URL");
        env::remove_var("APP.ONBOARDING_STRATEGY");
        env::remove_var("APP.PERIPHERAL_SYNC_MODE");
        env::remove_var("APP.AUTH0.DOMAIN");
        env::remove_var("APP.AUTH0.CLIENT_ID");
        env::remove_var("APP.AUTH0.SCOPE");
        env::remove_var("APP.AUTH0.AUDIENCE");
        env::remove_var("APP.WIFI.SSID");
        env::remove_var("APP.WIFI.PASSWORD");
        env::remove_var("APP.PLANT_PROFILES_SYNC_INTERVAL_SECS");
    }

    #[test]
    #[serial]
    fn test_from_env_with_different_values() {
        // Set environment variables for testing
        env::set_var("APP.BACKEND_URL", "http://localhost:8080/api");
        env::set_var("APP.DATABASE_URL", "mysql://localhost/test");
        env::set_var("APP.ONBOARDING_STRATEGY", "local");
        env::set_var("APP.PERIPHERAL_SYNC_MODE", "ble");
        env::set_var("APP.AUTH0.DOMAIN", "other.auth0.com");
        env::set_var("APP.AUTH0.CLIENT_ID", "other-client-id");
        env::set_var("APP.AUTH0.SCOPE", "email");
        env::set_var("APP.AUTH0.AUDIENCE", "other-audience");
        env::set_var("APP.WIFI.SSID", "other-wifi");
        env::set_var("APP.WIFI.PASSWORD", "other-password");

        // Load configuration from environment
        let config = AppConfig::from_env().unwrap();

        // Verify the configuration was loaded correctly
        assert_eq!(config.database_url, "mysql://localhost/test");
        match config.onboarding_strategy {
            OnboardingStrategy::Local => {}
            _ => panic!("Expected OnboardingStrategy::Local"),
        }
        match config.peripheral_sync_mode {
            PeripheralSyncMode::Ble => {}
            _ => panic!("Expected PeripherhalSyncMode::Ble"),
        }
        assert_eq!(config.auth0.domain, "other.auth0.com");
        assert_eq!(config.auth0.client_id, "other-client-id");
        assert_eq!(config.auth0.scope, "email");
        assert_eq!(config.auth0.audience, "other-audience");
        assert_eq!(config.wifi.ssid, "other-wifi");
        assert_eq!(config.wifi.password, "other-password");

        // Clean up environment variables
        env::remove_var("APP.BACKEND_URL");
        env::remove_var("APP.DATABASE_URL");
        env::remove_var("APP.ONBOARDING_STRATEGY");
        env::remove_var("APP.PERIPHERAL_SYNC_MODE");
        env::remove_var("APP.AUTH0.DOMAIN");
        env::remove_var("APP.AUTH0.CLIENT_ID");
        env::remove_var("APP.AUTH0.SCOPE");
        env::remove_var("APP.AUTH0.AUDIENCE");
    }

    #[test]
    #[serial]
    fn test_from_env_missing_values() {
        // Clear relevant environment variables
        env::remove_var("APP.BACKEND_URL");
        env::remove_var("APP.DATABASE_URL");
        env::remove_var("APP.ONBOARDING_STRATEGY");
        env::remove_var("APP.PERIPHERAL_SYNC_MODE");
        env::remove_var("APP.AUTH0.DOMAIN");
        env::remove_var("APP.AUTH0.CLIENT_ID");
        env::remove_var("APP.AUTH0.SCOPE");
        env::remove_var("APP.AUTH0.AUDIENCE");

        // Attempt to load configuration should fail
        let result = AppConfig::from_env();
        assert!(result.is_err());
    }
}
