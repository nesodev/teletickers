use crate::domain::{Compra, MetodoPago};
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, TipoEntradaPort};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateCompraUseCase {
    compra_repo: Arc<dyn CompraPort>,
    tipo_entrada_repo: Arc<dyn TipoEntradaPort>,
}

impl CreateCompraUseCase {
    pub fn new(compra_repo: Arc<dyn CompraPort>, tipo_entrada_repo: Arc<dyn TipoEntradaPort>) -> Self {
        Self { compra_repo, tipo_entrada_repo }
    }

    pub async fn execute(&self, usuario_id: Uuid, evento_id: Uuid, tipo_entrada_id: Uuid, cantidad: i32, metodo_pago: MetodoPago) -> AppResult<Compra> {
        let mut tipo_entrada = self.tipo_entrada_repo.find_by_id(tipo_entrada_id).await?.ok_or(AppError::NotFound("TipoEntrada".to_string()))?;

        tipo_entrada.validar_cantidad(cantidad).map_err(AppError::InternalServerError)?;

        let monto_total = tipo_entrada.precio * (cantidad as u32);

        tipo_entrada.reservar(cantidad).map_err(AppError::InternalServerError)?;
        self.tipo_entrada_repo.update(tipo_entrada).await?;

        let compra = Compra::new(usuario_id, evento_id, monto_total, metodo_pago);

        self.compra_repo.create(compra).await
    }
}
