# Plant pot simulation chip

Unified Wokwi custom chip that simulates:

- **Plant physics** — soil moisture charges on "Water Plant" and discharges over time
- **BH1730FVC** @ `0x29` on I2C0 (GPIO21/22)
- **SHTC3** @ `0x70` on I2C0
- **Soil sensor** @ `0x55` on I2C1 (GPIO27/26)

Canonical sources:

- [`mycelium-plant-sim.chip.c`](mycelium-plant-sim.chip.c)
- [`mycelium-plant-sim.chip.json`](mycelium-plant-sim.chip.json)

Copies also exist at the `edge-peripheral/` project root for Wokwi discovery.

## Controls

| Attribute | Default | Description |
|-----------|---------|-------------|
| `soil_pf_dry` | 45 | Dry baseline (pF) |
| `soil_pf_wet` | 120 | Saturated ceiling (pF) |
| `evap_rate` | 2 | Base evaporation (pF/s) |
| `water_amount` | 25 | pF added per water button press |
| `ambient_temp` | 22 | SHTC3 temperature (°C) |
| `ambient_humidity` | 55 | SHTC3 humidity (%RH) |
| `ambient_lux` | 800 | BH1730 lux |

Evaporation scales with lux, temperature, and inversely with humidity.
