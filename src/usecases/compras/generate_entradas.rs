use crate::domain::Entrada;
use crate::domain::value_objects::QrCode;
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, EntradaPort};
use crate::utils::qr_generator::generate_qr_data;
use std::sync::Arc;
use uuid::Uuid;

pub struct GenerateEntradasUseCase {
    compra_repo: Arc<dyn CompraPort>,
    entrada_repo: Arc<dyn EntradaPort>,
}

impl GenerateEntradasUseCase {
    pub fn new(compra_repo: Arc<dyn CompraPort>, entrada_repo: Arc<dyn EntradaPort>) -> Self {
        Self { compra_repo, entrada_repo }
    }

    pub async fn execute(&self, compra_id: Uuid, tipo_entrada_id: Uuid, evento_id: Uuid, cantidad: i32) -> AppResult<Vec<Entrada>> {
        let compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        if !compra.esta_pagado() {
            return Err(AppError::PaymentPending);
        }

        let mut entradas = Vec::new();

        for _ in 0..cantidad {
            let entrada_id = Uuid::new_v4();
            let qr_data = generate_qr_data(entrada_id, evento_id);
            let qr_code = QrCode::new(qr_data)?;

            let entrada = Entrada::new(tipo_entrada_id, compra_id, qr_code);
            let created = self.entrada_repo.create(entrada).await?;
            entradas.push(created);
        }

        Ok(entradas)
    }
}
