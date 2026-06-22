pub mod cfg;
pub mod data;
pub mod auth;
pub mod measurements;
pub mod onboarding;
pub mod ports;
pub mod status;

use aliri_reqwest::AccessTokenMiddleware;
use aliri_tokens::{backoff, jitter, sources::{self, oauth2::dto::RefreshTokenCredentialsSource}, ClientId, RefreshToken, TokenLifetimeConfig, TokenWatcher};
use anyhow::*;
use edge_client_backend::{
    apis::{configuration::Configuration, default_api},
    models::StationInsert,
};
use futures::{stream, StreamExt};
use reqwest::{Client, Request, Url};
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use chrono::Utc;
use crate::data::pool::connect_pool;
use crate::data::sqlite::{SqliteEdgeStateRepository, SqliteSyncSessionRepository};
use crate::data::types::SyncSessionRecord;
use crate::measurements::checkin::{events_to_checkin, log_checkin_station_error};
use crate::measurements::sync_metrics::extract_sync_session_metrics;
use crate::measurements::types::PeripheralSyncResult;
use crate::ports::plant_profiles::format_mac;
use crate::cfg::AppConfig;
use crate::measurements::make_peripheral_sync_stream_provider;
use crate::onboarding::make_onboarding;
use crate::ports::plant_profiles::{CachedPlantProfilePort, run_profile_sync};

#[tokio::main]
async fn main() {
    // Install a subscriber that logs to stdout with TRACE level enabled
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE) // allow trace level logs
        .init();

    if let Err(e) = work().await {
        tracing::error!(?e, "Application crashed");
        std::process::exit(1);
    }
}



async fn work() -> anyhow::Result<()> {
    let app_config = AppConfig::from_env()?;
    if app_config.database_ephemeral {
        tracing::info!("using ephemeral in-memory database (no disk writes)");
    }

    let pool = connect_pool(&app_config).await?;

    let edge_state_repo = SqliteEdgeStateRepository::new(pool.clone());
    let sync_session_repo = Arc::new(SqliteSyncSessionRepository::new(pool.clone()));
    let _edge_state = match edge_state_repo.get_state().await? {
        Some(state) => state,
        None => {
            let onboarding = make_onboarding(&app_config).await?;
            let edge_state = onboarding.process().await?;
            edge_state_repo.set_state(&edge_state).await?;
            edge_state
        }
    };

    let status = Arc::new(Mutex::new(crate::status::make_status(app_config.status_strategy.clone())?));
    let page_secs = Duration::from_secs(app_config.status_display_page_secs);
    tokio::spawn(crate::status::display_loop::run_sync_summary_display(
        Arc::clone(&sync_session_repo),
        status,
        page_secs,
    ));

    let jitter_source = jitter::NullJitter;
    let refresh_token = _edge_state.auth0_refresh_token.clone();
    let refresh_token_ref = RefreshToken::new(refresh_token).into_boxed_ref();
    let client_id = ClientId::new(app_config.auth0.client_id);
    let token_url = Url::parse(format!("https://{}/oauth/token", &app_config.auth0.domain).as_str())?;
    let credentials = RefreshTokenCredentialsSource { 
        client_id: client_id,
        client_secret: None,
        refresh_token: refresh_token_ref
    };
    let lifetime_config = TokenLifetimeConfig::default();
    let token_source = sources::oauth2::RefreshTokenSource::new(
        Client::default(), 
        token_url, 
        credentials, 
        lifetime_config
    );

    let token_watcher = TokenWatcher::spawn_from_token_source(token_source, jitter_source, backoff::ErrorBackoffConfig::default()).await?;

    let client = ClientBuilder::new(Client::default())
        .with(AccessTokenMiddleware::new(token_watcher).with_predicate(AlwaysMatch))
        .with(TracingMiddleware::default())
        .build();

    let configuration: Configuration = Configuration {
        base_path: app_config.backend_url,
        user_agent: None,
        client: client,
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None                
    };

    let plant_profile_store = Arc::new(CachedPlantProfilePort::new());
    let sync_interval =
        Duration::from_secs(app_config.plant_profiles_sync_interval_secs);
    tokio::spawn(run_profile_sync(
        configuration.clone(),
        plant_profile_store.clone(),
        sync_interval,
    ));

    let provider = make_peripheral_sync_stream_provider(
        &app_config.peripheral_sync_mode,
        plant_profile_store,
    )
    .await?;
    let stream = provider.stream().flat_map(stream::iter);

    stream
        .for_each(|m| async {
            if let Err(err) =
                sync_measurements(&configuration, sync_session_repo.as_ref(), m).await
            {
                tracing::error!("Failed to sync measurements {}", err);
            }
        })
        .await;

    Ok(())
}

async fn sync_measurements(
    configuration: &Configuration,
    sync_session_repo: &SqliteSyncSessionRepository,
    m: PeripheralSyncResult,
) -> anyhow::Result<()> {
    let mac = format_mac(&m.address);

    let station_insert = StationInsert::new(mac.clone(), "Unnamed".to_string());

    let station_id = default_api::add_station(configuration, station_insert).await?;
    let checkin_events = events_to_checkin(&m.events)?;
    if checkin_events.is_empty() {
        return Ok(());
    }

    if let Some(metrics) = extract_sync_session_metrics(&m.events) {
        let record = SyncSessionRecord {
            synced_at: Utc::now(),
            station_id: station_id.to_string(),
            mac: mac.clone(),
            measurement_count: i32::try_from(metrics.measurement_count)?,
            watering_count: i32::try_from(metrics.watering_count)?,
            min_battery: metrics
                .min_battery
                .map(i32::from),
            time_drift_secs: m.time_drift.num_seconds(),
            watered_at: metrics.watered_at,
        };

        if let Err(err) = sync_session_repo.insert_session(&record).await {
            tracing::warn!(?err, %station_id, "failed to record sync session metrics");
        }
    }

    let event_count = checkin_events.len();
    let inserted = default_api::checkin_station(
        configuration,
        &station_id.to_string(),
        Some(checkin_events),
    )
    .await
    .map_err(|err| {
        log_checkin_station_error(&err, &station_id, &mac, event_count);
        err
    })?;
    tracing::info!(%station_id, inserted, "checkin complete");



    Ok(())
}

#[derive(Debug, Clone)]
pub struct AlwaysMatch;

impl std::fmt::Display for AlwaysMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlwaysMatch")
    }
}

impl predicates_core::reflection::PredicateReflection for AlwaysMatch {}

impl predicates_core::Predicate<Request> for AlwaysMatch {
    fn eval(&self, _variable: &Request) -> bool {
        true
    }
}