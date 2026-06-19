#!/usr/bin/env python3
"""Seed local development data: stations, profiles, measurements, and waterings."""

from __future__ import annotations

import math
import os
import random
import sys
import time
from datetime import datetime, timedelta, timezone

import psycopg2
from psycopg2.extras import execute_batch

PG_HOST = os.environ.get("PG_HOST", "localhost")
PG_PORT = int(os.environ.get("PG_PORT", "5432"))
PG_USER = os.environ.get("PG_USER", "postgres")
PG_PASS = os.environ.get("PG_PASS", "postgres")
PG_DB = os.environ.get("PG_DB", "mycelium")
# Auth0 `sub` claim — plain string, e.g. auth0|abc123
PREPROV_USER_ID: str = os.environ.get("PREPROV_USER_ID", "local-dev-user")

MEASUREMENT_INTERVAL = timedelta(hours=1)
MEASUREMENT_WINDOW = timedelta(minutes=5)
HISTORY = timedelta(days=62)

STATIONS = [
    {
        "id": "aaaaaaaa-bbbb-cccc-dddd-000000000001",
        "mac": "aa:bb:cc:00:00:01",
        "name": "Living room ficus",
        "location": "Living room, south window",
        "description": "Seed station for local development",
    },
    {
        "id": "aaaaaaaa-bbbb-cccc-dddd-000000000002",
        "mac": "aa:bb:cc:00:00:02",
        "name": "Kitchen herbs",
        "location": "Kitchen windowsill",
        "description": "Seed station for local development",
    },
]

PROFILE = {
    "name": "Schefflera arboricola",
    "light_mmol_start": 2000,
    "light_mmol_end": 4000,
    "light_lux_start": 3700,
    "light_lux_end": 20000,
    "temperature_start": 10,
    "temperature_end": 32,
    "humidity_start": 30,
    "humidity_end": 80,
    "soil_moisture_start": 15,
    "soil_moisture_end": 60,
    "soil_ec_start": 350,
    "soil_ec_end": 2000,
}


def connect():
    return psycopg2.connect(
        host=PG_HOST,
        port=PG_PORT,
        user=PG_USER,
        password=PG_PASS,
        dbname=PG_DB,
    )


def wait_for_database(max_attempts: int = 60) -> None:
    for attempt in range(1, max_attempts + 1):
        try:
            with connect() as conn:
                with conn.cursor() as cur:
                    cur.execute(
                        """
                        SELECT 1
                        FROM information_schema.tables
                        WHERE table_schema = 'public'
                          AND table_name = 'station_measurements'
                        """
                    )
                    if cur.fetchone():
                        print("Database schema is ready")
                        return
        except psycopg2.Error as exc:
            print(f"Waiting for database ({attempt}/{max_attempts}): {exc}")

        time.sleep(2)

    raise RuntimeError("Timed out waiting for database migrations")


def already_seeded(cur) -> bool:
    cur.execute(
        "SELECT 1 FROM stations WHERE mac_addr = %s LIMIT 1",
        (STATIONS[0]["mac"],),
    )
    return cur.fetchone() is not None


def insert_station(cur, station: dict, created: datetime) -> None:
    cur.execute(
        """
        INSERT INTO stations (id, mac_addr, name, location, description, user_id, created)
        VALUES (%s::uuid, %s, %s, %s, %s, %s, %s)
        ON CONFLICT ON CONSTRAINT unique_mac DO NOTHING
        """,
        (
            str(station["id"]),
            str(station["mac"]),
            str(station["name"]),
            str(station["location"]),
            str(station["description"]),
            str(PREPROV_USER_ID),
            created,
        ),
    )


