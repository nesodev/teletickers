use crate::domain::TipoEntrada;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TipoEntradaPort: Send + Sync {
    async fn create(&self, tipo_entrada: TipoEntrada) -> AppResult<TipoEntrada>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TipoEntrada>>;
    async fn update(&self, tipo_entrada: TipoEntrada) -> AppResult<TipoEntrada>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_by_evento(&self, evento_id: Uuid) -> AppResult<Vec<TipoEntrada>>;
    async fn get_disponibilidad(&self, id: Uuid) -> AppResult<i32>;
}
