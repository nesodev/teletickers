use crate::domain::Comprobante;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ComprobantePort: Send + Sync {
    async fn create(&self, comprobante: Comprobante) -> AppResult<Comprobante>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Comprobante>>;
    async fn update(&self, comprobante: Comprobante) -> AppResult<Comprobante>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn get_next_correlativo(&self, serie: &str) -> AppResult<i32>;
}
