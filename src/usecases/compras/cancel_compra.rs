use crate::domain::Compra;
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, EntradaPort};
use std::sync::Arc;
use uuid::Uuid;

pub struct CancelCompraUseCase {
    compra_repo: Arc<dyn CompraPort>,
    entrada_repo: Arc<dyn EntradaPort>,
}

impl CancelCompraUseCase {
    pub fn new(compra_repo: Arc<dyn CompraPort>, entrada_repo: Arc<dyn EntradaPort>) -> Self {
        Self { compra_repo, entrada_repo }
    }

    pub async fn execute(&self, compra_id: Uuid) -> AppResult<Compra> {
        let mut compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        if !compra.puede_cancelarse() {
            return Err(AppError::BadRequest("La compra no puede ser cancelada".to_string()));
        }

        compra.cancelar();

        let entradas = self.entrada_repo.list_by_compra(compra_id).await?;
        for mut entrada in entradas {
            entrada.cancelar();
            self.entrada_repo.update(entrada).await?;
        }

        self.compra_repo.update(compra).await
    }
}
