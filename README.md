# TelematicsRustLesson

A Rust telematics API built with Axum, SeaORM, Tokio, and Postgres.

## Requirements

- Rust 1.85 or newer
- Docker and Docker Compose

## Setup

Start Postgres:

```sh
docker compose up -d
```

Create a local environment file:

```sh
cp .env.example .env
```

Run the API:

```sh
cargo run
```

The server listens on `http://localhost:3000`.

## API

Health check:

```sh
curl http://localhost:3000/
```

Create a tracker ping:

```sh
curl -X POST http://localhost:3000/pings \
  -H "Content-Type: application/json" \
  -d '{
    "number_plate": "KDA123X",
    "latitude": -1.286389,
    "longitude": 36.817223,
    "speed": 42.5,
    "recorded_at": "2026-08-03T09:00:00+03:00"
  }'
```

List pings for a vehicle:

```sh
curl "http://localhost:3000/pings?plate=KDA123X&page=1"
```
