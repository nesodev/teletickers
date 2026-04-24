use crate::domain::Evento;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait EventoPort: Send + Sync {
    async fn create(&self, evento: Evento) -> AppResult<Evento>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Evento>>;
    async fn update(&self, evento: Evento) -> AppResult<Evento>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_by_organizador(&self, organizador_id: Uuid) -> AppResult<Vec<Evento>>;
    async fn list_publicados(&self) -> AppResult<Vec<Evento>>;
    async fn list_by_categoria(&self, categoria: &str) -> AppResult<Vec<Evento>>;
    async fn list_by_region(&self, region: &str) -> AppResult<Vec<Evento>>;
    async fn search(&self, query: &str) -> AppResult<Vec<Evento>>;
}
