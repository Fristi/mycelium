use edge_protocol::v2_proto::PlantProfile;

#[derive(Debug, Clone, Default)]
pub struct WateringContext {
    pub last_watered_at: Option<u32>,
    pub last_duration_ms: u32,
}

pub struct WateringPolicy {
    pub base_duration_ms: u32,
    pub ms_per_percent_deficit: u32,
    pub min_duration_ms: u32,
    pub max_duration_ms: u32,
    pub cooldown_secs: u32,
}

impl Default for WateringPolicy {
    fn default() -> Self {
        Self {
            base_duration_ms: 500,
            ms_per_percent_deficit: 100,
            min_duration_ms: 500,
            max_duration_ms: 5000,
            cooldown_secs: 300,
        }
    }
}

pub const DEFAULT_POLICY: WateringPolicy = WateringPolicy {
    base_duration_ms: 500,
    ms_per_percent_deficit: 100,
    min_duration_ms: 500,
    max_duration_ms: 5000,
    cooldown_secs: 300,
};

pub struct WateringDecision {
    pub should_water: bool,
    pub duration_ms: u32,
}

pub fn decide_watering(
    current_pct: u8,
    profile: &PlantProfile,
    context: &WateringContext,
    now_unix: u32,
    policy: &WateringPolicy,
) -> WateringDecision {
    let no_water = WateringDecision {
        should_water: false,
        duration_ms: 0,
    };

    let Some(interval) = profile.r#soil_moisture.as_ref() else {
        return no_water;
    };

    let min = interval.r#start.clamp(0, 100) as u8;
    let max = interval.r#end.clamp(0, 100) as u8;

    if current_pct >= min {
        return no_water;
    }

    if let Some(last_at) = context.last_watered_at {
        let elapsed = now_unix.saturating_sub(last_at);
        if elapsed < policy.cooldown_secs {
            return no_water;
        }
    }

    let target = ((min as i32 + max as i32) / 2).clamp(0, 100) as u8;
    let deficit = target.saturating_sub(current_pct);
    let mut duration = policy.base_duration_ms + policy.ms_per_percent_deficit * deficit as u32;

    if context.last_duration_ms > 0 && current_pct < min {
        duration = ((duration as f32) * 1.5) as u32;
    }

    duration = duration.clamp(policy.min_duration_ms, policy.max_duration_ms);

    WateringDecision {
        should_water: true,
        duration_ms: duration,
    }
}
