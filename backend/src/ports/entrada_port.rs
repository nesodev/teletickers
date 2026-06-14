use crate::domain::Entrada;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait EntradaPort: Send + Sync {
    async fn create(&self, entrada: Entrada) -> AppResult<Entrada>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Entrada>>;
    async fn find_by_qr(&self, qr_code: &str) -> AppResult<Option<Entrada>>;
    async fn update(&self, entrada: Entrada) -> AppResult<Entrada>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_by_compra(&self, compra_id: Uuid) -> AppResult<Vec<Entrada>>;
    async fn list_by_tipo_entrada(&self, tipo_entrada_id: Uuid) -> AppResult<Vec<Entrada>>;
}
