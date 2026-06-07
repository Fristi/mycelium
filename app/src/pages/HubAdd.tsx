import { FormikProvider, useFormik } from "formik";
import { toFormikValidationSchema } from "zod-formik-adapter";
import { AddHubSchema } from "../schemas";
import * as z from "zod";
import { useQueryClient } from "react-query";
import { useNavigate, useParams } from "react-router-dom";
import InputField from "../components/InputField";
import { PrimaryButton } from "../components/PrimaryButton";
import { useEffect, useState } from "react";
import { CheckCircleIcon, ExclamationCircleIcon, PauseCircleIcon, UserIcon, WifiIcon } from "@heroicons/react/24/outline";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type HubAdd = z.infer<typeof AddHubSchema>;

type OnboardingStateAwaitingSettings = { _type: "AwaitingSettings" };
type OnboardingStateProvisioningWifi = { _type: "ProvisioningWifi" };
type OnboardingStateComplete = { _type: "Complete" };
type OnboardingStateAwaitingAuthorization = { _type: "AwaitingAuthorization"; url: string };
type OnboardingStateFailed = { _type: "Failed"; error: string };

type OnboardingState =
  | OnboardingStateAwaitingSettings
  | OnboardingStateProvisioningWifi
  | OnboardingStateComplete
  | OnboardingStateAwaitingAuthorization
  | OnboardingStateFailed;

type OnboardingStatusPayload = {
  phase: string;
  user_code: string;
  verification_uri_complete: string;
  error: string;
};

type OnboardingDevice = {
  id: string;
  name?: string | null;
  rssi?: number | null;
};

const mapStatusToState = (status: OnboardingStatusPayload): OnboardingState => {
  switch (status.phase) {
    case "ProvisioningWifi":
      return { _type: "ProvisioningWifi" };
    case "AwaitingAuthorization":
      return { _type: "AwaitingAuthorization", url: status.verification_uri_complete };
    case "Complete":
      return { _type: "Complete" };
    case "Failed":
      return { _type: "Failed", error: status.error || "Unknown error" };
    default:
      return { _type: "AwaitingSettings" };
  }
};

type OnboardingStateViewProps = {
  icon: React.ReactNode;
  header: string;
  children: React.ReactNode;
};

const OnboardingStateView: React.FC<OnboardingStateViewProps> = ({ children, icon, header }) => {
  return (
    <div className="text-center">
      {icon}
      <h3 className="mt-2 text-sm font-semibold text-gray-900">{header}</h3>
      <p className="mt-1 text-sm text-gray-500">{children}</p>
    </div>
  );
};

