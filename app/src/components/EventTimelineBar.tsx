import type { GrowthPeriod, StationWatering } from "@backendclient/api";
import { CloudRain, Droplets, Sun, SunDim, ThermometerSun } from "lucide-react";
import { useLayoutEffect, useMemo, useRef, useState, type CSSProperties, type ReactNode } from "react";
import {
  eventsInRange,
  formatDuration,
  formatTimeRange,
  formatWateringDuration,
  growthPeriodStyle,
  MeasurementPeriod,
  segmentPosition,
} from "../lib/timeline";

const MIN_WATERING_WIDTH_PX = 20;
const ICON_SIZE_PX = 14;
const TIMELINE_HEIGHT_PX = 28;
const WATERING_STICKOUT_PX = 12;

type Props = {
  rangeStart: number;
  rangeEnd: number;
  growthPeriods: GrowthPeriod[];
  waterings: StationWatering[];
  period: MeasurementPeriod;
  hasProfile: boolean;
};

type PeriodIconName = ReturnType<typeof growthPeriodStyle>["icon"];

function PeriodIcon({ icon }: { icon: PeriodIconName }) {
  const className = "block h-3.5 w-3.5 shrink-0 text-white";
  switch (icon) {
    case "sun":
      return <Sun className={className} strokeWidth={2} />;
    case "thermometer":
      return <ThermometerSun className={className} strokeWidth={2} />;
    case "sun-dim":
      return <SunDim className={className} strokeWidth={2} />;
    case "sun-high":
      return <Sun className={className} strokeWidth={2} />;
    case "cloud-rain":
      return <CloudRain className={className} strokeWidth={2} />;
  }
}

function segmentShowsIcon(widthPercent: number, barWidthPx: number): boolean {
  return barWidthPx > 0 && (widthPercent / 100) * barWidthPx >= ICON_SIZE_PX;
}

type HoverTargetProps = {
  label: string;
  detail: string;
  style: CSSProperties;
  children?: ReactNode;
  className?: string;
};

function HoverTarget({ label, detail, style, children, className = "" }: HoverTargetProps) {
  return (
    <div className={`group absolute cursor-default ${className}`} style={style}>
      {children}
      <div
        role="tooltip"
        className="pointer-events-none absolute bottom-[calc(100%+6px)] left-1/2 z-50 hidden -translate-x-1/2 rounded-md bg-gray-900 px-2.5 py-1.5 text-left text-white shadow-lg group-hover:block"
      >
        <p className="whitespace-nowrap text-xs font-medium">{label}</p>
        <p className="whitespace-nowrap text-[11px] text-gray-300">{detail}</p>
      </div>
    </div>
  );
}

type WateringMarkerProps = {
  label: string;
  detail: string;
  centerPercent: number;
};

function WateringMarker({ label, detail, centerPercent }: WateringMarkerProps) {
  return (
    <HoverTarget
      label={label}
      detail={detail}
      className="z-30 flex min-w-[20px] flex-col items-center justify-start rounded-t-md bg-blue-500 pt-1 shadow-md ring-1 ring-blue-700/25"
      style={{
        left: `calc(${centerPercent}% - ${MIN_WATERING_WIDTH_PX / 2}px)`,
        width: `${MIN_WATERING_WIDTH_PX}px`,
        bottom: 0,
        height: `${TIMELINE_HEIGHT_PX + WATERING_STICKOUT_PX}px`,
      }}
    >
      <Droplets className="block h-3.5 w-3.5 shrink-0 text-white" strokeWidth={2} />
    </HoverTarget>
  );
}

