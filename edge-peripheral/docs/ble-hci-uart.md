# BLE over UART — Wokwi + Lima VM E2E

Run the Wokwi `edge-peripheral` simulator with BLE HCI forwarded into a Lima Linux VM through `/dev/vhci`, then scan from BlueZ tools.

## Architecture

```
Wokwi ESP (sim firmware, trouble-host + GATT)
  UART1 GPIO16/17 → HCI H4
       ↓ Wokwi RFC2217 :4000 (host side)
       ↓ socat PTY link (/tmp/mycelium-hci)
edge-hci-bridge.py (host side, forwards H4 to /dev/vhci)
       ↓ Linux kernel vhci (inside Lima VM)
       ↓ BlueZ tools (bluetoothctl, btmon, hcitool)
```

## Prerequisites

- Wokwi VS Code extension for simulation
- A running Lima VM with BlueZ tooling available
- `socat` on the host
- Python bridge dependencies installed (`/opt/bumble-venv/bin/python` used below)
- Access to `/dev/vhci` on the host (`sudo chmod 666 /dev/vhci`)

## Run

### 1. Start Wokwi sim

Open `edge-peripheral/` in VS Code and start the Wokwi simulator (`just edge-peripheral-sim` or F1 → Wokwi: Start Simulator).

UART1 (GPIO16/17) is wired to `$serialMonitor`, so Wokwi RFC2217 port `4000` carries binary HCI.
Because UART1 is used for HCI transport, you should not expect normal text logs on that UART.

### 2. Start Lima VM

Start your Linux VM and ensure BlueZ is available there before wiring transport.

### 3. Bridge Wokwi RFC2217 to a PTY

Create a persistent PTY endpoint that other tools can open:

```bash
socat -d -d -x pty,link=/tmp/mycelium-hci,raw,echo=0 tcp:192.168.5.2:4000
```

This links the VM-reachable Wokwi TCP endpoint to `/tmp/mycelium-hci`.

### 4. Allow vhci access

```bash
sudo chmod 666 /dev/vhci
```

### 5. Start Python HCI bridge

Run from project root:

```bash
/opt/bumble-venv/bin/python edge-hci-bridge.py --pty /tmp/mycelium-hci --baud 115200 --verbose
```

The bridge:

- Answers HCI commands from the ESP host over UART
- Creates a virtual HCI path via `/dev/vhci`
- Exposes activity to Linux BLE tooling in the VM

### 6. Scan in VM with BlueZ

In the VM:

```bash
bluetoothctl scan on
```

You should see:

`SetDiscoveryFilter success`

## UART pin map

| UART | Pins | Purpose |
|------|------|---------|
| UART0 | esp:TX / esp:RX | Not exported in current Wokwi wiring |
| UART1 | GPIO17 TX / GPIO16 RX | HCI H4 (115200 baud) via `$serialMonitor` and RFC2217 `:4000` |

## Troubleshooting

- **No Wokwi logs**: Expected while UART1 is dedicated to HCI transport.
- **PTY does not appear**: Ensure `socat` is running and `/tmp/mycelium-hci` exists before starting the Python bridge.
- **`/dev/vhci` permission error**: Run `sudo chmod 666 /dev/vhci` again after reboot.
- **`scan on` succeeds but no devices found**:
  1. Keep `edge-hci-bridge.py --verbose` running and check that HCI events are flowing.
  2. Verify Wokwi is actually reachable on `192.168.5.2:4000` from the host where `socat` runs.
  3. Confirm the firmware enters an advertising state; if needed restart the simulator to reset stale runtime state.
  4. Use `btmon` in the VM to verify LE advertising reports are received at the controller level.
  5. If `btmon` shows no LE reports, the transport path is likely broken before BlueZ (Wokwi RFC2217, `socat`, or bridge configuration).

## Platform note

This workflow is host+VM specific: Wokwi and bridge run on host, BlueZ scan runs in Lima VM.
