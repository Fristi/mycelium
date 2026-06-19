import { Configuration, DefaultApi } from "@backendclient/index";
import type {
  GrowthPeriod,
  PlantProfile,
  PlantProfileVariables,
  Station,
  StationDetails,
  StationMeasurement,
  StationPlantProfile,
  StationUpdate,
  StationWatering,
} from "@backendclient/api";
import type { MeasurementPeriod } from "./lib/timeline";

export type {
  GrowthPeriod,
  PlantProfile,
  PlantProfileVariables,
  Station,
  StationDetails,
  StationMeasurement,
  StationUpdate,
  StationWatering,
};

export type WateringScheduleInterval = {
  _type: "Interval";
  schedule: string;
  period: string;
};

export type WateringScheduleThreshold = {
  _type: "Threshold";
  belowSoilPf: number;
  period: string;
};

export type WateringSchedule = WateringScheduleInterval | WateringScheduleThreshold;

function resolveApiBaseUrl(): string {
  const configured = import.meta.env.VITE_API_BASE_URL;
  if (configured) {
    const trimmed = configured.replace(/\/$/, "");
    return trimmed.endsWith("/api") ? trimmed : `${trimmed}/api`;
  }

  if (import.meta.env.DEV) {
    return "http://localhost:8080/api";
  }

  return "https://mycelium.markdejong.org/api";
}

export const apiBaseUrl = resolveApiBaseUrl();

export function avatarUrl(stationId: string): string {
  return `${apiBaseUrl}/stations/${stationId}/avatar`;
}

export function createRetriever<T>(f: (api: DefaultApi) => T): (jwt: string) => T {
  return (jwt) => {
    const config = new Configuration({
      basePath: apiBaseUrl,
      accessToken: () => jwt,
    });
    const api = new DefaultApi(config);
    return f(api);
  };
}

export function getStations() {
  return createRetriever((api) => api.listStations());
}

export function getStationDetails(id: string, period?: MeasurementPeriod) {
  return createRetriever((api) => api.getStation(id, period));
}

export function updateStation(id: string, update: StationUpdate) {
  return createRetriever((api) => api.updateStation(id, update));
}

export function getStationProfile(stationId: string) {
  return (token: string) =>
    createRetriever((api) => api.getProfiles())(token).then((response) => ({
      ...response,
      data: response.data.find((profile: StationPlantProfile) => profile.stationId === stationId)?.profile,
    }));
}
