use crate::graphql::context::GraphQLContext;
use crate::graphql::mutations::compra_mutation::CompraObject;
use async_graphql::*;

#[derive(Default)]
pub struct CompraQuery;

#[Object]
impl CompraQuery {
    async fn mis_compras(&self, ctx: &Context<'_>) -> Result<Vec<CompraObject>> {
        let context = ctx.data::<GraphQLContext>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let compras = context.compra_repo.list_by_usuario(user_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(compras.into_iter().map(|c| c.into()).collect())
    }

    async fn compra(&self, ctx: &Context<'_>, id: String) -> Result<Option<CompraObject>> {
        let context = ctx.data::<GraphQLContext>()?;
        let compra_id = id.parse().map_err(|_| Error::new("Invalid UUID"))?;

        let compra = context.compra_repo.find_by_id(compra_id).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(compra.map(|c| c.into()))
    }
}
