pub use sea_orm_migration::prelude::*;

mod m20251103_000001_create_table_api;
mod m20251103_000002_create_table_role;
mod m20251103_000003_create_table_role_api;
mod m20251103_000004_create_table_dnc_user;
mod m20251103_000005_create_admin_role;
mod m20251103_000005_create_admin_user;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251103_000001_create_table_api::Migration),
            Box::new(m20251103_000002_create_table_role::Migration),
            Box::new(m20251103_000003_create_table_role_api::Migration),
            Box::new(m20251103_000004_create_table_dnc_user::Migration),
            Box::new(m20251103_000005_create_admin_role::Migration),
            Box::new(m20251103_000005_create_admin_user::Migration),
        ]
    }
}
