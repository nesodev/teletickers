use crate::domain::value_objects::Money;
use crate::error::AppResult;
use crate::ports::{CompraPort, EventoPort};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub total_eventos: usize,
    pub total_ventas: i64,
    pub ingresos_totales: Money,
    pub eventos_activos: usize,
}

pub struct GetDashboardMetricsUseCase {
    evento_repo: Arc<dyn EventoPort>,
    compra_repo: Arc<dyn CompraPort>,
}

impl GetDashboardMetricsUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>, compra_repo: Arc<dyn CompraPort>) -> Self {
        Self { evento_repo, compra_repo }
    }

    pub async fn execute(&self, organizador_id: Uuid) -> AppResult<DashboardMetrics> {
        let eventos = self.evento_repo.list_by_organizador(organizador_id).await?;
        let total_eventos = eventos.len();
        let eventos_activos = eventos.iter().filter(|e| e.esta_activo()).count();

        let mut total_ventas = 0i64;
        let mut ingresos_totales = Money::zero();

        for evento in eventos {
            let compras = self.compra_repo.list_by_evento(evento.id).await?;
            for compra in compras {
                if compra.esta_pagado() {
                    total_ventas += 1;
                    ingresos_totales = ingresos_totales + compra.monto_total;
                }
            }
        }

        Ok(DashboardMetrics {
            total_eventos,
            total_ventas,
            ingresos_totales,
            eventos_activos,
        })
    }
}
