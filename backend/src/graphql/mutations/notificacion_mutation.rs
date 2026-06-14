use crate::graphql::context::GraphQLContext;
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct NotificacionMutation;

#[Object]
impl NotificacionMutation {
    async fn marcar_todas_leidas(&self, ctx: &Context<'_>) -> Result<bool> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        context.notificacion_repo.marcar_todas_leidas(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
