pub use sea_orm_migration::prelude::*;

mod m20261013_000001_create_user;
mod m20261013_000002_create_auth_tables;
mod m20261013_000003_create_teams;
mod m20261013_000004_enable_btree_gist;
mod m20261013_000005_create_work_policies;
mod m20261013_000006_create_work_sessions;
mod m20261013_000007_create_audit_log;
mod m20261013_000008_create_scope_views;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261013_000001_create_user::Migration),
            Box::new(m20261013_000002_create_auth_tables::Migration),
            Box::new(m20261013_000003_create_teams::Migration),
            Box::new(m20261013_000004_enable_btree_gist::Migration),
            Box::new(m20261013_000005_create_work_policies::Migration),
            Box::new(m20261013_000006_create_work_sessions::Migration),
            Box::new(m20261013_000007_create_audit_log::Migration),
            Box::new(m20261013_000008_create_scope_views::Migration),
        ]
    }
}
