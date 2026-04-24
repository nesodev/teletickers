use crate::domain::Compra;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CompraPort: Send + Sync {
    async fn create(&self, compra: Compra) -> AppResult<Compra>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Compra>>;
    async fn update(&self, compra: Compra) -> AppResult<Compra>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_by_usuario(&self, usuario_id: Uuid) -> AppResult<Vec<Compra>>;
    async fn list_by_evento(&self, evento_id: Uuid) -> AppResult<Vec<Compra>>;
    async fn count_by_evento(&self, evento_id: Uuid) -> AppResult<i64>;
}
