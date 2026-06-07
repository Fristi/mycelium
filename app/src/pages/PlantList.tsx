import { Link } from "react-router-dom";
import Retrieve from "../Retrieve";
import { getStations } from "../api";
import PlantCard from "../components/PlantCard";
import { Station } from "../api";

export const PlantList = () => {
  const renderData = (stations: Station[]) => {
    return (
      <div className="mx-auto mt-8 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-3">
        {stations.map((s) => (
          <PlantCard key={s.id} station={s} />
        ))}
      </div>
    );
  };

  return (
    <div>
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-gray-900">Your plants</h1>
        <Link
          to="/hub-add"
          className="inline-flex justify-center rounded-md border border-transparent bg-lime-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-lime-700 focus:outline-none focus:ring-2 focus:ring-lime-500 focus:ring-offset-2"
        >
          Add hub
        </Link>
      </div>
      <Retrieve dataKey="stations" retriever={getStations()} renderData={renderData} />
    </div>
  );
};
