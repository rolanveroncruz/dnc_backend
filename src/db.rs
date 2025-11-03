use sea_orm::{Database, DatabaseConnection, DbErr};

const DATABASE_URL: &str = "postgresql://rolanveroncruz:1lap2ace@localhost:5432/dnc";
pub async fn connect_to_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}
