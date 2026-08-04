use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TrackerPings::Table)
                    .if_not_exists()
                    .col(pk_auto(TrackerPings::Id).big_integer())
                    .col(string(TrackerPings::NumberPlate))
                    .col(double(TrackerPings::Latitude))
                    .col(double(TrackerPings::Longitude))
                    .col(double(TrackerPings::Speed))
                    .col(timestamp_with_time_zone(TrackerPings::RecordedAt))
                    .col(timestamp_with_time_zone(TrackerPings::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TrackerPings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TrackerPings {
    Table,
    Id,
    NumberPlate,
    Latitude,
    Longitude,
    Speed,
    RecordedAt,
    CreatedAt,
}
