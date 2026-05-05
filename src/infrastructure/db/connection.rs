use crate::config::DatabaseConfig;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    Database::connect(&config.url).await
}

pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await
}
