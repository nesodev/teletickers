use crate::error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn upload_file(&self, file_name: &str, content: Vec<u8>, content_type: &str) -> AppResult<String>;
    async fn delete_file(&self, file_url: &str) -> AppResult<()>;
    async fn get_file_url(&self, file_name: &str) -> AppResult<String>;
}
