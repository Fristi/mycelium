import { useMemo, useState } from "react";
import { Area, AreaChart, CartesianGrid, Line, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
import type { GrowthPeriod, StationMeasurement, StationWatering } from "@backendclient/api";
import EventTimelineBar from "./EventTimelineBar";
import {
  computeTimeRange,
  formatAxisTick,
  MeasurementPeriod,
  PERIOD_OPTIONS,
} from "../lib/timeline";

type MetricKey = "soilPf" | "temperature" | "lux" | "humidity";

const METRICS: { key: MetricKey; header: string; label: string }[] = [
  { key: "soilPf", header: "Soil capacitive", label: "pF" },
  { key: "temperature", header: "Temperature", label: "°C" },
  { key: "lux", header: "Lux", label: "lx" },
  { key: "humidity", header: "Relative humidity", label: "%" },
];

const CHART_MARGIN = { top: 8, right: 16, left: 8, bottom: 0 };
const Y_AXIS_WIDTH = 52;

type Props = {
  measurements: StationMeasurement[];
  waterings: StationWatering[];
  growthPeriods: GrowthPeriod[];
  period: MeasurementPeriod;
  hasProfile: boolean;
  onPeriodChange: (period: MeasurementPeriod) => void;
};

export default function MeasurementEventChart(props: Props) {
  const { measurements, waterings, growthPeriods, period, hasProfile, onPeriodChange } = props;
  const [metric, setMetric] = useState<MetricKey>("soilPf");

  const selected = METRICS.find((m) => m.key === metric) ?? METRICS[0];

  const chartData = useMemo(
    () =>
      measurements.map((m) => ({
        timestamp: new Date(m.on).getTime(),
        on: m.on,
        value: m[metric],
      })),
    [measurements, metric]
  );

  const { rangeStart, rangeEnd } = useMemo(
    () => computeTimeRange(measurements, period),
    [measurements, period]
  );

  return (
    <div className="bg-white shadow sm:rounded-lg mb-5">
      <div className="px-4 py-5 sm:p-6">
        <div className="mb-3 flex flex-wrap items-center justify-between gap-3">
          <div className="flex flex-wrap gap-1">
            {METRICS.map((m) => (
              <button
                key={m.key}
                type="button"
                onClick={() => setMetric(m.key)}
                className={`rounded-md px-3 py-1.5 text-sm font-medium transition ${
                  metric === m.key
                    ? "bg-green-700 text-white"
                    : "bg-gray-100 text-gray-700 hover:bg-gray-200"
                }`}
              >
                {m.header}
              </button>
            ))}
          </div>
          <select
            value={period}
            onChange={(e) => onPeriodChange(e.target.value as MeasurementPeriod)}
            className="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-sm text-gray-700 shadow-sm focus:border-green-500 focus:outline-none focus:ring-1 focus:ring-green-500"
          >
            {PERIOD_OPTIONS.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>

        <h3 className="text-lg font-medium leading-6 text-gray-900">{selected.header}</h3>
        <div className="mt-2 text-sm text-gray-500">
          <ResponsiveContainer width="100%" height={200}>
            <AreaChart data={chartData} margin={CHART_MARGIN}>
              <defs>
                <linearGradient id="measurement-gradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#15803d" stopOpacity={0.1} />
                  <stop offset="95%" stopColor="#FFFFFF" stopOpacity={0.1} />
                </linearGradient>
              </defs>
              <CartesianGrid vertical={false} stroke="#DDD" />
              <XAxis
                type="number"
                dataKey="timestamp"
                domain={[rangeStart, rangeEnd]}
                scale="time"
                tickFormatter={(value) => formatAxisTick(value, period)}
              />
              <YAxis width={Y_AXIS_WIDTH} />
              <Tooltip
                labelFormatter={(value) => formatAxisTick(value, period)}
                formatter={(value: number) => [`${value} ${selected.label}`, selected.header]}
              />
              <Line
                type="monotone"
                unit={selected.label}
                strokeLinecap="round"
                strokeWidth={2}
                style={{ strokeDasharray: "40% 60%" }}
                dataKey="value"
                stroke="#15803d"
                dot={false}
                legendType="none"
              />
              <Area
                type="monotone"
                dataKey="value"
                stroke="#15803d"
                strokeWidth={2}
                fillOpacity={1}
                fill="url(#measurement-gradient)"
              />
            </AreaChart>
          </ResponsiveContainer>

          <div
            className="w-full overflow-visible pb-1 pt-6"
            style={{
              paddingLeft: CHART_MARGIN.left + Y_AXIS_WIDTH,
              paddingRight: CHART_MARGIN.right,
            }}
          >
            <EventTimelineBar
              rangeStart={rangeStart}
              rangeEnd={rangeEnd}
              growthPeriods={growthPeriods}
              waterings={waterings}
              period={period}
              hasProfile={hasProfile}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