export const HubProvisioning = () => {
  const { deviceId } = useParams();
  const navigate = useNavigate();

  if (deviceId == null) return <p>Invalid device id</p>;

  const [state, setState] = useState<OnboardingState>({ _type: "AwaitingSettings" });
  const [wifi, setWifi] = useState<{ ssid: string; password: string } | null>(null);

  useEffect(() => {
    const stored = sessionStorage.getItem(`hub-onboarding-wifi:${deviceId}`);
    if (stored) {
      setWifi(JSON.parse(stored));
    }
  }, [deviceId]);

  useEffect(() => {
    if (!wifi) return;

    let disposed = false;
    let unlisten: (() => void) | undefined;

    listen<OnboardingStatusPayload>("onboarding-status", (event) => {
      if (!disposed) {
        setState(mapStatusToState(event.payload));
      }
    }).then((fn) => {
      unlisten = fn;
    });

    invoke("provision_hub_device", {
      deviceId,
      ssid: wifi.ssid,
      password: wifi.password,
    }).catch((err) => {
      console.error(err);
      if (!disposed) {
        setState({ _type: "Failed", error: String(err) });
      }
    });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [deviceId, wifi]);

  const handleOnClickFinish = () => {
    sessionStorage.removeItem(`hub-onboarding-wifi:${deviceId}`);
    navigate("/");
  };

  if (state._type == "AwaitingAuthorization") {
    return (
      <OnboardingStateView header="Awaiting authorization" icon={<UserIcon className="mx-auto h-12 w-12 text-gray-400" />}>
        <p className="pb-2">Authorize this hub, then you can add details for peripherals it discovers.</p>
        <PrimaryButton target="_blank" href={state.url} text="Authorize" />
      </OnboardingStateView>
    );
  } else if (state._type == "AwaitingSettings") {
    return (
      <OnboardingStateView header="Connecting to hub" icon={<PauseCircleIcon className="mx-auto h-12 w-12 text-gray-400" />}>
        <p className="pb-2">Provisioning WiFi credentials to the hub...</p>
      </OnboardingStateView>
    );
  } else if (state._type == "ProvisioningWifi") {
    return (
      <OnboardingStateView header="Connecting to WiFi" icon={<WifiIcon className="mx-auto h-12 w-12 text-gray-400" />}>
        <p>The hub is connecting to your WiFi network</p>
      </OnboardingStateView>
    );
  } else if (state._type == "Failed") {
    return (
      <OnboardingStateView header="Hub onboarding failed" icon={<ExclamationCircleIcon className="mx-auto h-12 w-12 text-gray-400" />}>
        <p className="pb-2">
          An error occurred: <i>{state.error}</i>
        </p>
        <PrimaryButton href="/" text="Overview" />
      </OnboardingStateView>
    );
  } else {
    return (
      <OnboardingStateView header="Hub added" icon={<CheckCircleIcon className="mx-auto h-12 w-12 text-gray-400" />}>
        <p className="pb-2">
          The hub is online. It will discover peripherals nearby — you can add names and locations for each one from the
          overview.
        </p>
        <PrimaryButton onClick={handleOnClickFinish} text="Overview" />
      </OnboardingStateView>
    );
  }
};

export const HubAdd = () => {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [scanning, setScanning] = useState(false);

  const form = useFormik({
    enableReinitialize: true,
    initialValues: { wifi_ssid: "", wifi_password: "" },
    validationSchema: toFormikValidationSchema(AddHubSchema),
    onSubmit: async (values: HubAdd) => {
      setSubmitError(null);
      setScanning(true);
      try {
        const devices = await invoke<OnboardingDevice[]>("scan_onboarding_devices");
        if (devices.length === 0) {
          setSubmitError("No Mycelium hub found nearby. Ensure the hub is powered on and in onboarding mode.");
          return;
        }

        const device = devices[0];
        sessionStorage.setItem(
          `hub-onboarding-wifi:${device.id}`,
          JSON.stringify({ ssid: values.wifi_ssid, password: values.wifi_password }),
        );
        queryClient.invalidateQueries("plants");
        navigate(`/hub-add/${device.id}`);
      } catch (err) {
        console.error(err);
        setSubmitError(String(err));
      } finally {
        setScanning(false);
      }
    },
  });

  return (
    <FormikProvider value={form}>
      <form className="space-y-10 divide-y divide-gray-900/10" onSubmit={form.handleSubmit}>
        <div className="mt-5 lg:mt-6 bg-white shadow sm:rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <div>
              <h3 className="text-lg leading-6 font-medium text-gray-900">Add hub</h3>
              <p className="mt-1 max-w-2xl text-sm text-gray-500">
                Connect a Mycelium hub to your WiFi. After onboarding, the hub discovers peripherals — you can configure
                each one separately.
              </p>
            </div>

            <div className="mt-6 space-y-4">
              <InputField
                type="text"
                id="wifi_ssid"
                name="wifi_ssid"
                label="SSID"
                placeholder="My SSID ..."
                value={form.values.wifi_ssid}
                onChange={form.handleChange}
                helperText="SSID is required"
              />

              <InputField
                type="password"
                id="wifi_password"
                name="wifi_password"
                label="Password"
                placeholder="Your WiFi password"
                value={form.values.wifi_password}
                onChange={form.handleChange}
                helperText="Password is required"
              />
            </div>
          </div>
        </div>
        {submitError && <p className="text-sm text-red-600">{submitError}</p>}
        <div className="pt-5">
          <div className="flex justify-end">
            <PrimaryButton text={scanning ? "Scanning..." : "Add hub"} />
          </div>
        </div>
      </form>
    </FormikProvider>
  );
};
