use crate::domain::{Evento, RestriccionEdad};
use crate::graphql::context::GraphQLContext;
use crate::usecases::eventos::*;
use async_graphql::*;
use chrono::{NaiveDate, NaiveTime};
use std::sync::Arc;

#[derive(Default)]
pub struct EventoMutation;

#[derive(SimpleObject)]
pub struct EventoObject {
    pub id: String,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub fecha: String,
    pub hora: String,
    pub region: String,
    pub provincia: String,
    pub distrito: String,
    pub categoria: String,
    pub aforo: i32,
    pub estado: String,
}

impl From<Evento> for EventoObject {
    fn from(evento: Evento) -> Self {
        Self {
            id: evento.id.to_string(),
            titulo: evento.titulo,
            descripcion: evento.descripcion,
            fecha: evento.fecha.to_string(),
            hora: evento.hora.to_string(),
            region: evento.ubicacion.region,
            provincia: evento.ubicacion.provincia,
            distrito: evento.ubicacion.distrito,
            categoria: evento.categoria,
            aforo: evento.aforo,
            estado: evento.estado.as_str().to_string(),
        }
    }
}

#[Object]
impl EventoMutation {
    async fn create_evento(
        &self,
        ctx: &Context<'_>,
        titulo: String,
        descripcion: Option<String>,
        fecha: String,
        hora: String,
        region: String,
        provincia: String,
        distrito: String,
        categoria: String,
        aforo: i32,
        etiquetas: Option<Vec<String>>,
        restriccion_edad: Option<String>,
        miniatura: Option<String>,
    ) -> Result<EventoObject> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let fecha_parsed = NaiveDate::parse_from_str(&fecha, "%Y-%m-%d").map_err(|_| Error::new("Invalid date format"))?;
        let hora_parsed = NaiveTime::parse_from_str(&hora, "%H:%M:%S").map_err(|_| Error::new("Invalid time format"))?;

        let restriccion = restriccion_edad.and_then(|r| RestriccionEdad::from_str(&r)).unwrap_or(RestriccionEdad::TodoPublico);

        let use_case = CreateEventoUseCase::new(context.evento_repo.clone());

        let evento = use_case
            .execute(
                titulo,
                descripcion,
                fecha_parsed,
                hora_parsed,
                region,
                provincia,
                distrito,
                user_id,
                categoria,
                aforo,
                etiquetas,
                restriccion,
                miniatura,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(evento.into())
    }

    async fn publish_evento(&self, ctx: &Context<'_>, evento_id: String) -> Result<EventoObject> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let id = evento_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = PublishEventoUseCase::new(context.evento_repo.clone());

        let evento = use_case.execute(id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(evento.into())
    }

    async fn cancel_evento(&self, ctx: &Context<'_>, evento_id: String) -> Result<EventoObject> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let id = evento_id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = CancelEventoUseCase::new(context.evento_repo.clone());

        let evento = use_case.execute(id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(evento.into())
    }
}
