use crate::domain::{Evento, RestriccionEdad};
use crate::error::{AppError, AppResult};
use crate::ports::EventoPort;
use chrono::{NaiveDate, NaiveTime};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateEventoUseCase {
    evento_repo: Arc<dyn EventoPort>,
}

impl UpdateEventoUseCase {
    pub fn new(evento_repo: Arc<dyn EventoPort>) -> Self {
        Self { evento_repo }
    }

    pub async fn execute(
        &self,
        evento_id: Uuid,
        titulo: Option<String>,
        descripcion: Option<String>,
        fecha: Option<NaiveDate>,
        hora: Option<NaiveTime>,
        categoria: Option<String>,
        aforo: Option<i32>,
        etiquetas: Option<Vec<String>>,
        restriccion_edad: Option<RestriccionEdad>,
        miniatura: Option<String>,
    ) -> AppResult<Evento> {
        let mut evento = self.evento_repo.find_by_id(evento_id).await?.ok_or(AppError::EventNotFound)?;

        if evento.esta_publicado() {
            return Err(AppError::EventAlreadyPublished);
        }

        if let Some(t) = titulo {
            evento.titulo = t;
        }
        if let Some(d) = descripcion {
            evento.descripcion = Some(d);
        }
        if let Some(f) = fecha {
            evento.fecha = f;
        }
        if let Some(h) = hora {
            evento.hora = h;
        }
        if let Some(c) = categoria {
            evento.categoria = c;
        }
        if let Some(a) = aforo {
            evento.aforo = a;
        }
        if let Some(e) = etiquetas {
            evento.etiquetas = Some(e);
        }
        if let Some(r) = restriccion_edad {
            evento.restriccion_edad = r;
        }
        if let Some(m) = miniatura {
            evento.miniatura = Some(m);
        }

        evento.fecha_actualizacion = chrono::Utc::now();

        self.evento_repo.update(evento).await
    }
}
