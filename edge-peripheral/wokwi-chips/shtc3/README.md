# SHTC3 protocol (0x70)

Implemented in [`mycelium-plant-sim.chip.c`](../plant-pot/mycelium-plant-sim.chip.c) on the I2C0 bus.

- Normal-mode measurement command: `[0x78, 0x66]`
- Response: 6 bytes (temperature + CRC, humidity + CRC) using Sensirion CRC-8

Matches `shtcx2::shtc3` usage in [`src/gauge.rs`](../../src/gauge.rs).
