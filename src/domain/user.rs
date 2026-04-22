use crate::domain::value_objects::{Dni, Email};
use crate::error::{AppError, AppResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstadoUsuario {
    Activo,
    Baneado,
}

impl EstadoUsuario {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoUsuario::Activo => "activo",
            EstadoUsuario::Baneado => "baneado",
        }
    }

    pub fn from_str(s: &str) -> AppResult<Self> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoUsuario::Activo),
            "baneado" => Ok(EstadoUsuario::Baneado),
            _ => Err(AppError::ValidationError(format!("Estado de usuario inválido: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub nombre: String,
    pub email: Email,
    pub numero_cel: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub estado: EstadoUsuario,
    pub dni: Dni,
    pub fecha_registro: DateTime<Utc>,
}

impl User {
    pub fn new(nombre: String, email: Email, numero_cel: Option<String>, password_hash: String, dni: Dni) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre,
            email,
            numero_cel,
            password_hash,
            estado: EstadoUsuario::Activo,
            dni,
            fecha_registro: Utc::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.estado, EstadoUsuario::Activo)
    }

    pub fn ban(&mut self) -> AppResult<()> {
        if self.estado == EstadoUsuario::Baneado {
            return Err(AppError::ValidationError("El usuario ya está baneado".to_string()));
        }
        self.estado = EstadoUsuario::Baneado;
        Ok(())
    }

    pub fn unban(&mut self) -> AppResult<()> {
        if self.estado == EstadoUsuario::Activo {
            return Err(AppError::ValidationError("El usuario ya está activo".to_string()));
        }
        self.estado = EstadoUsuario::Activo;
        Ok(())
    }

    pub fn update_profile(&mut self, nombre: Option<String>, numero_cel: Option<String>) -> AppResult<()> {
        if let Some(n) = nombre {
            if n.trim().is_empty() {
                return Err(AppError::ValidationError("El nombre no puede estar vacío".to_string()));
            }
            self.nombre = n;
        }

        if let Some(cel) = numero_cel {
            // Validación básica de teléfono peruano (9 dígitos)
            if !cel.is_empty() && (cel.len() != 9 || !cel.chars().all(|c| c.is_ascii_digit())) {
                return Err(AppError::ValidationError("Número de celular inválido".to_string()));
            }
            self.numero_cel = if cel.is_empty() { None } else { Some(cel) };
        }

        Ok(())
    }
}
