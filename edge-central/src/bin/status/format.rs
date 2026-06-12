use chrono::{DateTime, Utc};

use crate::data::types::StationSyncSummary;
use crate::status::SyncSummaryDisplay;

pub fn empty_sync_summary_display() -> SyncSummaryDisplay {
    SyncSummaryDisplay {
        lines: ["No sync data".to_string(), String::new(), String::new()],
        page: None,
    }
}

pub fn format_sync_summary_display(
    summary: &StationSyncSummary,
    page: Option<(usize, usize)>,
) -> SyncSummaryDisplay {
    let battery = summary
        .min_battery
        .map(|b| format!("Bat:{b}%"))
        .unwrap_or_else(|| "Bat:--".to_string());

    SyncSummaryDisplay {
        lines: [
            summary.mac.clone(),
            format!(
                "M:{} W:{} {}",
                summary.measurement_count, summary.watering_count, battery
            ),
            format!(
                "Drift:{}s Sync:{}",
                summary.avg_time_drift_secs,
                format_hhmm(summary.max_synced_at)
            ),
        ],
        page,
    }
}

fn format_hhmm(dt: DateTime<Utc>) -> String {
    dt.format("%H:%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_summary() -> StationSyncSummary {
        StationSyncSummary {
            station_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            mac: "40:f5:20:b7:85:40".to_string(),
            measurement_count: 42,
            watering_count: 5,
            min_battery: Some(72),
            avg_time_drift_secs: -3,
            max_watered_at: DateTime::<Utc>::from_timestamp(1_700_000_600, 0),
            max_synced_at: DateTime::<Utc>::from_timestamp(1_700_001_920, 0).unwrap(),
        }
    }

    #[test]
    fn formats_summary_lines() {
        let summary = sample_summary();
        let display = format_sync_summary_display(&summary, None);
        assert_eq!(display.lines[0], "40:f5:20:b7:85:40");
        assert_eq!(display.lines[1], "M:42 W:5 Bat:72%");
        assert_eq!(
            display.lines[2],
            format!(
                "Drift:-3s Sync:{}",
                summary.max_synced_at.format("%H:%M")
            )
        );
        assert!(display.page.is_none());
    }

    #[test]
    fn includes_page_when_multiple_stations() {
        let display = format_sync_summary_display(&sample_summary(), Some((2, 3)));
        assert_eq!(display.page, Some((2, 3)));
    }

    #[test]
    fn empty_display_shows_message() {
        let display = empty_sync_summary_display();
        assert_eq!(display.lines[0], "No sync data");
        assert!(display.page.is_none());
    }

    #[test]
    fn null_battery_shows_dash() {
        let mut summary = sample_summary();
        summary.min_battery = None;
        let display = format_sync_summary_display(&summary, None);
        assert_eq!(display.lines[1], "M:42 W:5 Bat:--");
    }
}
