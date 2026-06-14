use crate::domain::value_objects::Money;
use crate::error::{AppError, AppResult};
use crate::ports::{CompraPort, EntradaPort, EventoPort};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventoStats {
    pub evento_id: Uuid,
    pub titulo: String,
    pub total_ventas: i64,
    pub ingresos: Money,
    pub entradas_vendidas: usize,
    pub aforo: i32,
    pub porcentaje_ocupacion: f64,
}

pub struct GetEventoStatsUseCase {
    evento_repo: Arc<dyn EventoPort>,
    compra_repo: Arc<dyn CompraPort>,
    entrada_repo: Arc<dyn EntradaPort>,
}

impl GetEventoStatsUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>, compra_repo: Arc<dyn CompraPort>, entrada_repo: Arc<dyn EntradaPort>) -> Self {
        Self {
            evento_repo,
            compra_repo,
            entrada_repo,
        }
    }

    pub async fn execute(&self, evento_id: Uuid) -> AppResult<EventoStats> {
        let evento = self.evento_repo.find_by_id(evento_id).await?.ok_or(AppError::EventNotFound)?;

        let compras = self.compra_repo.list_by_evento(evento_id).await?;
        let total_ventas = self.compra_repo.count_by_evento(evento_id).await?;

        let mut ingresos = Money::zero();
        let mut entradas_vendidas = 0;

        for compra in compras {
            if compra.esta_pagado() {
                ingresos = ingresos + compra.monto_total;
                let entradas = self.entrada_repo.list_by_compra(compra.id).await?;
                entradas_vendidas += entradas.len();
            }
        }

        let porcentaje_ocupacion = if evento.aforo > 0 { (entradas_vendidas as f64 / evento.aforo as f64) * 100.0 } else { 0.0 };

        Ok(EventoStats {
            evento_id: evento.id,
            titulo: evento.titulo,
            total_ventas,
            ingresos,
            entradas_vendidas,
            aforo: evento.aforo,
            porcentaje_ocupacion,
        })
    }
}
