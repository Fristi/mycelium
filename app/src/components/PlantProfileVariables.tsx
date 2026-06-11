import { Leaf } from "lucide-react";
import { PlantProfileVariables } from "../backend-client/api";

export interface PlantProfileVariablesDisplayProps {
  variables: PlantProfileVariables;
}

export const PlantProfileVariablesDisplay: React.FC<PlantProfileVariablesDisplayProps> = ({ variables }) => {
  return (
    <div className="relative mt-5">
      <Leaf className="pointer-events-none absolute bottom-4 right-4 h-24 w-24 text-green-100 opacity-70" />
      <div className="flex flex-wrap gap-3">
        <div className="flex items-center gap-1">
          🌞 <span>Light (µmol): {variables.lightMmol.start}–{variables.lightMmol.end}</span>
        </div>
        <div className="flex items-center gap-1">
          💡 <span>Light (Lux): {variables.lightLux.start}–{variables.lightLux.end}</span>
        </div>
        <div className="flex items-center gap-1">
          🌡️ <span>Temperature: {variables.temperature.start}–{variables.temperature.end}°C</span>
        </div>
        <div className="flex items-center gap-1">
          💧 <span>Humidity: {variables.humidity.start}–{variables.humidity.end}%</span>
        </div>
        <div className="flex items-center gap-1">
          🌱 <span>Soil Moisture: {variables.soilMoisture.start}–{variables.soilMoisture.end}%</span>
        </div>
        <div className="flex items-center gap-1">
          ⚡ <span>Soil EC: {variables.soilEc.start}–{variables.soilEc.end} mS/cm</span>
        </div>
      </div>
    </div>
  );
};
