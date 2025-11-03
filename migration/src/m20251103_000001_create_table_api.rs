use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(API::Table)
                    .if_not_exists()
                    .col(pk_auto(API::Id))
                    .col(string(API::Method))
                    .col(string(API::Path))
                    .col(string(API::Descr))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(API::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum API{
    Table,
    Id,
    Method,
    Path,
    Descr,
}
