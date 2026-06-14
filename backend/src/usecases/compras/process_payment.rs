use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, PaymentPort, PaymentRequest, PaymentResponse};
use std::sync::Arc;
use uuid::Uuid;

pub struct ProcessPaymentUseCase {
    compra_repo: Arc<dyn CompraPort>,
    payment_service: Arc<dyn PaymentPort>,
}

impl ProcessPaymentUseCase {
    pub fn new(compra_repo: Arc<dyn CompraPort>, payment_service: Arc<dyn PaymentPort>) -> Self {
        Self { compra_repo, payment_service }
    }

    pub async fn execute(&self, compra_id: Uuid, email: String) -> AppResult<PaymentResponse> {
        let compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        let payment_request = PaymentRequest {
            compra_id: compra.id,
            monto: compra.monto_total,
            metodo_pago: compra.metodo_pago.clone(),
            email,
            descripcion: format!("Compra de entradas #{}", compra.id),
        };

        let payment_response = self.payment_service.process_payment(payment_request).await?;

        Ok(payment_response)
    }

    pub async fn confirm_payment(&self, compra_id: Uuid, codigo_operacion: String) -> AppResult<()> {
        let mut compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        compra.marcar_como_pagado(codigo_operacion);

        self.compra_repo.update(compra).await?;

        Ok(())
    }
}
