use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tracker_pings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub number_plate: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub recorded_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}