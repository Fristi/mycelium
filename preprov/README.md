# Preprov

Seeds the local development database with sample stations, plant profiles, ~2 months of hourly measurements, and watering events.

## What it inserts

- 2 stations owned by `PREPROV_USER_ID`
- A Schefflera plant profile per station (for growth-period classification)
- Hourly measurements for the last 62 days
- Watering events every 6–11 days

The script is idempotent: if the first seed station MAC already exists, it exits without changes.

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `PG_HOST` | `localhost` | PostgreSQL host |
| `PG_PORT` | `5432` | PostgreSQL port |
| `PG_USER` | `postgres` | Database user |
| `PG_PASS` | `postgres` | Database password |
| `PG_DB` | `mycelium` | Database name |
| `PREPROV_USER_ID` | `local-dev-user` | Auth0 `sub` that owns the seed stations |

Set `PREPROV_USER_ID` to your Auth0 user id so the seeded plants show up after login.

## Run via docker compose

From the repo root (loads `backend/.env` automatically; shell env overrides):

```bash
PREPROV_USER_ID='auth0|your-sub' just backend-dev
```

Or put `PREPROV_USER_ID` in `backend/.env` and run:

```bash
just backend-dev
```

Or build the image and start services separately:

```bash
just backend-build
PREPROV_USER_ID='auth0|your-sub' just backend-compose-up
```

Preprov runs once after the backend starts and migrations complete.

## Run locally

```bash
pip install -r requirements.txt
PG_HOST=localhost PREPROV_USER_ID='auth0|your-sub' python seed.py
```
