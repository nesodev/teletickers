use crate::graphql::context::GraphQLContext;
use crate::graphql::mutations::auth_mutation::UserObject;
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let user_id = context.current_user_id.ok_or_else(|| Error::new("Unauthorized"))?;

        let user = context
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("User not found"))?;

        Ok(user.into())
    }
}
