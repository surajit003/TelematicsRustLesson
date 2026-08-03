use sea_orm::{
    DatabaseConnection, ActiveModelTrait, Set,
    EntityTrait, QueryFilter, QueryOrder, ColumnTrait, PaginatorTrait,
};
use crate::entities::tracker_pings;
use crate::handlers::dtos::NewPing;

const PAGE_SIZE: u64 = 10;

pub async fn create_ping(
    db: &DatabaseConnection,
    input: NewPing,
) -> Result<i64, sea_orm::DbErr> {
    let now = chrono::Utc::now().fixed_offset();

    let active = tracker_pings::ActiveModel {
        number_plate: Set(input.number_plate),
        latitude: Set(input.latitude),
        longitude: Set(input.longitude),
        speed: Set(input.speed),
        recorded_at: Set(input.recorded_at),
        created_at: Set(now),
        ..Default::default()
    };

    let saved = active.insert(db).await?;
    Ok(saved.id)
}

// List pings for a plate, newest first, paginated.
// Returns (rows_for_this_page, total_pages).
pub async fn list_pings_by_plate(
    db: &DatabaseConnection,
    plate: &str,
    page: u64,
) -> Result<(Vec<tracker_pings::Model>, u64), sea_orm::DbErr> {
    let paginator = tracker_pings::Entity::find()
        .filter(tracker_pings::Column::NumberPlate.eq(plate))
        .order_by_desc(tracker_pings::Column::RecordedAt)
        .paginate(db, PAGE_SIZE);

    let total_pages = paginator.num_pages().await?;
    let rows = paginator.fetch_page(page.saturating_sub(1)).await?;

    Ok((rows, total_pages))
}