use crate::domain::value_objects::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoComprobante {
    Boleta,
    Factura,
}

impl TipoComprobante {
    pub fn as_str(&self) -> &str {
        match self {
            TipoComprobante::Boleta => "boleta",
            TipoComprobante::Factura => "factura",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "boleta" => Some(TipoComprobante::Boleta),
            "factura" => Some(TipoComprobante::Factura),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comprobante {
    pub id: Uuid,
    pub tipo: TipoComprobante,
    pub fecha_emision: DateTime<Utc>,
    pub ruc_empresa: Option<String>,
    pub serie: String,
    pub correlativo: i32,
    pub monto_total: Money,
    pub igv: Option<Money>,
    pub archivo_pdf: Option<String>,
}

impl Comprobante {
    pub fn new_boleta(serie: String, correlativo: i32, monto_total: Money) -> Self {
        let igv = monto_total.calcular_igv();

        Self {
            id: Uuid::new_v4(),
            tipo: TipoComprobante::Boleta,
            fecha_emision: Utc::now(),
            ruc_empresa: None,
            serie,
            correlativo,
            monto_total,
            igv: Some(igv),
            archivo_pdf: None,
        }
    }

    pub fn new_factura(ruc_empresa: String, serie: String, correlativo: i32, monto_total: Money) -> Self {
        let igv = monto_total.calcular_igv();

        Self {
            id: Uuid::new_v4(),
            tipo: TipoComprobante::Factura,
            fecha_emision: Utc::now(),
            ruc_empresa: Some(ruc_empresa),
            serie,
            correlativo,
            monto_total,
            igv: Some(igv),
            archivo_pdf: None,
        }
    }

    pub fn vincular_pdf(&mut self, url: String) {
        self.archivo_pdf = Some(url);
    }

    pub fn numero_completo(&self) -> String {
        format!("{}-{:08}", self.serie, self.correlativo)
    }
}
