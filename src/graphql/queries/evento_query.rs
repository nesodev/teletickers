use crate::graphql::context::GraphQLContext;
use crate::graphql::mutations::evento_mutation::EventoObject;
use crate::usecases::eventos::GetEventosUseCase;
use async_graphql::*;

#[derive(Default)]
pub struct EventoQuery;

#[Object]
impl EventoQuery {
    async fn evento(&self, ctx: &Context<'_>, id: String) -> Result<Option<EventoObject>> {
        let context = ctx.data::<GraphQLContext>()?;
        let evento_id = id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let use_case = GetEventosUseCase::new(context.evento_repo.clone());

        let evento = use_case.get_by_id(evento_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(evento.map(|e| e.into()))
    }

    async fn eventos_publicados(&self, ctx: &Context<'_>) -> Result<Vec<EventoObject>> {
        let context = ctx.data::<GraphQLContext>()?;

        let use_case = GetEventosUseCase::new(context.evento_repo.clone());

        let eventos = use_case.list_publicados().await.map_err(|e| Error::new(e.to_string()))?;

        Ok(eventos.into_iter().map(|e| e.into()).collect())
    }

    async fn mis_eventos(&self, ctx: &Context<'_>) -> Result<Vec<EventoObject>> {
        let context = ctx.data::<GraphQLContext>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let use_case = GetEventosUseCase::new(context.evento_repo.clone());

        let eventos = use_case.list_by_organizador(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(eventos.into_iter().map(|e| e.into()).collect())
    }

    async fn search_eventos(&self, ctx: &Context<'_>, query: String) -> Result<Vec<EventoObject>> {
        let context = ctx.data::<GraphQLContext>()?;

        let use_case = GetEventosUseCase::new(context.evento_repo.clone());

        let eventos = use_case.search(&query).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(eventos.into_iter().map(|e| e.into()).collect())
    }
}
