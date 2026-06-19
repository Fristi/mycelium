import type { GrowthPeriodKind } from "@backendclient/api";
import moment from "moment";

export type MeasurementPeriod = "last-24-hours" | "last-7-days" | "last-2-weeks" | "last-month";

export const PERIOD_OPTIONS: { value: MeasurementPeriod; label: string }[] = [
  { value: "last-24-hours", label: "24 hours" },
  { value: "last-7-days", label: "7 days" },
  { value: "last-2-weeks", label: "2 weeks" },
  { value: "last-month", label: "1 month" },
];

export function formatDuration(start: string, end: string): string {
  const ms = moment(end).diff(moment(start));
  if (ms < 60_000) return `${Math.round(ms / 1000)} sec`;
  if (ms < 3_600_000) return moment.duration(ms).humanize();
  const hours = Math.floor(ms / 3_600_000);
  const mins = Math.round((ms % 3_600_000) / 60_000);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}

function parseTime(value: string | number): moment.Moment {
  return typeof value === "number" ? moment(value) : moment(value);
}

export function formatAxisTick(value: string | number, period: MeasurementPeriod): string {
  const time = parseTime(value);
  if (period === "last-24-hours") return time.format("HH:mm");
  return time.format("MMM D");
}

export function formatInstant(value: string | number, period: MeasurementPeriod): string {
  const time = parseTime(value);
  if (period === "last-24-hours") return time.format("HH:mm");
  return time.format("MMM D HH:mm");
}

export function formatTimeRange(start: string, end: string, period: MeasurementPeriod): string {
  return `${formatInstant(start, period)} – ${formatInstant(end, period)}`;
}

export function formatWateringDuration(durationMsec: number): string {
  if (durationMsec < 60_000) return `${Math.round(durationMsec / 1000)} sec`;
  return moment.duration(durationMsec).humanize();
}

export type PeriodSegmentStyle = {
  label: string;
  bgClass: string;
  icon: "sun" | "thermometer" | "sun-dim" | "sun-high" | "cloud-rain";
};

export function growthPeriodStyle(kind: GrowthPeriodKind): PeriodSegmentStyle {
  if (kind._type === "Productive") {
    return { label: "Productive", bgClass: "bg-teal-500", icon: "sun" };
  }

  const reason = kind.reason._type;
  switch (reason) {
    case "HeatStress":
      return { label: "Heat stress", bgClass: "bg-amber-500", icon: "thermometer" };
    case "LowLight":
      return { label: "Low light", bgClass: "bg-amber-600", icon: "sun-dim" };
    case "HighLight":
      return { label: "Too much light", bgClass: "bg-orange-500", icon: "sun-high" };
    case "LowHumidity":
      return { label: "Low humidity", bgClass: "bg-amber-500", icon: "cloud-rain" };
    case "HighHumidity":
      return { label: "High humidity", bgClass: "bg-amber-500", icon: "cloud-rain" };
    default:
      return { label: "Non-productive", bgClass: "bg-amber-500", icon: "thermometer" };
  }
}

export function segmentPosition(
  start: string,
  end: string,
  rangeStart: number,
  rangeEnd: number
): { left: number; width: number } {
  const total = rangeEnd - rangeStart;
  if (total <= 0) return { left: 0, width: 100 };

  const segStart = Math.max(moment(start).valueOf(), rangeStart);
  const segEnd = Math.min(moment(end).valueOf(), rangeEnd);
  const left = ((segStart - rangeStart) / total) * 100;
  const width = ((segEnd - segStart) / total) * 100;
  return { left, width: Math.max(width, 0) };
}

export function eventsInRange<T extends { start?: string; end?: string; occurredAt?: string }>(
  events: T[],
  rangeStart: number,
  rangeEnd: number
): T[] {
  return events.filter((event) => {
    const start = event.start ?? event.occurredAt;
    const end = event.end ?? event.occurredAt;
    if (!start || !end) return false;
    const segStart = parseTime(start).valueOf();
    const segEnd = parseTime(end).valueOf();
    return segEnd > rangeStart && segStart < rangeEnd;
  });
}

function periodBucketMs(period: MeasurementPeriod): number {
  if (period === "last-24-hours") return 15 * 60 * 1000;
  return 24 * 60 * 60 * 1000;
}

/** Shared chart + event-bar domain from measurement buckets only. */
export function computeTimeRange(
  measurements: { on: string }[],
  period: MeasurementPeriod
): { rangeStart: number; rangeEnd: number } {
  if (measurements.length === 0) {
    const now = Date.now();
    const lookbackMs: Record<MeasurementPeriod, number> = {
      "last-24-hours": 24 * 60 * 60 * 1000,
      "last-7-days": 7 * 24 * 60 * 60 * 1000,
      "last-2-weeks": 14 * 24 * 60 * 60 * 1000,
      "last-month": 31 * 24 * 60 * 60 * 1000,
    };
    return { rangeStart: now - lookbackMs[period], rangeEnd: now };
  }

  const timestamps = measurements.map((m) => parseTime(m.on).valueOf());
  const rangeStart = Math.min(...timestamps);
  const rangeEnd = Math.max(...timestamps) + periodBucketMs(period);

  return { rangeStart, rangeEnd };
}
