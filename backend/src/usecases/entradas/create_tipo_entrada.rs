use crate::domain::TipoEntrada;
use crate::domain::value_objects::Money;
use crate::error::AppResult;
use crate::ports::TipoEntradaPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateTipoEntradaUseCase {
    tipo_entrada_repo: Arc<dyn TipoEntradaPort>,
}

impl CreateTipoEntradaUseCase {
    pub fn new(tipo_entrada_repo: Arc<dyn TipoEntradaPort>) -> Self {
        Self { tipo_entrada_repo }
    }

    pub async fn execute(&self, evento_id: Uuid, nombre: String, precio: Money, max_por_compra: i32, cantidad_disponible: i32, descripcion: Option<String>) -> AppResult<TipoEntrada> {
        let tipo_entrada = TipoEntrada::new(evento_id, nombre, precio, max_por_compra, cantidad_disponible, descripcion);

        self.tipo_entrada_repo.create(tipo_entrada).await
    }
}
