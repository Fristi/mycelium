use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tracing::warn;

use crate::data::sqlite::SqliteSyncSessionRepository;
use crate::status::format::{empty_sync_summary_display, format_sync_summary_display};
use crate::status::Status;

pub async fn run_sync_summary_display(
    repo: Arc<SqliteSyncSessionRepository>,
    status: Arc<Mutex<Box<dyn Status>>>,
    page_secs: Duration,
) {
    loop {
        match repo.list_station_summaries().await {
            Ok(summaries) if summaries.is_empty() => {
                show_page(&status, &empty_sync_summary_display()).await;
                tokio::time::sleep(page_secs).await;
            }
            Ok(summaries) => {
                let total = summaries.len();
                for (index, summary) in summaries.iter().enumerate() {
                    let page = if total > 1 {
                        Some((index + 1, total))
                    } else {
                        None
                    };
                    let display = format_sync_summary_display(summary, page);
                    show_page(&status, &display).await;
                    tokio::time::sleep(page_secs).await;
                }
            }
            Err(err) => {
                warn!(?err, "failed to load sync session summaries for display");
                tokio::time::sleep(page_secs).await;
            }
        }
    }
}

async fn show_page(status: &Arc<Mutex<Box<dyn Status>>>, display: &crate::status::SyncSummaryDisplay) {
    let mut guard = status.lock().await;
    if let Err(err) = guard.show_sync_summary(display) {
        warn!(?err, "failed to update sync summary display");
    }
}
