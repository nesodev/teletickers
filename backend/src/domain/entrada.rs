use crate::domain::value_objects::QrCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EstadoEntrada {
    Valido,
    Usado,
    Cancelado,
}

impl EstadoEntrada {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoEntrada::Valido => "valido",
            EstadoEntrada::Usado => "usado",
            EstadoEntrada::Cancelado => "cancelado",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "valido" => Some(EstadoEntrada::Valido),
            "usado" => Some(EstadoEntrada::Usado),
            "cancelado" => Some(EstadoEntrada::Cancelado),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entrada {
    pub id: Uuid,
    pub tipo_entrada_id: Uuid,
    pub compra_id: Uuid,
    pub qr_code: QrCode,
    pub estado: EstadoEntrada,
    pub fecha_uso: Option<DateTime<Utc>>,
}

impl Entrada {
    pub fn new(tipo_entrada_id: Uuid, compra_id: Uuid, qr_code: QrCode) -> Self {
        Self {
            id: Uuid::new_v4(),
            tipo_entrada_id,
            compra_id,
            qr_code,
            estado: EstadoEntrada::Valido,
            fecha_uso: None,
        }
    }

    pub fn puede_usarse(&self) -> bool {
        matches!(self.estado, EstadoEntrada::Valido)
    }

    pub fn usar(&mut self) -> Result<(), String> {
        if !self.puede_usarse() {
            return Err("La entrada no está válida para uso".to_string());
        }

        self.estado = EstadoEntrada::Usado;
        self.fecha_uso = Some(Utc::now());
        Ok(())
    }

    pub fn cancelar(&mut self) {
        self.estado = EstadoEntrada::Cancelado;
    }

    pub fn esta_usado(&self) -> bool {
        matches!(self.estado, EstadoEntrada::Usado)
    }

    pub fn esta_valido(&self) -> bool {
        matches!(self.estado, EstadoEntrada::Valido)
    }
}
