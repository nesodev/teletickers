use crate::ports::EntradaPort;
use crate::usecases::entradas::ValidateQrUseCase;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ValidateQrRequest {
    pub qr_code: String,
}

#[derive(Serialize)]
pub struct ValidateQrResponse {
    pub success: bool,
    pub message: String,
    pub entrada_id: Option<String>,
}

#[post("/qr/validate")]
pub async fn validate_qr(entrada_repo: web::Data<Arc<dyn EntradaPort>>, req: web::Json<ValidateQrRequest>) -> impl Responder {
    let use_case = ValidateQrUseCase::new(entrada_repo.get_ref().clone());

    match use_case.execute(req.qr_code.clone()).await {
        Ok(entrada) => HttpResponse::Ok().json(ValidateQrResponse {
            success: true,
            message: "Entrada válida y marcada como usada".to_string(),
            entrada_id: Some(entrada.id.to_string()),
        }),
        Err(e) => HttpResponse::BadRequest().json(ValidateQrResponse {
            success: false,
            message: e.to_string(),
            entrada_id: None,
        }),
    }
}
