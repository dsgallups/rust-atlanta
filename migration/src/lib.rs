#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251028_091506_projects;
mod m20251028_091758_events;
mod m20251028_092024_news;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251028_091506_projects::Migration),
            Box::new(m20251028_091758_events::Migration),
            Box::new(m20251028_092024_news::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}