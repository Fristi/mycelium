import * as z from "zod";

export const AttributeSchema = z.object({
  name: z.string({ required_error: "Name is required" }).trim().min(1, "Name cannot be empty"),
  description: z.string({ required_error: "Description is required" }).trim().min(1, "Name cannot be empty"),
  location: z.string({ required_error: "Location is required" }).trim().min(1, "Name cannot be empty"),
});

export const WateringScheduleSchema = z.discriminatedUnion("_type", [
  z.object({ _type: z.literal("Interval"), schedule: z.string(), period: z.string() }),
  z.object({ _type: z.literal("Threshold"), belowSoilPf: z.number(), period: z.string() }),
]);

export const WifiCredentialsSchema = z.object({
  ssid: z.string({ required_error: "SSID is required" }),
  password: z.string({ required_error: "Password is required" }),
});

export const AddPlantSchema = z.object({
  name: z.string({ required_error: "Name is required" }).trim().min(1, "Name cannot be empty"),
  description: z.string({ required_error: "Description is required" }).trim().min(1, "Name cannot be empty"),
  location: z.string({ required_error: "Location is required" }).trim().min(1, "Name cannot be empty"),
  wifi_ssid: z.string({ required_error: "SSID is required" }).trim().min(1, "Name cannot be empty"),
  wifi_password: z.string({ required_error: "Password is required" }).trim().min(1, "Name cannot be empty"),
}); 
