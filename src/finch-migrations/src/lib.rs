pub use sea_schema::migration::prelude::*;

mod m20220101_000001_create_team_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_team_table::Migration)]
    }
}
