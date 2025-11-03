use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251103_000002_create_table_role::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DNCUser::Table)
                    .if_not_exists()
                    .col(pk_auto(DNCUser::Id))
                    .col(string(DNCUser::Email))
                    .col(string(DNCUser::Password))
                    .col(string(DNCUser::Name))
                    .col(ColumnDef::new(DNCUser::Role).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role")
                            .from(DNCUser::Table, DNCUser::Role)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DNCUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DNCUser{
    Table,
    Id,
    Email,
    Password,
    Name,
    Role,
}