export default function EventTimelineBar(props: Props) {
  const { rangeStart, rangeEnd, growthPeriods, waterings, period, hasProfile } = props;
  const barRef = useRef<HTMLDivElement>(null);
  const [barWidth, setBarWidth] = useState(0);

  useLayoutEffect(() => {
    const element = barRef.current;
    if (!element) return;

    const update = (width: number) => setBarWidth(width);
    update(element.getBoundingClientRect().width);

    const observer = new ResizeObserver(([entry]) => update(entry.contentRect.width));
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  const visibleGrowthPeriods = eventsInRange(growthPeriods, rangeStart, rangeEnd);
  const visibleWaterings = eventsInRange(waterings, rangeStart, rangeEnd);

  const growthSegments = useMemo(
    () =>
      visibleGrowthPeriods
        .map((segment, idx) => {
          const { left, width } = segmentPosition(segment.start, segment.end, rangeStart, rangeEnd);
          if (width <= 0) return null;

          const style = growthPeriodStyle(segment.kind);
          const duration = formatDuration(segment.start, segment.end);
          const timeRange = formatTimeRange(segment.start, segment.end, period);

          return {
            key: `growth-${idx}`,
            left,
            width,
            label: style.label,
            detail: `${duration} · ${timeRange}`,
            bgClass: style.bgClass,
            icon: style.icon,
          };
        })
        .filter((segment): segment is NonNullable<typeof segment> => segment !== null),
    [visibleGrowthPeriods, rangeStart, rangeEnd, period]
  );

  return (
    <div
      ref={barRef}
      className="relative w-full"
      style={{ height: TIMELINE_HEIGHT_PX + WATERING_STICKOUT_PX }}
    >
      {/* Layer 1: segment backgrounds (clipped to bar shape) */}
      <div
        className="absolute inset-x-0 bottom-0 overflow-hidden rounded-sm bg-gray-100"
        style={{ height: TIMELINE_HEIGHT_PX }}
      >
        {growthSegments.map((segment) => (
          <div
            key={`${segment.key}-bg`}
            className={`absolute inset-y-0 ${segment.bgClass}`}
            style={{ left: `${segment.left}%`, width: `${segment.width}%` }}
          />
        ))}
      </div>

      {!hasProfile && growthSegments.length === 0 && (
        <div
          className="absolute inset-x-0 bottom-0 z-10 flex items-center justify-center rounded-sm bg-gray-200 text-xs text-gray-500"
          style={{ height: TIMELINE_HEIGHT_PX }}
        >
          Set a plant profile to see productive periods
        </div>
      )}

      {/* Layer 2: icons above all backgrounds so neighbors cannot cover them */}
      <div
        className="pointer-events-none absolute inset-x-0 bottom-0 z-10"
        style={{ height: TIMELINE_HEIGHT_PX }}
      >
        {growthSegments.map((segment) =>
          segmentShowsIcon(segment.width, barWidth) ? (
            <div
              key={`${segment.key}-icon`}
              className="absolute inset-y-0 flex items-center justify-center"
              style={{ left: `${segment.left}%`, width: `${segment.width}%` }}
            >
              <PeriodIcon icon={segment.icon} />
            </div>
          ) : null
        )}
      </div>

      {/* Layer 3: invisible hover targets for growth segments */}
      {growthSegments.map((segment) => (
        <HoverTarget
          key={`${segment.key}-hover`}
          label={segment.label}
          detail={segment.detail}
          className="z-20"
          style={{
            left: `${segment.left}%`,
            width: `${segment.width}%`,
            bottom: 0,
            height: TIMELINE_HEIGHT_PX,
          }}
        />
      ))}

      {/* Layer 4: watering markers that stick out above the bar */}
      {visibleWaterings.map((watering, idx) => {
        const center = segmentPosition(watering.occurredAt, watering.occurredAt, rangeStart, rangeEnd).left;
        const duration = formatWateringDuration(watering.durationMsec);
        const timeRange = formatTimeRange(watering.occurredAt, watering.occurredAt, period);

        return (
          <WateringMarker
            key={`watering-${idx}`}
            label="Watered"
            detail={`${duration} · ${timeRange}`}
            centerPercent={center}
          />
        );
      })}
    </div>
  );
}
