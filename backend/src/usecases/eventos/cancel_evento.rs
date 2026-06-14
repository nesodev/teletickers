use crate::domain::Evento;
use crate::error::{AppError, AppResult};
use crate::ports::EventoPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct CancelEventoUseCase {
    evento_repo: Arc<dyn EventoPort>,
}

impl CancelEventoUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>) -> Self {
        Self { evento_repo }
    }

    pub async fn execute(&self, evento_id: Uuid) -> AppResult<Evento> {
        let mut evento = self.evento_repo.find_by_id(evento_id).await?.ok_or(AppError::EventNotFound)?;

        evento.cancelar().map_err(AppError::InternalServerError)?;

        self.evento_repo.update(evento).await
    }
}
