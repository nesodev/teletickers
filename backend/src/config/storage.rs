use std::env;

pub struct StorageConfig {
    pub bucket_name: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: Option<String>,
}

impl StorageConfig {
    pub fn from_env() -> Self {
        Self {
            bucket_name: env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set"),
            region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            access_key: env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set"),
            secret_key: env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set"),
            endpoint: env::var("S3_ENDPOINT").ok(),
        }
    }
}
