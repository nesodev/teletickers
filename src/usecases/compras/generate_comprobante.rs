use crate::config::ComprobanteConfig;
use crate::domain::{Comprobante, TipoComprobante};
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, ComprobantePort};
use std::sync::Arc;
use uuid::Uuid;

pub struct GenerateComprobanteUseCase {
    compra_repo: Arc<dyn CompraPort>,
    comprobante_repo: Arc<dyn ComprobantePort>,
    config: ComprobanteConfig,
}

impl GenerateComprobanteUseCase {
    pub fn new(compra_repo: Arc<dyn CompraPort>, comprobante_repo: Arc<dyn ComprobantePort>, config: ComprobanteConfig) -> Self {
        Self {
            compra_repo,
            comprobante_repo,
            config,
        }
    }

    pub async fn execute(&self, compra_id: Uuid, tipo: TipoComprobante, ruc_cliente: Option<String>) -> AppResult<Comprobante> {
        let mut compra = self.compra_repo.find_by_id(compra_id).await?.ok_or(AppError::PurchaseNotFound)?;

        if !compra.esta_pagado() {
            return Err(AppError::PaymentPending);
        }

        let serie = match tipo {
            TipoComprobante::Boleta => self.config.serie_boleta.clone(),
            TipoComprobante::Factura => self.config.serie_factura.clone(),
        };

        let correlativo = self.comprobante_repo.get_next_correlativo(&serie).await?;

        let comprobante = match tipo {
            TipoComprobante::Boleta => Comprobante::new_boleta(serie, correlativo, compra.monto_total),
            TipoComprobante::Factura => {
                let ruc = ruc_cliente.ok_or_else(|| AppError::ValidationError("RUC requerido para factura".to_string()))?;
                Comprobante::new_factura(ruc, serie, correlativo, compra.monto_total)
            }
        };

        let created = self.comprobante_repo.create(comprobante).await?;

        compra.vincular_comprobante(created.id);
        self.compra_repo.update(compra).await?;

        Ok(created)
    }
}
