use crate::error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> AppResult<()>;
    async fn send_sms(&self, phone: &str, message: &str) -> AppResult<()>;
}
