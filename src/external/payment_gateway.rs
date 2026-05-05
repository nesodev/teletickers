use crate::config::PaymentConfig;
use crate::domain::{MetodoPago, Money};
use crate::error::{AppError, AppResult};
use crate::ports::{PaymentPort, PaymentRequest, PaymentResponse};
use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct StripePaymentRequest {
    amount: i64,
    currency: String,
    description: String,
    payment_method_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct StripePaymentResponse {
    id: String,
    status: String,
    client_secret: Option<String>,
}

pub struct PaymentGatewayService {
    config: PaymentConfig,
    client: Client,
}

impl PaymentGatewayService {
    pub fn new(config: PaymentConfig) -> Self {
        Self { config, client: Client::new() }
    }

    async fn process_stripe(&self, request: &PaymentRequest) -> AppResult<PaymentResponse> {
        let amount_cents = (request.monto.amount() * Decimal::from(100)).to_i64().unwrap_or(0);

        let stripe_request = StripePaymentRequest {
            amount: amount_cents,
            currency: "pen".to_string(),
            description: request.descripcion.clone(),
            payment_method_types: vec!["card".to_string()],
        };

        let response = self
            .client
            .post("https://api.stripe.com/v1/payment_intents")
            .bearer_auth(&self.config.stripe_secret_key)
            .json(&stripe_request)
            .send()
            .await
            .map_err(|e| AppError::PaymentFailed(format!("Stripe request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::PaymentFailed(format!("Stripe returned status: {}", response.status())));
        }

        let stripe_response: StripePaymentResponse = response.json().await.map_err(|e| AppError::PaymentFailed(format!("Failed to parse response: {}", e)))?;

        Ok(PaymentResponse {
            transaction_id: stripe_response.id.clone(),
            status: stripe_response.status,
            payment_url: stripe_response.client_secret,
            codigo_operacion: stripe_response.id,
        })
    }

    async fn process_yape(&self, _request: &PaymentRequest) -> AppResult<PaymentResponse> {
        Ok(PaymentResponse {
            transaction_id: uuid::Uuid::new_v4().to_string(),
            status: "pending".to_string(),
            payment_url: Some("https://yape.com.pe/payment".to_string()),
            codigo_operacion: format!("YAPE-{}", uuid::Uuid::new_v4()),
        })
    }

    async fn process_plin(&self, _request: &PaymentRequest) -> AppResult<PaymentResponse> {
        Ok(PaymentResponse {
            transaction_id: uuid::Uuid::new_v4().to_string(),
            status: "pending".to_string(),
            payment_url: Some("https://plin.pe/payment".to_string()),
            codigo_operacion: format!("PLIN-{}", uuid::Uuid::new_v4()),
        })
    }
}

#[async_trait]
impl PaymentPort for PaymentGatewayService {
    async fn process_payment(&self, request: PaymentRequest) -> AppResult<PaymentResponse> {
        match request.metodo_pago {
            MetodoPago::Tarjeta => self.process_stripe(&request).await,
            MetodoPago::Yape => self.process_yape(&request).await,
            MetodoPago::Plin => self.process_plin(&request).await,
        }
    }

    async fn verify_payment(&self, transaction_id: &str) -> AppResult<bool> {
        let url = format!("https://api.stripe.com/v1/payment_intents/{}", transaction_id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.config.stripe_secret_key)
            .send()
            .await
            .map_err(|e| AppError::PaymentFailed(format!("Verification failed: {}", e)))?;

        if !response.status().is_success() {
            return Ok(false);
        }

        let payment: StripePaymentResponse = response.json().await.map_err(|e| AppError::PaymentFailed(format!("Failed to parse response: {}", e)))?;

        Ok(payment.status == "succeeded")
    }

    async fn cancel_payment(&self, _transaction_id: &str) -> AppResult<()> {
        Ok(())
    }

    async fn refund_payment(&self, transaction_id: &str, _monto: Money) -> AppResult<String> {
        Ok(format!("REFUND-{}", transaction_id))
    }
}
