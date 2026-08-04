pub use sea_orm_migration::prelude::*;

mod m20260804_070340_create_tracker_pings;
mod m20260804_070348_create_vehicles;
mod m20260804_070354_add_vehicle_fk;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260804_070340_create_tracker_pings::Migration),
            Box::new(m20260804_070348_create_vehicles::Migration),
            Box::new(m20260804_070354_add_vehicle_fk::Migration),
        ]
    }
}
