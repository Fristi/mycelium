import React, { useState } from "react";
import { PlantProfile } from "@backendclient/api";
import { useParams } from "react-router";
import { createRetriever } from "../api";
import { useAuth } from "../AuthContext";
import { useNavigate } from "react-router-dom";
import { PlantProfileDisplay } from "../components/PlantProfile";
import { CheckIcon } from "@heroicons/react/24/solid";

enum Step {
  UploadImage,
  SelectProfile,
  Confirm,
}

const PlantAvatarUpload: React.FC = () => {
  const [step, setStep] = useState<Step>(Step.UploadImage);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [profiles, setProfiles] = useState<PlantProfile[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();
  const [selectedProfile, setSelectedProfile] = useState<PlantProfile | null>(null);

  const { plantId } = useParams();
  const auth = useAuth();

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setSelectedFile(e.target.files[0]);
      setProfiles([]);
      setError(null);
    }
  };

  const handleUpload = async () => {
    if (!selectedFile) return;

    setLoading(true);
    setError(null);

    try {
      const response = await createRetriever((api) => api.uploadAvatar(plantId ?? "", selectedFile))(auth.token ?? "");
      setProfiles(response.data);
      setStep(Step.SelectProfile);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "Something went wrong");
    } finally {
      setLoading(false);
    }
  };

  const handleProfileSelect = (profile: PlantProfile) => {
    setSelectedProfile(profile);
    setStep(Step.Confirm);
  };

  const handleConfirm = () => {
    if (selectedProfile) {
      createRetriever((api) => api.setProfile(plantId ?? "", selectedProfile))(auth.token ?? "").then(() => navigate(`/plants/${plantId}`));
    }
  };

  const steps = [
    { id: "01", name: "Upload Image", status: step === Step.UploadImage ? "current" : step > Step.UploadImage ? "complete" : "upcoming" },
    { id: "02", name: "Select Profile", status: step === Step.SelectProfile ? "current" : step > Step.SelectProfile ? "complete" : "upcoming" },
    { id: "03", name: "Confirm", status: step === Step.Confirm ? "current" : "upcoming" },
  ];

  return (
    <div className="mx-auto rounded-md border bg-white p-4 shadow-md">
      <nav aria-label="Progress" className="mb-6">
        <ol role="list" className="divide-y divide-gray-300 rounded-md border border-gray-300 md:flex md:divide-y-0">
          {steps.map((s, idx) => (
            <li key={s.name} className="relative md:flex md:flex-1">
              {s.status === "complete" ? (
                <span className="flex items-center px-6 py-4 text-sm font-medium">
                  <span className="flex h-10 w-10 items-center justify-center rounded-full bg-green-600">
                    <CheckIcon className="h-6 w-6 text-white" />
                  </span>
                  <span className="ml-4 text-sm font-medium text-gray-900">{s.name}</span>
                </span>
              ) : s.status === "current" ? (
                <span className="flex items-center px-6 py-4 text-sm font-medium">
                  <span className="flex h-10 w-10 items-center justify-center rounded-full border-2 border-green-600">
                    <span className="text-green-600">{s.id}</span>
                  </span>
                  <span className="ml-4 text-sm font-medium text-green-600">{s.name}</span>
                </span>
              ) : (
                <span className="flex items-center px-6 py-4 text-sm font-medium">
                  <span className="flex h-10 w-10 items-center justify-center rounded-full border-2 border-gray-300">
                    <span className="text-gray-500">{s.id}</span>
                  </span>
                  <span className="ml-4 text-sm font-medium text-gray-500">{s.name}</span>
                </span>
              )}
              {idx !== steps.length - 1 && (
                <div aria-hidden="true" className="absolute right-0 top-0 hidden h-full w-5 md:block">
                  <svg fill="none" viewBox="0 0 22 80" preserveAspectRatio="none" className="h-full w-full text-gray-300">
                    <path d="M0 -2L20 40L0 82" stroke="currentColor" vectorEffect="non-scaling-stroke" strokeLinejoin="round" />
                  </svg>
                </div>
              )}
            </li>
          ))}
        </ol>
      </nav>

      {step === Step.UploadImage && (
        <>
          <h2 className="mb-4 text-lg font-semibold">Upload an Image</h2>
          <input type="file" accept="image/*" onChange={handleFileChange} className="mb-4 w-full rounded border p-2 text-sm text-gray-700" />
          {selectedFile && (
            <div className="mb-4">
              <p>Selected file: {selectedFile.name}</p>
              <button
                onClick={handleUpload}
                disabled={loading}
                className="mt-2 rounded bg-green-500 px-4 py-2 text-white hover:bg-green-600 disabled:opacity-50"
              >
                {loading ? "Uploading..." : "Upload"}
              </button>
            </div>
          )}
          {error && <p className="mb-4 text-red-500">{error}</p>}
        </>
      )}

      {step === Step.SelectProfile && (
        <>
          <h2 className="mb-4 text-lg font-semibold">Select a Profile</h2>
          <button onClick={() => setStep(Step.UploadImage)} className="mb-4 rounded border px-4 py-2 hover:bg-gray-100">
            Back
          </button>
          <ul className="grid grid-cols-1 gap-4 md:grid-cols-3">
            {profiles.map((profile) => (
              <li key={profile.name}>
                <button onClick={() => handleProfileSelect(profile)} className="flex w-full flex-col gap-2 p-4 text-left">
                  <PlantProfileDisplay profile={profile} />
                </button>
              </li>
            ))}
          </ul>
        </>
      )}

      {step === Step.Confirm && selectedProfile && (
        <>
          <h2 className="mb-4 text-lg font-semibold">Confirm Selection</h2>
          <PlantProfileDisplay profile={selectedProfile} />
          <div className="flex gap-2">
            <button onClick={handleConfirm} className="rounded bg-green-500 px-4 py-2 text-white hover:bg-green-600">
              Confirm
            </button>
            <button onClick={() => setStep(Step.SelectProfile)} className="rounded border px-4 py-2 hover:bg-gray-100">
              Back
            </button>
          </div>
        </>
      )}
    </div>
  );
};

export default PlantAvatarUpload;
