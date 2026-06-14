use crate::domain::User;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserPort: Send + Sync {
    async fn create(&self, user: User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_dni(&self, dni: &str) -> AppResult<Option<User>>;
    async fn update(&self, user: User) -> AppResult<User>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_all(&self) -> AppResult<Vec<User>>;
}
