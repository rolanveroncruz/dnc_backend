use sea_orm_migration::prelude::* ;
use crate::sea_orm::DatabaseBackend;
use crate::sea_orm::Statement;
use password_auth::generate_hash;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Hash the password securely
        let password_plain = "<PASSWORD>";
        let password_hash = generate_hash(password_plain);

        // Insert the Admin user with the hashed password and linked role_id
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres, // or MySql / Sqlite — adjust accordingly
            r#"
                INSERT INTO dnc_user (email, password, name, role)
                VALUES ($1, $2, $3, (SELECT id FROM role WHERE name = 'Administrator' LIMIT 1))
            "#,
            vec![
                "admin@dnc.com.ph".into(),
                password_hash.into(),
                "Administrator".into(),
            ],
        );

        db.execute(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "
                DELETE FROM dnc_user
                WHERE name = 'Administrator';
                ",
            )
            .await?;

        Ok(())
    }
}