def insert_profile(cur, station_id: str) -> None:
    cur.execute(
        """
        INSERT INTO station_profile (
            station_id, name,
            light_mmol_start, light_mmol_end,
            light_lux_start, light_lux_end,
            temperature_start, temperature_end,
            humidity_start, humidity_end,
            soil_moisture_start, soil_moisture_end,
            soil_ec_start, soil_ec_end
        ) VALUES (
            %s::uuid, %s,
            %s, %s,
            %s, %s,
            %s, %s,
            %s, %s,
            %s, %s,
            %s, %s
        )
        ON CONFLICT (station_id) DO NOTHING
        """,
        (
            str(station_id),
            PROFILE["name"],
            PROFILE["light_mmol_start"],
            PROFILE["light_mmol_end"],
            PROFILE["light_lux_start"],
            PROFILE["light_lux_end"],
            PROFILE["temperature_start"],
            PROFILE["temperature_end"],
            PROFILE["humidity_start"],
            PROFILE["humidity_end"],
            PROFILE["soil_moisture_start"],
            PROFILE["soil_moisture_end"],
            PROFILE["soil_ec_start"],
            PROFILE["soil_ec_end"],
        ),
    )


def lux_for_hour(hour: int, rng: random.Random) -> float:
    if hour < 6 or hour >= 21:
        return rng.uniform(0, 40)
    daylight = math.sin((hour - 6) / 15 * math.pi)
    return max(0, daylight * rng.uniform(2500, 9000))


def temperature_for_hour(hour: int, rng: random.Random) -> float:
    base = 21.0
    swing = 4.0 * math.sin((hour - 9) / 24 * 2 * math.pi)
    return base + swing + rng.uniform(-0.8, 0.8)


def generate_measurements(
    station_id: str,
    start: datetime,
    end: datetime,
    rng: random.Random,
) -> list[tuple]:
    rows: list[tuple] = []
    soil_pf = rng.uniform(3.2, 3.8)
    battery = 95
    current = start
    next_watering = start + timedelta(days=rng.randint(5, 9))
    waterings: list[tuple] = []

    while current <= end:
        hour = current.hour
        lux = lux_for_hour(hour, rng)
        temperature = temperature_for_hour(hour, rng)
        humidity = rng.uniform(48, 68)

        soil_pf += rng.uniform(0.01, 0.04)
        if current >= next_watering:
            soil_pf = rng.uniform(3.0, 3.6)
            waterings.append(
                (
                    str(station_id),
                    current + timedelta(minutes=rng.randint(10, 50)),
                    rng.randint(12_000, 45_000),
                )
            )
            next_watering = current + timedelta(days=rng.randint(6, 11))

        if lux < 500 and hour >= 10 and rng.random() < 0.02:
            temperature += rng.uniform(2, 5)

        if hour == 3 and current.weekday() == 0:
            battery = max(72, battery - rng.randint(1, 3))

        ended = current + MEASUREMENT_WINDOW
        rows.append(
            (
                str(station_id),
                current,
                ended,
                int(battery),
                round(temperature, 2),
                round(humidity, 2),
                round(lux, 2),
                round(soil_pf, 2),
            )
        )
        current += MEASUREMENT_INTERVAL

    return rows, waterings


def main() -> int:
    print(f"Preprov user id: {PREPROV_USER_ID}")
    wait_for_database()

    now = datetime.now(timezone.utc).replace(minute=0, second=0, microsecond=0)
    start = now - HISTORY

    with connect() as conn:
        with conn.cursor() as cur:
            if already_seeded(cur):
                print("Seed data already present, skipping")
                return 0

            created = start
            for station in STATIONS:
                insert_station(cur, station, created)
                insert_profile(cur, station["id"])

            measurement_rows: list[tuple] = []
            watering_rows: list[tuple] = []

            for station in STATIONS:
                rng = random.Random(station["mac"])
                rows, waterings = generate_measurements(station["id"], start, now, rng)
                measurement_rows.extend(rows)
                watering_rows.extend(waterings)

            execute_batch(
                cur,
                """
                INSERT INTO station_measurements (
                    station_id, occurred_on, ended_on, battery, temperature, humidity, lux, soil_pf
                ) VALUES (%s::uuid, %s, %s, %s, %s, %s, %s, %s)
                """,
                measurement_rows,
                page_size=500,
            )

            execute_batch(
                cur,
                """
                INSERT INTO station_waterings (station_id, occurred_at, duration_msec)
                VALUES (%s::uuid, %s, %s)
                """,
                watering_rows,
                page_size=100,
            )

            conn.commit()
            print(
                f"Inserted {len(STATIONS)} stations, "
                f"{len(measurement_rows)} measurements, "
                f"{len(watering_rows)} waterings"
            )

    return 0


if __name__ == "__main__":
    sys.exit(main())
