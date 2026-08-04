use crate::handlers::dtos::{NewVehicle, VehicleOut};
use crate::services;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DatabaseConnection;

// Small helper to convert a Model into a VehicleOut (avoids repeating the mapping)
fn to_out(v: crate::entities::vehicles::Model) -> VehicleOut {
    VehicleOut {
        number_plate: v.number_plate,
        make: v.make,
        model: v.model,
        year: v.year,
        created_at: v.created_at,
    }
}

// CREATE — POST /vehicles
pub async fn create(
    State(db): State<DatabaseConnection>,
    Json(input): Json<NewVehicle>,
) -> Result<(StatusCode, Json<String>), StatusCode> {
    match services::vehicles::create_vehicle(&db, input).await {
        Ok(plate) => Ok((StatusCode::CREATED, Json(plate))),
        Err(e) => {
            eprintln!("create vehicle failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// READ ONE — GET /vehicles/{plate}
pub async fn get(
    State(db): State<DatabaseConnection>,
    Path(plate): Path<String>,
) -> Result<Json<VehicleOut>, StatusCode> {
    match services::vehicles::get_vehicle(&db, &plate).await {
        Ok(Some(v)) => Ok(Json(to_out(v))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("get vehicle failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// LIST ALL — GET /vehicles
pub async fn list(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<VehicleOut>>, StatusCode> {
    match services::vehicles::list_vehicles(&db).await {
        Ok(vehicles) => {
            let items = vehicles.into_iter().map(to_out).collect();
            Ok(Json(items))
        }
        Err(e) => {
            eprintln!("list vehicles failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// UPDATE — PUT /vehicles/{plate} — your turn
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(plate): Path<String>,
    Json(input): Json<NewVehicle>,
) -> Result<Json<VehicleOut>, StatusCode> {
    match services::vehicles::update_vehicle(&db, &plate, input).await {
        Ok(Some(v)) => Ok(Json(to_out(v))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("get vehicle failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// DELETE — DELETE /vehicles/{plate} — your turn
pub async fn delete(
    State(db): State<DatabaseConnection>,
    Path(plate): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match services::vehicles::delete_vehicle(&db, &plate).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("get vehicle failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
