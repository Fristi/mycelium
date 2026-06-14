# BLE over UART — Linux sim E2E

Run the Wokwi `edge-peripheral` simulator with real BLE GATT sync through a Linux HCI bridge and unchanged `edge-central`.

## Architecture

```
Wokwi ESP (sim firmware, trouble-host + GATT)
  UART0 → serial monitor (logs)
  UART1 GPIO16/17 → HCI H4
       ↓ socat PTY or Wokwi RFC2217
edge-hci-bridge (HCI controller emulator + BlueZ peripheral)
       ↓ BlueZ
edge-central (btleplug, unchanged)
```

## Prerequisites (Linux)

- BlueZ (`bluetoothd` running)
- User in `bluetooth` group (or run bridge with appropriate permissions)
- Wokwi VS Code extension for simulation
- `socat` for PTY bridging

## Build

```bash
# ESP sim firmware (requires ESP Rust toolchain)
just edge-peripheral-sim-build

# Linux HCI bridge
just edge-hci-bridge-build
```

## Run

### 1. Start Wokwi sim

Open `edge-peripheral/` in VS Code and start the Wokwi simulator (`just edge-peripheral-sim` or F1 → Wokwi: Start Simulator).

UART0 (`esp:TX`/`esp:RX`) shows firmware logs. UART1 (GPIO16/17) carries binary HCI.

### 2. Bridge UART1 to a PTY

If Wokwi exposes HCI on a second serial monitor (`$serialMonitor:hci`), attach socat to the Wokwi RFC2217 port for UART1 when available. Until dual RFC2217 is configured, route UART1 through the Wokwi **hci** serial monitor tab and use a TCP-to-PTY bridge.

Default console RFC2217 (UART0 logs only):

```bash
# Port 4000 from edge-peripheral/wokwi.toml — do NOT use for HCI
```

HCI PTY (adjust TCP target to your Wokwi HCI export):

```bash
socat pty,link=/tmp/mycelium-hci,raw,echo=0 tcp:localhost:4001
```

### 3. Start edge-hci-bridge

```bash
just edge-hci-bridge-run
# or:
RUST_LOG=info edge-hci-bridge/target/release/edge-hci-bridge --port /tmp/mycelium-hci --baud 115200
```

The bridge:

- Answers HCI commands from the ESP host over UART
- Advertises `Mycelium` on BlueZ when the firmware enables LE advertising
- Proxies GATT reads/writes to the ESP stack over HCI ACL

### 4. Run edge-central

On the same Linux host:

```bash
just central-run-local
```

`edge-central` scans for service `0xFFF0` and device name `Mycelium`, then performs time sync and measurement flush as on hardware.

## UART pin map

| UART | Pins | Purpose |
|------|------|---------|
| UART0 | esp:TX / esp:RX | Console logs (`$serialMonitor`) |
| UART1 | GPIO17 TX / GPIO16 RX | HCI H4 (115200 baud) |

## Troubleshooting

- **No H4 traffic**: Confirm `diagram.json` wires GPIO16/17 to the HCI serial monitor and firmware is built with `--features sim`.
- **Bridge open fails**: Create the PTY first with socat; ensure `/tmp/mycelium-hci` exists.
- **Advertise permission denied**: Check BlueZ is running and your user can use D-Bus (`bluetooth` group).
- **Central finds device but sync fails**: ATT handle map in the bridge may need tuning for your trouble-host build; check `RUST_LOG=debug` on the bridge.

## Platform note

Linux only. macOS is not supported for the bridge or `edge-central` BLE path in this workflow.
