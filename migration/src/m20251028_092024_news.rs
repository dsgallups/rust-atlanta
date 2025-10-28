use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "news",
            &[
            
            ("id", ColType::PkAuto),
            
            ("title", ColType::String),
            ("excerpt", ColType::String),
            ("content", ColType::String),
            ("published_at", ColType::TimestampWithTimeZoneNull),
            ("category", ColType::String),
            ("read_time", ColType::Integer),
            ("featured", ColType::Boolean),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "news").await
    }
}
