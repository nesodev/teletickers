use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a number"),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .expect("DB_MIN_CONNECTIONS must be a number"),
        }
    }

    pub async fn create_connection(&self) -> Result<DatabaseConnection, DbErr> {
        Database::connect(&self.url).await
    }
}
