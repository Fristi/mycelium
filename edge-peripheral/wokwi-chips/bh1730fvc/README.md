# BH1730FVC protocol (0x29)

Implemented in [`mycelium-plant-sim.chip.c`](../plant-pot/mycelium-plant-sim.chip.c) on the I2C0 bus.

- Reset command: `0xE4`
- Register writes: `[0x80 | reg, value]`
- Register reads: write `[0x80 | reg]`, then read byte(s)
- Single-shot mode control value: `0x0B`
- Light data block: read 4 bytes from register `0x14`

Matches `bh1730fvc` crate usage in [`src/gauge.rs`](../../src/gauge.rs).
