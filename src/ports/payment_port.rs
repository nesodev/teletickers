use crate::domain::{MetodoPago, Money};
use crate::error::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub compra_id: Uuid,
    pub monto: Money,
    pub metodo_pago: MetodoPago,
    pub email: String,
    pub descripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub transaction_id: String,
    pub status: String,
    pub payment_url: Option<String>,
    pub codigo_operacion: String,
}

#[async_trait]
pub trait PaymentPort: Send + Sync {
    async fn process_payment(&self, request: PaymentRequest) -> AppResult<PaymentResponse>;
    async fn verify_payment(&self, transaction_id: &str) -> AppResult<bool>;
    async fn cancel_payment(&self, transaction_id: &str) -> AppResult<()>;
    async fn refund_payment(&self, transaction_id: &str, monto: Money) -> AppResult<String>;
}
