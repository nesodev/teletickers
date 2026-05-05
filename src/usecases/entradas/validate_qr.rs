use crate::domain::Entrada;
use crate::error::{AppError, AppResult};
use crate::ports::EntradaPort;
use std::sync::Arc;

pub struct ValidateQrUseCase {
    entrada_repo: Arc<dyn EntradaPort>,
}

impl ValidateQrUseCase {
    pub fn new(entrada_repo: Arc<dyn EntradaPort>) -> Self {
        Self { entrada_repo }
    }

    pub async fn execute(&self, qr_code: String) -> AppResult<Entrada> {
        let mut entrada = self.entrada_repo.find_by_qr(&qr_code).await?.ok_or(AppError::NotFound("Entrada".to_string()))?;

        entrada.usar().map_err(AppError::InternalServerError)?;

        self.entrada_repo.update(entrada).await
    }
}
