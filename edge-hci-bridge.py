#!/usr/bin/env python3
"""
edge-hci-bridge — dual virtual controller version
====================================================

Bridges the ESP32 firmware's HCI *host* (trouble-host, over UART/PTY) to
BlueZ (also an HCI *host*, via vhci) by sandwiching TWO Bumble virtual
Controllers between them, joined by an in-process LocalLink.

This is the correct shape, derived from bumble/examples/run_controller.py:

    ESP32 (HCI HOST, trouble-host)
        │  serial:/tmp/mycelium-hci,115200
        ▼
    Controller "ESP"  ──┐
                         │  LocalLink (in-process, shares the RF/advertising
                         │  state between the two controllers)
    Controller "BLUEZ" ──┘
        │  vhci
        ▼
    BlueZ (HCI HOST) → hciN → edge-central (btleplug)

Why two controllers are needed
-------------------------------
Both the ESP32 (trouble-host) and BlueZ are HCI *hosts* — neither one
speaks the controller side of HCI (they SEND HCI_Reset, LE_Set_Advertising_*,
etc. and expect Command_Complete events back). A single hci_bridge.py
(host-transport <-> controller-transport) cannot connect two hosts directly.

Each Bumble Controller here terminates one host's HCI traffic as a real
(virtual) controller would: it answers HCI_Reset, processes
LE_Set_Advertising_Enable, etc. The LocalLink then propagates the resulting
RF-layer state (advertising packets, connection establishment) between the
two controllers — so when the ESP32 starts advertising via "Controller ESP",
"Controller BLUEZ" sees that advertisement and surfaces it to BlueZ, which
edge-central can then scan and connect to.

Usage
-----
    /opt/bumble-venv/bin/python edge-hci-bridge.py \\
        --pty /tmp/mycelium-hci --baud 115200

Prerequisites
-------------
    sudo modprobe hci_vhci
    sudo chmod 666 /dev/vhci   (or appropriate udev rule)
"""

import argparse
import asyncio
import inspect
import logging

import bumble.logging
from bumble import hci
from bumble.controller import Controller
from bumble.link import LocalLink
from bumble.transport import open_transport

logger = logging.getLogger(__name__)


async def _start_controller_if_needed(controller: Controller) -> None:
    """
    Start Bumble controllers explicitly when the runtime/API requires it.

    Some Bumble versions auto-wire handlers in __init__, others expose an
    explicit start() coroutine/sync method. Calling this defensively avoids
    a "RX HCI_Reset only" stall when command processing is not started.
    """
    start = getattr(controller, "start", None)
    if start is None:
        return

    result = start()
    if inspect.isawaitable(result):
        await result


async def run_bridge(pty_path: str, baud: int) -> None:
    serial_spec = f"serial:{pty_path},{baud}"

    logger.info("Opening ESP32 HCI UART: %s", serial_spec)
    logger.info("Opening Linux VHCI (BlueZ side): vhci")

    async with await open_transport(serial_spec) as esp_transport:
        async with await open_transport("vhci") as bluez_transport:
            link = LocalLink()

            # Controller terminating the ESP32's HCI host traffic.
            # The ESP32 (trouble-host) sends HCI_Reset, LE_Set_Advertising_*,
            # etc. to THIS controller, which answers them.
            controller_esp = Controller(
                "ESP32",
                host_source=esp_transport.source,
                host_sink=esp_transport.sink,
                link=link,
            )

            # Controller exposed to BlueZ via vhci. Shares the LocalLink with
            # controller_esp, so the ESP32's advertising/connections are
            # visible here too.
            controller_bluez = Controller(
                "BLUEZ",
                host_source=bluez_transport.source,
                host_sink=bluez_transport.sink,
                link=link,
            )

            await _start_controller_if_needed(controller_esp)
            await _start_controller_if_needed(controller_bluez)

            logger.info("Bridge running. Press Ctrl+C to stop.")
            logger.info("Check: hciconfig -a   (look for a new hciN, UP RUNNING)")

            # Run until either transport closes or we're cancelled.
            await asyncio.wait(
                [
                    asyncio.ensure_future(esp_transport.source.terminated),
                    asyncio.ensure_future(bluez_transport.source.terminated),
                ],
                return_when=asyncio.FIRST_COMPLETED,
            )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Bridge ESP32 HCI (serial) <-> BlueZ (vhci) via two "
        "Bumble virtual controllers on a shared LocalLink"
    )
    parser.add_argument(
        "--pty", default="/tmp/mycelium-hci",
        help="PTY path created by socat (default: /tmp/mycelium-hci)",
    )
    parser.add_argument(
        "--baud", type=int, default=115_200,
        help="UART baud rate (default: 115200)",
    )
    parser.add_argument(
        "--verbose", "-v", action="store_true",
        help="Enable debug logging (very verbose: full HCI packet dumps)",
    )
    args = parser.parse_args()

    if args.verbose:
        bumble.logging.setup_basic_logging("DEBUG")
    else:
        bumble.logging.setup_basic_logging("INFO")

    try:
        asyncio.run(run_bridge(args.pty, args.baud))
    except KeyboardInterrupt:
        logger.info("Bridge stopped.")


if __name__ == "__main__":
    main()