mod entities;
mod handlers;
mod services;
mod routes;

use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;

    let db = Database::connect(&db_url).await?;
    db.get_schema_registry("telematics::entities::*")
        .sync(&db)
        .await?;
    println!("Database ready.");

    let app = routes::build_router(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}