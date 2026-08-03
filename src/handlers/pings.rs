use axum::{extract::{State, Query}, Json, http::StatusCode};
use sea_orm::DatabaseConnection;
use crate::handlers::dtos::{NewPing, PingSaved, ListQuery, PingOut, PingList};
use crate::services;

pub async fn health() -> &'static str {
    "Telematics API is running!"
}

pub async fn ingest(
    State(db): State<DatabaseConnection>,
    Json(input): Json<NewPing>,
) -> Result<(StatusCode, Json<PingSaved>), StatusCode> {
    match services::pings::create_ping(&db, input).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(PingSaved { id }))),
        Err(e) => {
            eprintln!("ingest failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// GET /pings?plate=KDA123X&page=1
pub async fn list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ListQuery>,
) -> Result<Json<PingList>, StatusCode> {
    match services::pings::list_pings_by_plate(&db, &params.plate, params.page).await {
        Ok((rows, total_pages)) => {
            let items = rows
                .into_iter()
                .map(|p| PingOut {
                    id: p.id,
                    number_plate: p.number_plate,
                    latitude: p.latitude,
                    longitude: p.longitude,
                    speed: p.speed,
                    recorded_at: p.recorded_at,
                    created_at: p.created_at,
                })
                .collect();

            Ok(Json(PingList {
                page: params.page,
                total_pages,
                items,
            }))
        }
        Err(e) => {
            eprintln!("list failed: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}