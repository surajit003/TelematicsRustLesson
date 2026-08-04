use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewPing {
    pub number_plate: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub recorded_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Debug, Serialize)]
pub struct PingSaved {
    pub id: i64,
}

// Query parameters for the list endpoint: ?plate=...&page=...
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub plate: String,
    #[serde(default = "default_page")]
    pub page: u64,
}

fn default_page() -> u64 {
    1
}

// One ping in the response list.
#[derive(Debug, Serialize)]
pub struct PingOut {
    pub id: i64,
    pub number_plate: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub recorded_at: chrono::DateTime<chrono::FixedOffset>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

// The paginated response wrapper.
#[derive(Debug, Serialize)]
pub struct PingList {
    pub page: u64,
    pub total_pages: u64,
    pub items: Vec<PingOut>,
}

// Incoming: what the client sends to create or update a vehicle
#[derive(Debug, Deserialize)]
pub struct NewVehicle {
    pub number_plate: String,
    pub make: String,
    pub model: String,
    pub year: i32,
}

// Outgoing: what we send back in responses
#[derive(Debug, Serialize)]
pub struct VehicleOut {
    pub number_plate: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
