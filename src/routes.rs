use crate::handlers::{pings, vehicles};
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn build_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(pings::health))
        .route("/pings", post(pings::ingest).get(pings::list))
        .route("/pings/latest", get(pings::get_ping))
        .route("/vehicles", post(vehicles::create).get(vehicles::list))
        .route(
            "/vehicles/{plate}",
            get(vehicles::get)
                .put(vehicles::update)
                .delete(vehicles::delete),
        )
        .with_state(db)
}
