use crate::domain::Notificacion;
use crate::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait NotificacionPort: Send + Sync {
    async fn create(&self, notificacion: Notificacion) -> AppResult<Notificacion>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Notificacion>>;
    async fn update(&self, notificacion: Notificacion) -> AppResult<Notificacion>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_by_usuario(&self, usuario_id: Uuid) -> AppResult<Vec<Notificacion>>;
    async fn list_no_leidas(&self, usuario_id: Uuid) -> AppResult<Vec<Notificacion>>;
    async fn marcar_todas_leidas(&self, usuario_id: Uuid) -> AppResult<()>;
    async fn count_no_leidas(&self, usuario_id: Uuid) -> AppResult<i64>;
}
