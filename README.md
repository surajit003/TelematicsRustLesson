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

Apply database migrations:

```sh
DATABASE_URL=postgres://telematics:telematics@localhost:5455/telematics \
  cargo run --manifest-path migration/Cargo.toml -- up
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

## Vehicle API

Vehicles are keyed by `number_plate`. The create and update request body is:

```json
{
  "number_plate": "KDA123X",
  "make": "Toyota",
  "model": "Hilux",
  "year": 2024
}
```

Available endpoints:

| Method | Path | Description |
| --- | --- | --- |
| `POST` | `/vehicles` | Create a vehicle and return the created plate |
| `GET` | `/vehicles` | List all vehicles |
| `GET` | `/vehicles/{plate}` | Get one vehicle by plate |
| `PUT` | `/vehicles/{plate}` | Update a vehicle by plate |
| `DELETE` | `/vehicles/{plate}` | Delete a vehicle by plate |

Create a vehicle:

```sh
curl -X POST http://localhost:3000/vehicles \
  -H "Content-Type: application/json" \
  -d '{
    "number_plate": "KDA123X",
    "make": "Toyota",
    "model": "Hilux",
    "year": 2024
  }'
```

List vehicles:

```sh
curl http://localhost:3000/vehicles
```

Get a vehicle:

```sh
curl http://localhost:3000/vehicles/KDA123X
```

Update a vehicle:

```sh
curl -X PUT http://localhost:3000/vehicles/KDA123X \
  -H "Content-Type: application/json" \
  -d '{
    "number_plate": "KDA123X",
    "make": "Toyota",
    "model": "Land Cruiser",
    "year": 2025
  }'
```

Delete a vehicle:

```sh
curl -X DELETE http://localhost:3000/vehicles/KDA123X
```

## Tracker Ping API

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
