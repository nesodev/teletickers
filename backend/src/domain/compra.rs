use crate::domain::value_objects::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetodoPago {
    Yape,
    Plin,
    Tarjeta,
}

impl MetodoPago {
    pub fn as_str(&self) -> &str {
        match self {
            MetodoPago::Yape => "yape",
            MetodoPago::Plin => "plin",
            MetodoPago::Tarjeta => "tarjeta",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "yape" => Some(MetodoPago::Yape),
            "plin" => Some(MetodoPago::Plin),
            "tarjeta" => Some(MetodoPago::Tarjeta),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EstadoPago {
    Pendiente,
    Pagado,
    Cancelado,
    Reembolsado,
}

impl EstadoPago {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoPago::Pendiente => "pendiente",
            EstadoPago::Pagado => "pagado",
            EstadoPago::Cancelado => "cancelado",
            EstadoPago::Reembolsado => "reembolsado",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pendiente" => Some(EstadoPago::Pendiente),
            "pagado" => Some(EstadoPago::Pagado),
            "cancelado" => Some(EstadoPago::Cancelado),
            "reembolsado" => Some(EstadoPago::Reembolsado),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compra {
    pub id: Uuid,
    pub usuario_id: Uuid,
    pub evento_id: Uuid,
    pub comprobante_id: Option<Uuid>,
    pub fecha_compra: DateTime<Utc>,
    pub monto_total: Money,
    pub metodo_pago: MetodoPago,
    pub estado_pago: EstadoPago,
    pub codigo_operacion: Option<String>,
}

impl Compra {
    pub fn new(usuario_id: Uuid, evento_id: Uuid, monto_total: Money, metodo_pago: MetodoPago) -> Self {
        Self {
            id: Uuid::new_v4(),
            usuario_id,
            evento_id,
            comprobante_id: None,
            fecha_compra: Utc::now(),
            monto_total,
            metodo_pago,
            estado_pago: EstadoPago::Pendiente,
            codigo_operacion: None,
        }
    }

    pub fn marcar_como_pagado(&mut self, codigo_operacion: String) {
        self.estado_pago = EstadoPago::Pagado;
        self.codigo_operacion = Some(codigo_operacion);
    }

    pub fn cancelar(&mut self) {
        self.estado_pago = EstadoPago::Cancelado;
    }

    pub fn reembolsar(&mut self) {
        self.estado_pago = EstadoPago::Reembolsado;
    }

    pub fn vincular_comprobante(&mut self, comprobante_id: Uuid) {
        self.comprobante_id = Some(comprobante_id);
    }

    pub fn esta_pagado(&self) -> bool {
        matches!(self.estado_pago, EstadoPago::Pagado)
    }

    pub fn puede_cancelarse(&self) -> bool {
        matches!(self.estado_pago, EstadoPago::Pendiente | EstadoPago::Pagado)
    }
}
