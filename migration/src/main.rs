mod entities;

use sea_orm::{Database, ConnectionTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env so DATABASE_URL is available
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")?;

    // Connect to Postgres
    let db = Database::connect(&db_url).await?;
    println!("Connected to the database!");

    // Sync: make the database schema match our entity definitions
    db.get_schema_registry("telematics::entities::*")
        .sync(&db)
        .await?;
    println!("Schema synced — table should exist now.");

    Ok(())
}