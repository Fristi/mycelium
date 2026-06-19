import { useState } from "react";
import { Link, useParams } from "react-router-dom";
import MeasurementEventChart from "../components/MeasurementEventChart";
import {
  PlantProfile,
  StationDetails,
  avatarUrl,
  getStationDetails,
  getStationProfile,
} from "../api";
import Retrieve from "../Retrieve";
import PlantLocation from "../components/PlantLocation";
import { PlantProfileDisplay } from "../components/PlantProfile";
import { Activity, Leaf } from "lucide-react";
import type { MeasurementPeriod } from "../lib/timeline";
import { useQuery } from "react-query";
import { useAuth } from "../AuthContext";

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

export const PlantView = () => {
  const { plantId } = useParams();
  const stationId = plantId ?? "";
  const auth = useAuth();
  const token = auth.token ?? "";
  const [period, setPeriod] = useState<MeasurementPeriod>("last-24-hours");

  const { data: profileData } = useQuery(
    [`plant/${stationId}/profile`, token],
    () => getStationProfile(stationId)(token),
    { enabled: token.length > 0 && stationId.length > 0 }
  );
  const hasProfile = !!profileData?.data;

  const NoProfileCard = () => (
    <div className="relative mx-auto overflow-hidden rounded-2xl bg-white p-6 shadow-md">
      <Leaf className="pointer-events-none absolute bottom-4 right-4 h-24 w-24 text-green-100 opacity-70" />
      <div className="relative z-10 flex flex-col items-center text-center">
        <h2 className="mb-2 text-xl font-semibold text-gray-800">No Profile detected</h2>
        <p className="text-sm text-gray-500">
          You haven't selected a plant profile yet. Upload a picture of your plant and we'll try to classify it.
        </p>
        <Link to={`/plants/${stationId}/avatar`} className="mt-4 rounded-lg bg-green-500 px-4 py-2 text-white transition hover:bg-green-600">
          Upload picture
        </Link>
      </div>
    </div>
  );

  const renderProfile = (data?: PlantProfile) => {
    if (data) {
      return <PlantProfileDisplay profile={data} />;
    }
    return <NoProfileCard />;
  };

  const renderMeasurement = (stationDetails: StationDetails) => {
    const measurements = stationDetails.measurements ?? [];
    if (measurements.length === 0) {
      return <NoMeasurements />;
    }

    return (
      <MeasurementEventChart
        measurements={measurements}
        waterings={stationDetails.waterings ?? []}
        growthPeriods={stationDetails.growthPeriods ?? []}
        period={period}
        hasProfile={hasProfile}
        onPeriodChange={setPeriod}
      />
    );
  };

  const renderData = (stationDetails: StationDetails) => {
    const station = stationDetails.station;

    return (
      <>
        <div className="bg-white shadow sm:rounded-lg">
          <div className="px-4 sm:px-6 lg:mx-auto lg:px-8">
            <div className="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200">
              <div className="min-w-0 flex-1">
                <div className="flex items-center">
                  <img className="hidden h-16 w-16 rounded-full sm:block" src={avatarUrl(station.id)} alt="" />
                  <div>
                    <div className="flex items-center">
                      <img className="h-16 w-16 rounded-full sm:hidden" src={avatarUrl(station.id)} alt="" />
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
                {renderMeasurement(stationDetails)}
              </div>
              <div className="space-y-8 lg:col-start-3">
                <div>
                  <h2 className="mb-5 text-sm font-semibold leading-6 text-gray-900">Profile</h2>
                  <Retrieve dataKey={`plant/${station.id}/profile`} retriever={getStationProfile(station.id)} renderData={renderProfile} />
                </div>
              </div>
            </div>
          </div>
        </main>
      </>
    );
  };

  return (
    <Retrieve
      dataKey={`plant/${plantId}/details/${period}`}
      retriever={getStationDetails(plantId ?? "", period)}
      renderData={renderData}
    />
  );
};
