use crate::ports::CompraPort;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PaymentWebhookRequest {
    pub compra_id: String,
    pub transaction_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct PaymentWebhookResponse {
    pub success: bool,
    pub message: String,
}

#[post("/webhook/payment")]
pub async fn payment_webhookx(compra_repo: web::Data<Arc<dyn CompraPort>>, req: web::Json<PaymentWebhookRequest>) -> impl Responder {
    if req.status != "success" {
        return HttpResponse::Ok().json(PaymentWebhookResponse {
            success: false,
            message: "Payment not successful".to_string(),
        });
    }

    let compra_id = match req.compra_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(PaymentWebhookResponse {
                success: false,
                message: "Invalid compra_id".to_string(),
            });
        }
    };

    match compra_repo.find_by_id(compra_id).await {
        Ok(Some(mut compra)) => {
            compra.marcar_como_pagado(req.transaction_id.clone());
            match compra_repo.update(compra).await {
                Ok(_) => HttpResponse::Ok().json(PaymentWebhookResponse {
                    success: true,
                    message: "Payment confirmed".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(PaymentWebhookResponse {
                    success: false,
                    message: e.to_string(),
                }),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(PaymentWebhookResponse {
            success: false,
            message: "Compra not found".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(PaymentWebhookResponse {
            success: false,
            message: e.to_string(),
        }),
    }
}
