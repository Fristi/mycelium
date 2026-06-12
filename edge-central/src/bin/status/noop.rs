use crate::status::{OnboardingDisplay, Status, SyncSummaryDisplay};
use tracing::info;

pub struct NoopStatus;

impl NoopStatus {
    pub fn new() -> Self {
        Self {}
    }
}

impl Status for NoopStatus {
    fn show_onboarding(&mut self, onboarding: &OnboardingDisplay) -> anyhow::Result<()> {
        info!(
            line1 = %onboarding.line1,
            line2 = ?onboarding.line2,
            "onboarding display"
        );
        Ok(())
    }

    fn show_sync_summary(&mut self, summary: &SyncSummaryDisplay) -> anyhow::Result<()> {
        info!(
            line1 = %summary.lines[0],
            line2 = %summary.lines[1],
            line3 = %summary.lines[2],
            page = ?summary.page,
            "sync summary display"
        );
        Ok(())
    }
}
