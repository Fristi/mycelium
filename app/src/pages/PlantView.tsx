import { Link, useParams } from "react-router-dom";
import AreaGraph from "../components/AreaGraph";
import {
  PlantProfile,
  StationDetails,
  StationLog,
  StationMeasurement,
  WateringSchedule,
  avatarUrl,
  getStationDetails,
  getStationLog,
  getStationProfile,
} from "../api";
import Retrieve from "../Retrieve";
import PlantLocation from "../components/PlantLocation";
import { PlantProfileDisplay } from "../components/PlantProfile";
import { CalendarDaysIcon, EyeDropperIcon } from "@heroicons/react/20/solid";
import { Activity, Leaf } from "lucide-react";
import moment from "moment";

type ScheduleChangedProps = {
  schedule: WateringSchedule;
  on: string;
  lastItem: boolean;
};

const relativeDate = (date: string) => {
  return moment(date).startOf("hour").fromNow();
};

const PlantLogItemScheduleChanged = (props: ScheduleChangedProps) => {
  return (
    <li>
      <div className="relative pb-8">
        {!props.lastItem && <span className="absolute left-5 top-5 -ml-px h-full w-0.5 bg-gray-200" aria-hidden="true" />}
        <div className="relative flex items-start space-x-3">
          <div>
            <div className="relative px-1">
              <div className="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 ring-8 ring-white">
                <CalendarDaysIcon className="h-5 w-5 text-emerald-500" aria-hidden="true" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </li>
  );
};

type WateredProps = {
  period: string;
  on: string;
  lastItem: boolean;
};

const PlantLogItemWatered = (props: WateredProps) => {
  return (
    <li>
      <div className="relative pb-8">
        {!props.lastItem && <span className="absolute left-5 top-5 -ml-px h-full w-0.5 bg-gray-200" aria-hidden="true" />}
        <div className="relative flex items-start space-x-3">
          <div>
            <div className="relative px-1">
              <div className="flex h-8 w-8 items-center justify-center rounded-full bg-gray-100 ring-8 ring-white">
                <EyeDropperIcon className="h-5 w-5 text-blue-500" aria-hidden="true" />
              </div>
            </div>
          </div>
          <div className="min-w-0 flex-1 py-1.5">
            <div className="text-sm text-gray-500">
              Watered plant for <span className="font-semibold">{props.period}</span> - {relativeDate(props.on)}
            </div>
          </div>
        </div>
      </div>
    </li>
  );
};

type PlantLogProps = { plantId: string };

const PlantLog = (props: PlantLogProps) => {
  const renderEvent = (item: StationLog, idx: number, lastItem: boolean) => {
    const event = item.event as { _type?: string; period?: string; schedule?: WateringSchedule };
    switch (event._type) {
      case "ScheduleChanged":
        return (
          <PlantLogItemScheduleChanged
            key={`item-${idx}`}
            on={item.on}
            schedule={event.schedule ?? { _type: "Interval", schedule: "", period: "" }}
            lastItem={lastItem}
          />
        );
      case "Watered":
        return <PlantLogItemWatered key={`item-${idx}`} on={item.on} period={event.period ?? ""} lastItem={lastItem} />;
      default:
        return null;
    }
  };

  const renderPlantLog = (log: StationLog[]) => {
    return (
      <div className="flow-root">
        <ul role="list" className="-mb-8">
          {log.map((item, idx) => renderEvent(item, idx, idx === log.length - 1))}
        </ul>
      </div>
    );
  };

  return <Retrieve dataKey={`plants/${props.plantId}/log`} retriever={getStationLog(props.plantId)} renderData={renderPlantLog} />;
};

const NoMeasurements = () => {
  return (
    <div className="mb-4 max-w-lg md:mt-4 lg:mt-20">
      <div className="flex items-center gap-2 text-green-600 dark:text-green-400">
        <Activity className="h-5 w-5" />
        <p className="text-base/8 font-semibold">No data</p>
      </div>
      <h1 className="text-1xl mt-4 font-semibold tracking-tight text-pretty text-gray-900 sm:text-6xl dark:text-white">No measurements</h1>
      <p className="mt-6 text-lg font-medium text-pretty text-gray-500 sm:text-xl/8 dark:text-gray-400">
        Once measurements are recorded, they'll appear here with detailed insights and trends over time.
      </p>
    </div>
  );
};

type MeasurementSeries = {
  batteryVoltage: { on: string; value: number }[];
  humidity: { on: string; value: number }[];
  lux: { on: string; value: number }[];
  soilPf: { on: string; value: number }[];
  tankPf: { on: string; value: number }[];
  temperature: { on: string; value: number }[];
};

