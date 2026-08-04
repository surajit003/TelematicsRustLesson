use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vehicles::Table)
                    .if_not_exists()
                    .col(string(Vehicles::NumberPlate).primary_key())
                    .col(string(Vehicles::Make))
                    .col(string(Vehicles::Model))
                    .col(integer(Vehicles::Year))
                    .col(timestamp_with_time_zone(Vehicles::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vehicles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Vehicles {
    Table,
    NumberPlate,
    Make,
    Model,
    Year,
    CreatedAt,
}
