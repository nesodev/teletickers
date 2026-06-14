use crate::domain::Notificacion;
use crate::graphql::context::GraphQLContext;
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct NotificacionQuery;

#[derive(SimpleObject)]
pub struct NotificacionObject {
    pub id: String,
    pub tipo: String,
    pub titulo: String,
    pub mensaje: String,
    pub leido: bool,
    pub evento_id: Option<String>,
}

impl From<Notificacion> for NotificacionObject {
    fn from(notif: Notificacion) -> Self {
        Self {
            id: notif.id.to_string(),
            tipo: notif.tipo.as_str().to_string(),
            titulo: notif.titulo,
            mensaje: notif.mensaje,
            leido: notif.leido,
            evento_id: notif.evento_id.map(|id| id.to_string()),
        }
    }
}

#[Object]
impl NotificacionQuery {
    async fn mis_notificaciones(&self, ctx: &Context<'_>) -> Result<Vec<NotificacionObject>> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let notificaciones = context.notificacion_repo.list_by_usuario(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(notificaciones.into_iter().map(|n| n.into()).collect())
    }

    async fn notificaciones_no_leidas(&self, ctx: &Context<'_>) -> Result<Vec<NotificacionObject>> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let notificaciones = context.notificacion_repo.list_no_leidas(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(notificaciones.into_iter().map(|n| n.into()).collect())
    }
}
