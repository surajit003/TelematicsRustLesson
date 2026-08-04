use crate::entities::vehicles;
use crate::handlers::dtos::NewVehicle;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

// CREATE — fully written as your reference
pub async fn create_vehicle(
    db: &DatabaseConnection,
    input: NewVehicle,
) -> Result<String, sea_orm::DbErr> {
    let now = chrono::Utc::now().fixed_offset();

    let active = vehicles::ActiveModel {
        number_plate: Set(input.number_plate),
        make: Set(input.make),
        model: Set(input.model),
        year: Set(input.year),
        created_at: Set(now),
    };

    let saved = active.insert(db).await?;
    Ok(saved.number_plate)
}

// READ ONE — fully written as your reference
pub async fn get_vehicle(
    db: &DatabaseConnection,
    plate: &str,
) -> Result<Option<vehicles::Model>, sea_orm::DbErr> {
    let res = vehicles::Entity::find_by_id(plate.to_string())
        .one(db)
        .await?;
    Ok(res)
}

// LIST ALL — your turn
pub async fn list_vehicles(
    db: &DatabaseConnection,
) -> Result<Vec<vehicles::Model>, sea_orm::DbErr> {
    let res = vehicles::Entity::find().all(db).await?;
    Ok(res)
}

// UPDATE — your turn (new operation!)
pub async fn update_vehicle(
    db: &DatabaseConnection,
    plate: &str,
    input: NewVehicle,
) -> Result<Option<vehicles::Model>, sea_orm::DbErr> {
    // 1. find the existing vehicle by id (like get_vehicle)
    let res = vehicles::Entity::find_by_id(plate.to_string())
        .one(db)
        .await?;
    match res {
        Some(model) => {
            let mut active: vehicles::ActiveModel = model.into();
            active.make = Set(input.make);
            active.model = Set(input.model);
            active.year = Set(input.year);
            let updated = active.update(db).await?;
            Ok(Some(updated))
        }
        None => Ok(None),
    }
}

// DELETE — your turn (new operation!)
pub async fn delete_vehicle(db: &DatabaseConnection, plate: &str) -> Result<bool, sea_orm::DbErr> {
    let res = vehicles::Entity::delete_by_id(plate.to_string())
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}
