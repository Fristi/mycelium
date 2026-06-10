use crate::status::{OnboardingDisplay, Status};
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
}
