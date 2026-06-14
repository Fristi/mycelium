# BLE over UART — Task List (Linux sim E2E)

## Phase 1: `sim` firmware with BLE over UART

- [x] **Task 1:** Merge BLE-UART into `sim` feature (`bt-hci`, no `esp-radio`)
- [x] **Task 2:** Implement `hci_uart.rs` — UART1 GPIO16/17 custom `EspUartTransport`
- [x] **Task 3:** Processor sim path — real `ble::run` over UART; remove BLE skip
- [x] **Task 4:** Wokwi — `rfc2217ServerPort` + UART1 HCI serial monitor wiring

### Checkpoint: Peripheral sim BLE bytes
- [x] `sim` build passes
- [ ] H4 packets visible through HCI serial path (manual Wokwi verify)

---

## Phase 2: Linux host bridge

- [x] **Task 5:** `edge-hci-bridge` — HCI controller emulator + BlueZ (Linux only)
- [x] **Task 6:** `docs/ble-hci-uart.md` — Linux-only runbook
- [ ] **Task 7:** E2E — edge-central discovers and syncs `Mycelium` on Linux

### Checkpoint: Linux sim E2E
- [ ] Full Wokwi → UART → bridge → BlueZ → edge-central path works
- [x] `hardware` build unaffected

---

## Phase 3: Polish (optional)

- [ ] **Task 8:** CI compile gate for `--features sim`
- [ ] **Task 9:** Tune ATT handle map in bridge after live capture
