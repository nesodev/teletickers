use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrCode(String);

impl QrCode {
    /// Genera un nuevo código QR único
    pub fn generate() -> Self {
        let uuid = Uuid::new_v4();
        QrCode(format!("TICKY-{}", uuid.to_string().to_uppercase()))
    }

    /// Crea un QrCode desde un string existente
    pub fn from_string(value: impl Into<String>) -> AppResult<Self> {
        let value = value.into();
        Self::validate(&value)?;
        Ok(QrCode(value))
    }

    fn validate(value: &str) -> AppResult<()> {
        if value.is_empty() {
            return Err(AppError::ValidationError("El código QR no puede estar vacío".to_string()));
        }

        if value.len() > 255 {
            return Err(AppError::ValidationError("El código QR es demasiado largo".to_string()));
        }

        Ok(())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }

    /// Verifica si el QR tiene el formato esperado de Ticky
    pub fn is_valid_ticky_format(&self) -> bool {
        self.0.starts_with("TICKY-")
    }
}

impl std::fmt::Display for QrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
