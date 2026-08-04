use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_pings_vehicle")
                    .from(TrackerPings::Table, TrackerPings::NumberPlate)
                    .to(Vehicles::Table, Vehicles::NumberPlate)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_pings_vehicle")
                    .table(TrackerPings::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TrackerPings {
    Table,
    NumberPlate,
}

#[derive(DeriveIden)]
enum Vehicles {
    Table,
    NumberPlate,
}