export const PlantView = () => {
  const { plantId } = useParams();

  const NoProfileCard = () => (
    <div className="relative mx-auto overflow-hidden rounded-2xl bg-white p-6 shadow-md">
      <Leaf className="pointer-events-none absolute bottom-4 right-4 h-24 w-24 text-green-100 opacity-70" />
      <div className="relative z-10 flex flex-col items-center text-center">
        <h2 className="mb-2 text-xl font-semibold text-gray-800">No Profile detected</h2>
        <p className="text-sm text-gray-500">
          You haven't selected a plant profile yet. Upload a picture of your plant and we'll try to classify it.
        </p>
        <Link to={`/plants/${plantId ?? ""}/avatar`} className="mt-4 rounded-lg bg-green-500 px-4 py-2 text-white transition hover:bg-green-600">
          Upload picture
        </Link>
      </div>
    </div>
  );

  const splitMeasurements = (data?: StationMeasurement[]): MeasurementSeries | undefined => {
    if (!data?.length) {
      return undefined;
    }

    return {
      batteryVoltage: data.map((x) => ({ on: x.on, value: x.batteryVoltage })),
      humidity: data.map((x) => ({ on: x.on, value: x.humidity })),
      lux: data.map((x) => ({ on: x.on, value: x.lux })),
      soilPf: data.map((x) => ({ on: x.on, value: x.soilPf })),
      tankPf: data.map((x) => ({ on: x.on, value: x.tankPf })),
      temperature: data.map((x) => ({ on: x.on, value: x.temperature })),
    };
  };

  const renderProfile = (data?: PlantProfile) => {
    if (data) {
      return <PlantProfileDisplay profile={data} />;
    }
    return <NoProfileCard />;
  };

  const renderMeasurement = (measurements: MeasurementSeries) => (
    <>
      <AreaGraph header="Soil capacitive" label="pF" data={measurements.soilPf} />
      <AreaGraph header="Relative humidity" label="%" data={measurements.humidity} />
      <AreaGraph header="Temperature" label="Celsius" data={measurements.temperature} />
      <AreaGraph header="Lux" label="lx" data={measurements.lux} />
      <AreaGraph header="Watertank capacitive" label="pF" data={measurements.tankPf} />
      <AreaGraph header="Battery voltage" label="V" data={measurements.batteryVoltage} />
    </>
  );

  const renderData = (stationDetails: StationDetails) => {
    const station = stationDetails.station;
    const stationId = station.id;
    const measurements = splitMeasurements(stationDetails.measurements);

    return (
      <>
        <div className="bg-white shadow sm:rounded-lg">
          <div className="px-4 sm:px-6 lg:mx-auto lg:px-8">
            <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200">
              <div className="min-w-0 flex-1">
                <div className="flex items-center">
                  <img className="hidden h-16 w-16 rounded-full sm:block" src={avatarUrl(stationId)} alt="" />
                  <div>
                    <div className="flex items-center">
                      <img className="h-16 w-16 rounded-full sm:hidden" src={avatarUrl(stationId)} alt="" />
                      <div className="pl-7">
                        <h1 className="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:leading-9">{station.name}</h1>
                        <p>
                          <PlantLocation location={station.location ?? ""} />
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div className="mt-6 flex space-x-3 md:ml-4 md:mt-0">
                <Link
                  to={`/plants/${station.id}/edit`}
                  className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2"
                >
                  Settings
                </Link>
                <Link
                  to={`/plants/${station.id}/avatar`}
                  className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2"
                >
                  Upload image
                </Link>
              </div>
            </div>
          </div>
        </div>

        <main>
          <div className="mx-auto max-w-7xl py-4">
            <div className="mx-auto grid max-w-2xl grid-cols-1 grid-rows-1 items-start gap-x-8 lg:mx-0 lg:max-w-none lg:grid-cols-3">
              <div className="sm:mx-0 lg:col-span-2 lg:row-span-2 lg:row-end-2">
                {measurements ? renderMeasurement(measurements) : <NoMeasurements />}
              </div>
              <div className="space-y-8 lg:col-start-3">
                <div>
                  <h2 className="mb-5 text-sm font-semibold leading-6 text-gray-900">Profile</h2>
                  <Retrieve dataKey={`plant/${stationId}/profile`} retriever={getStationProfile(stationId)} renderData={renderProfile} />
                </div>
                <div>
                  <h2 className="mb-5 text-sm font-semibold leading-6 text-gray-900">Activity</h2>
                  <PlantLog plantId={stationId} />
                </div>
              </div>
            </div>
          </div>
        </main>
      </>
    );
  };

  return <Retrieve dataKey={`plant/${plantId}/details`} retriever={getStationDetails(plantId ?? "")} renderData={renderData} />;
};
