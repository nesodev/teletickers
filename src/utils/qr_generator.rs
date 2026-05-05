use crate::error::{AppError, AppResult};
use qrcode::QrCode;
use qrcode::render::svg;
use uuid::Uuid;

pub fn generate_qr_code(entrada_id: Uuid, evento_id: Uuid) -> AppResult<String> {
    let data = format!("TICKY:{}:{}", evento_id, entrada_id);

    let code = QrCode::new(data.as_bytes()).map_err(|e| AppError::InternalServerError(format!("Error generating QR: {}", e)))?;

    let svg_string = code.render::<svg::Color>().min_dimensions(200, 200).build();

    Ok(svg_string)
}

pub fn generate_qr_data(entrada_id: Uuid, evento_id: Uuid) -> String {
    format!("TICKY:{}:{}", evento_id, entrada_id)
}

pub fn parse_qr_data(qr_data: &str) -> AppResult<(Uuid, Uuid)> {
    let parts: Vec<&str> = qr_data.split(':').collect();

    if parts.len() != 3 || parts[0] != "TICKY" {
        return Err(AppError::ValidationError("QR inválido".to_string()));
    }

    let evento_id = Uuid::parse_str(parts[1]).map_err(|_| AppError::ValidationError("Evento ID inválido en QR".to_string()))?;

    let entrada_id = Uuid::parse_str(parts[2]).map_err(|_| AppError::ValidationError("Entrada ID inválido en QR".to_string()))?;

    Ok((evento_id, entrada_id))
}
