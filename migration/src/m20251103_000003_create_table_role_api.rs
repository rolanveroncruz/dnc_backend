use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(RoleAPI::Table)
                    .if_not_exists()
                    .col(pk_auto(RoleAPI::Id))
                    .col(string(RoleAPI::RoleId))
                    .col(string(RoleAPI::APIId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(RoleAPI::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleAPI{
    Table,
    Id,
    RoleId,
    APIId,
}
