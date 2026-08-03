use axum::{routing::{get, post}, Router};
use sea_orm::DatabaseConnection;
use crate::handlers::pings;

pub fn build_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(pings::health))
        .route("/pings", post(pings::ingest).get(pings::list))
        .with_state(db)
}