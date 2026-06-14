use crate::config::StorageConfig;
use crate::error::{AppError, AppResult};
use crate::ports::StoragePort;
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

pub struct SupabaseStorageService {
    client: Client,
    endpoint: String,
    api_key: String,
    bucket: String,
}

impl SupabaseStorageService {
    pub fn new(config: StorageConfig) -> Self {
        let endpoint = config.endpoint.unwrap_or_else(|| "https://your-project.supabase.co".to_string());
        Self {
            client: Client::new(),
            endpoint,
            api_key: config.access_key,
            bucket: config.bucket_name,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/storage/v1/object/{}", self.endpoint, path)
    }
}

#[async_trait]
impl StoragePort for SupabaseStorageService {
    async fn upload_file(&self, file_name: &str, content: Vec<u8>, content_type: &str) -> AppResult<String> {
        let url = self.build_url(&format!("{}/{}", self.bucket, file_name));
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", content_type)
            .body(content)
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Upload failed: {}", e)))?;

        if response.status() != StatusCode::OK && response.status() != StatusCode::CREATED {
            return Err(AppError::StorageError(format!("Upload failed with status: {}", response.status())));
        }

        Ok(format!("{}/storage/v1/object/public/{}/{}", self.endpoint, self.bucket, file_name))
    }

    async fn delete_file(&self, file_url: &str) -> AppResult<()> {
        let file_name = file_url.split('/').last().ok_or_else(|| AppError::ValidationError("Invalid file URL".to_string()))?;
        let url = self.build_url(&format!("{}/{}", self.bucket, file_name));
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Delete failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::StorageError(format!("Delete failed with status: {}", response.status())));
        }

        Ok(())
    }

    async fn get_file_url(&self, file_name: &str) -> AppResult<String> {
        Ok(format!("{}/storage/v1/object/public/{}/{}", self.endpoint, self.bucket, file_name))
    }
}
