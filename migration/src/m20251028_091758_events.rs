use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "events",
            &[
            
            ("id", ColType::PkAuto),
            
            ("title", ColType::String),
            ("description", ColType::String),
            ("date", ColType::TimestampWithTimeZone),
            ("location", ColType::String),
            ("event_type", ColType::String),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "events").await
    }
}
