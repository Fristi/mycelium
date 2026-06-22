# Soil sensor protocol (0x55)

Implemented in [`mycelium-plant-sim.chip.c`](../plant-pot/mycelium-plant-sim.chip.c) on the I2C1 bus.

| MCU write | Behavior |
|-----------|----------|
| `[0x11, 0x01]` | Start conversion |
| `[0x12]` | Arm 3-byte read |
| MCU read 3 bytes | `soil_pf = d0 + d1*256 + d2/256` |

Matches [`src/moisture.rs`](../../src/moisture.rs).
