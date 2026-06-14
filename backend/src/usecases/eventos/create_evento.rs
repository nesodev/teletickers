use crate::domain::{Evento, RestriccionEdad, Ubicacion};
use crate::error::AppResult;
use crate::ports::EventoPort;
use chrono::{NaiveDate, NaiveTime};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateEventoUseCase {
    evento_repo: Arc<dyn EventoPort>,
}

impl CreateEventoUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>) -> Self {
        Self { evento_repo }
    }

    pub async fn execute(
        &self,
        titulo: String,
        descripcion: Option<String>,
        fecha: NaiveDate,
        hora: NaiveTime,
        region: String,
        provincia: String,
        distrito: String,
        organizador_id: Uuid,
        categoria: String,
        aforo: i32,
        etiquetas: Option<Vec<String>>,
        restriccion_edad: RestriccionEdad,
        miniatura: Option<String>,
    ) -> AppResult<Evento> {
        let ubicacion = Ubicacion { region, provincia, distrito };

        let evento = Evento::new(
            titulo,
            descripcion,
            fecha,
            hora,
            ubicacion,
            organizador_id,
            categoria,
            aforo,
            etiquetas,
            restriccion_edad,
            miniatura,
        );

        self.evento_repo.create(evento).await
    }
}
