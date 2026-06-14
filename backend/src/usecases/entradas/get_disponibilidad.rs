use crate::domain::TipoEntrada;
use crate::error::AppResult;
use crate::ports::TipoEntradaPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetDisponibilidadUseCase {
    tipo_entrada_repo: Arc<dyn TipoEntradaPort>,
}

impl GetDisponibilidadUseCase {
    pub fn new(tipo_entrada_repo: Arc<dyn TipoEntradaPort>) -> Self {
        Self { tipo_entrada_repo }
    }

    pub async fn execute(&self, tipo_entrada_id: Uuid) -> AppResult<i32> {
        self.tipo_entrada_repo.get_disponibilidad(tipo_entrada_id).await
    }

    pub async fn list_by_evento(&self, evento_id: Uuid) -> AppResult<Vec<TipoEntrada>> {
        self.tipo_entrada_repo.list_by_evento(evento_id).await
    }
}
