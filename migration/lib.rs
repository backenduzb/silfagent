mod mig20261107_01_crate_users;
mod mig20261108_02_create_business_messages;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(mig20261107_01_crate_users::Migration),
            Box::new(mig20261108_02_create_business_messages::Migration),
        ]
    }
}
