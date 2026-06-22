use clap::{Args, Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum OnboardingStrategy {
    Ble,
    Local,
}

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum PeripheralSyncMode {
    Ble,
    Random,
}

#[derive(Debug, Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub scope: String,
    pub audience: String,
}

#[derive(Debug, Clone)]
pub struct WifiConfig {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Clone, Default, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum StatusStrategy {
    /// Log status to tracing; no hardware display.
    #[default]
    Noop,
    /// SSD1306 OLED over I2C (Linux aarch64 hub only).
    I2c,
}

#[derive(Debug, Clone, Default, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum WifiProvisionerMode {
    /// Accept BLE WiFi credentials but do not change system networking.
    #[default]
    Noop,
    /// Join WiFi via nmcli or wpa_cli on the hub.
    System,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub backend_url: String,
    pub database_url: String,
    /// In-memory SQLite; no filesystem writes, data discarded on exit.
    pub database_ephemeral: bool,
    pub onboarding_strategy: OnboardingStrategy,
    pub peripheral_sync_mode: PeripheralSyncMode,
    pub plant_profiles_sync_interval_secs: u64,
    pub status_display_page_secs: u64,
    pub status_strategy: StatusStrategy,
    pub auth0: Auth0Config,
    pub wifi: Option<WifiConfig>,
    /// How BLE onboarding applies WiFi credentials (default: noop).
    pub wifi_provisioner: WifiProvisionerMode,
    /// Wireless interface for `system` WiFi provisioning (default: wlan0).
    pub wifi_interface: Option<String>,
}

#[derive(Args, Debug)]
#[command(next_help_heading = "Auth0")]
struct Auth0Args {
    #[arg(
        long = "auth0-domain",
        env = "APP.AUTH0.DOMAIN",
        default_value = "mycelium-greens.eu.auth0.com"
    )]
    domain: String,
    #[arg(
        long = "auth0-client-id",
        env = "APP.AUTH0.CLIENT_ID",
        default_value = "i9p7v3jAPo8z6mwiuCt6IB5dGNAG1xaz"
    )]
    client_id: String,
    #[arg(
        long = "auth0-scope",
        env = "APP.AUTH0.SCOPE",
        default_value = "offline_access"
    )]
    scope: String,
    #[arg(
        long = "auth0-audience",
        env = "APP.AUTH0.AUDIENCE",
        default_value = "mycelium.co"
    )]
    audience: String,
}

#[derive(Args, Debug, Default)]
#[command(next_help_heading = "WiFi")]
struct WifiArgs {
    #[arg(
        long = "wifi-ssid",
        env = "APP.WIFI.SSID",
        default_value = "wifi_user"
    )]
    ssid: Option<String>,
    #[arg(
        long = "wifi-password",
        env = "APP.WIFI.PASSWORD",
        default_value = "wifi_password"
    )]
    password: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "edge-central", about = "Mycelium edge hub")]
struct AppConfigParser {
    #[arg(
        long,
        env = "APP.BACKEND_URL",
        default_value = "https://mycelium.markdejong.org/api"
    )]
    backend_url: String,
    #[arg(
        long,
        env = "APP.DATABASE_URL",
        default_value = "sqlite://mycelium.db"
    )]
    database_url: String,
    #[arg(long, env = "APP.DATABASE_EPHEMERAL", default_value_t = false)]
    database_ephemeral: bool,
    #[arg(
        long,
        env = "APP.ONBOARDING_STRATEGY",
        default_value = "local"
    )]
    onboarding_strategy: OnboardingStrategy,
    #[arg(
        long,
        env = "APP.PERIPHERAL_SYNC_MODE",
        default_value = "ble"
    )]
    peripheral_sync_mode: PeripheralSyncMode,
    #[arg(
        long,
        env = "APP.PLANT_PROFILES_SYNC_INTERVAL_SECS",
        default_value_t = 60
    )]
    plant_profiles_sync_interval_secs: u64,
    #[arg(
        long,
        env = "APP.STATUS_DISPLAY_PAGE_SECS",
        default_value_t = 10
    )]
    status_display_page_secs: u64,
    #[arg(long, env = "APP.STATUS_STRATEGY", default_value = "noop")]
    status_strategy: StatusStrategy,
    #[arg(long, env = "APP.WIFI_PROVISIONER", default_value = "noop")]
    wifi_provisioner: WifiProvisionerMode,
    #[arg(long, env = "APP.WIFI_INTERFACE")]
    wifi_interface: Option<String>,
    #[command(flatten)]
    auth0: Auth0Args,
    #[command(flatten)]
    wifi: WifiArgs,
}

