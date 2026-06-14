use crate::domain::Evento;
use crate::error::AppResult;
use crate::ports::EventoPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetEventosUseCase {
    evento_repo: Arc<dyn EventoPort>,
}

impl GetEventosUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>) -> Self {
        Self { evento_repo }
    }

    pub async fn get_by_id(&self, evento_id: Uuid) -> AppResult<Option<Evento>> {
        self.evento_repo.find_by_id(evento_id).await
    }

    pub async fn list_publicados(&self) -> AppResult<Vec<Evento>> {
        self.evento_repo.list_publicados().await
    }

    pub async fn list_by_organizador(&self, organizador_id: Uuid) -> AppResult<Vec<Evento>> {
        self.evento_repo.list_by_organizador(organizador_id).await
    }

    pub async fn list_by_categoria(&self, categoria: &str) -> AppResult<Vec<Evento>> {
        self.evento_repo.list_by_categoria(categoria).await
    }

    pub async fn list_by_region(&self, region: &str) -> AppResult<Vec<Evento>> {
        self.evento_repo.list_by_region(region).await
    }

    pub async fn search(&self, query: &str) -> AppResult<Vec<Evento>> {
        self.evento_repo.search(query).await
    }
}
