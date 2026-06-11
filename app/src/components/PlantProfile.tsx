import { PlantProfile } from "@backendclient/api";
import { PlantProfileVariablesDisplay } from "./PlantProfileVariables";

export interface PlantProfileDisplayProps {
  profile: PlantProfile;
}

export const PlantProfileDisplay: React.FC<PlantProfileDisplayProps> = ({ profile }) => {
  return (
    <div className="mx-auto rounded-2xl bg-white p-6 shadow-md">
      <h1 className="truncate text-2xl font-bold text-gray-900 dark:text-white">{profile.name}</h1>
      <img
        className="mt-4 rounded-md outline -outline-offset-1 outline-black/5 dark:outline-white/10"
        src={`https://opb-img.plantbook.io/${encodeURIComponent(profile.name.toLowerCase())}.jpg`}
        alt={profile.name}
      />
      <PlantProfileVariablesDisplay variables={profile.variables} />
    </div>
  );
};