impl TryFrom<AppConfigParser> for AppConfig {
    type Error = anyhow::Error;

    fn try_from(parser: AppConfigParser) -> Result<Self, Self::Error> {
        let wifi = match (parser.wifi.ssid, parser.wifi.password) {
            (Some(ssid), Some(password)) => Some(WifiConfig { ssid, password }),
            (None, None) => None,
            _ => anyhow::bail!("APP.WIFI.SSID and APP.WIFI.PASSWORD must both be set or both omitted"),
        };

        Ok(AppConfig {
            backend_url: parser.backend_url,
            database_url: parser.database_url,
            database_ephemeral: parser.database_ephemeral,
            onboarding_strategy: parser.onboarding_strategy,
            peripheral_sync_mode: parser.peripheral_sync_mode,
            plant_profiles_sync_interval_secs: parser.plant_profiles_sync_interval_secs,
            status_display_page_secs: parser.status_display_page_secs,
            status_strategy: parser.status_strategy,
            auth0: Auth0Config {
                domain: parser.auth0.domain,
                client_id: parser.auth0.client_id,
                scope: parser.auth0.scope,
                audience: parser.auth0.audience,
            },
            wifi,
            wifi_provisioner: parser.wifi_provisioner,
            wifi_interface: parser.wifi_interface,
        })
    }
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<AppConfig> {
        let parsed = AppConfigParser::try_parse().map_err(anyhow::Error::msg)?;
        parsed.try_into()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use std::env;

    /// Load config from environment only (avoids parsing `cargo test` CLI args).
    fn parse_from_env() -> anyhow::Result<AppConfig> {
        AppConfigParser::try_parse_from(["edge-central"])
            .map_err(anyhow::Error::msg)?
            .try_into()
    }

    fn set_required_env() {
        env::set_var("APP.BACKEND_URL", "http://localhost:8080/api");
        env::set_var("APP.DATABASE_URL", "postgres://localhost/test");
        env::set_var("APP.ONBOARDING_STRATEGY", "ble");
        env::set_var("APP.PERIPHERAL_SYNC_MODE", "random");
        env::set_var("APP.AUTH0.DOMAIN", "test.auth0.com");
        env::set_var("APP.AUTH0.CLIENT_ID", "test-client-id");
        env::set_var("APP.AUTH0.SCOPE", "openid profile");
        env::set_var("APP.AUTH0.AUDIENCE", "test-audience");
    }

    fn clear_required_env() {
        env::remove_var("APP.BACKEND_URL");
        env::remove_var("APP.DATABASE_URL");
        env::remove_var("APP.DATABASE_EPHEMERAL");
        env::remove_var("APP.ONBOARDING_STRATEGY");
        env::remove_var("APP.PERIPHERAL_SYNC_MODE");
        env::remove_var("APP.AUTH0.DOMAIN");
        env::remove_var("APP.AUTH0.CLIENT_ID");
        env::remove_var("APP.AUTH0.SCOPE");
        env::remove_var("APP.AUTH0.AUDIENCE");
        env::remove_var("APP.STATUS_STRATEGY");
        env::remove_var("APP.WIFI.SSID");
        env::remove_var("APP.WIFI.PASSWORD");
    }

    #[test]
    #[serial]
    fn test_defaults_without_env() {
        clear_required_env();

        let config = parse_from_env().unwrap();

        assert_eq!(
            config.backend_url,
            "https://mycelium.markdejong.org/api"
        );
        assert_eq!(config.database_url, "sqlite://mycelium.db");
        assert!(!config.database_ephemeral);
        match config.onboarding_strategy {
            OnboardingStrategy::Local => {}
            _ => panic!("Expected OnboardingStrategy::Local"),
        }
        match config.peripheral_sync_mode {
            PeripheralSyncMode::Ble => {}
            _ => panic!("Expected PeripheralSyncMode::Ble"),
        }
        match config.status_strategy {
            StatusStrategy::Noop => {}
            _ => panic!("Expected StatusStrategy::Noop"),
        }
        assert_eq!(config.auth0.domain, "mycelium-greens.eu.auth0.com");
        assert_eq!(config.auth0.scope, "offline_access");
        assert_eq!(config.auth0.audience, "mycelium.co");
        let wifi = config.wifi.as_ref().expect("wifi config");
        assert_eq!(wifi.ssid, "wifi_user");
        assert_eq!(wifi.password, "wifi_password");
    }

    #[test]
    #[serial]
    fn test_from_env() {
        set_required_env();
        env::set_var("APP.WIFI.SSID", "test-wifi");
        env::set_var("APP.WIFI.PASSWORD", "test-password");
        env::set_var("APP.PLANT_PROFILES_SYNC_INTERVAL_SECS", "120");

        let config = parse_from_env().unwrap();

        assert_eq!(config.database_url, "postgres://localhost/test");
        assert_eq!(config.plant_profiles_sync_interval_secs, 120);
        match config.onboarding_strategy {
            OnboardingStrategy::Ble => {}
            _ => panic!("Expected OnboardingStrategy::Ble"),
        }
        match config.peripheral_sync_mode {
            PeripheralSyncMode::Random => {}
            _ => panic!("Expected PeripheralSyncMode::Random"),
        }
        assert_eq!(config.auth0.domain, "test.auth0.com");
        assert_eq!(config.auth0.client_id, "test-client-id");
        assert_eq!(config.auth0.scope, "openid profile");
        assert_eq!(config.auth0.audience, "test-audience");
        let wifi = config.wifi.as_ref().expect("wifi config");
        assert_eq!(wifi.ssid, "test-wifi");
        assert_eq!(wifi.password, "test-password");

        env::remove_var("APP.WIFI.SSID");
        env::remove_var("APP.WIFI.PASSWORD");
        env::remove_var("APP.PLANT_PROFILES_SYNC_INTERVAL_SECS");
        clear_required_env();
    }

    #[test]
    #[serial]
    fn test_from_env_with_different_values() {
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

        let config = parse_from_env().unwrap();

        assert_eq!(config.database_url, "mysql://localhost/test");
        match config.onboarding_strategy {
            OnboardingStrategy::Local => {}
            _ => panic!("Expected OnboardingStrategy::Local"),
        }
        match config.peripheral_sync_mode {
            PeripheralSyncMode::Ble => {}
            _ => panic!("Expected PeripheralSyncMode::Ble"),
        }
        assert_eq!(config.auth0.domain, "other.auth0.com");
        assert_eq!(config.auth0.client_id, "other-client-id");
        assert_eq!(config.auth0.scope, "email");
        assert_eq!(config.auth0.audience, "other-audience");
        let wifi = config.wifi.as_ref().expect("wifi config");
        assert_eq!(wifi.ssid, "other-wifi");
        assert_eq!(wifi.password, "other-password");

        env::remove_var("APP.WIFI.SSID");
        env::remove_var("APP.WIFI.PASSWORD");
        clear_required_env();
    }

    #[test]
    #[serial]
    fn test_wifi_defaults() {
        set_required_env();
        env::remove_var("APP.WIFI.SSID");
        env::remove_var("APP.WIFI.PASSWORD");

        let config = parse_from_env().unwrap();
        let wifi = config.wifi.as_ref().expect("wifi config");
        assert_eq!(wifi.ssid, "wifi_user");
        assert_eq!(wifi.password, "wifi_password");

        clear_required_env();
    }

    #[test]
    #[serial]
    fn test_cli_overrides_env() {
        set_required_env();
        env::set_var("APP.DATABASE_URL", "postgres://localhost/test");

        let config: AppConfig = AppConfigParser::try_parse_from([
            "edge-central",
            "--database-url",
            "sqlite://override.db",
        ])
        .unwrap()
        .try_into()
        .unwrap();

        assert_eq!(config.database_url, "sqlite://override.db");

        clear_required_env();
    }
}
