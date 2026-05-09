use crate::domain::User;
use crate::graphql::context::GraphQLContext;
use crate::infrastructure::auth::{jwt_service::JwtService, password_hasher::PasswordHasherService};
use crate::usecases::auth::{login_user::LoginUserUseCase, register_user::RegisterUserUseCase};
use async_graphql::*;
use std::sync::Arc;

#[derive(Default)]
pub struct AuthMutation;

#[derive(SimpleObject)]
pub struct AuthPayload {
    pub user: UserObject,
    pub token: String,
}

#[derive(SimpleObject)]
pub struct UserObject {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub numero_cel: Option<String>,
    pub dni: String,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            nombre: user.nombre,
            email: user.email.value().to_string(),
            numero_cel: user.numero_cel,
            dni: user.dni.value().to_string(),
        }
    }
}

#[Object]
impl AuthMutation {
    async fn register(&self, ctx: &Context<'_>, nombre: String, email: String, password: String, dni: String, numero_cel: Option<String>) -> Result<AuthPayload> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let password_hasher = ctx.data::<Arc<PasswordHasherService>>()?;
        let jwt_service = ctx.data::<Arc<JwtService>>()?;

        let use_case = RegisterUserUseCase::new(context.user_repo.clone(), password_hasher.clone());

        let user = use_case.execute(nombre, email, numero_cel, password, dni).await.map_err(|e| Error::new(e.to_string()))?;

        let token = jwt_service.generate_token(user.id).map_err(|e| Error::new(e.to_string()))?;

        Ok(AuthPayload { user: user.into(), token })
    }

    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthPayload> {
        let context = ctx.data::<Arc<GraphQLContext>>()?;
        let password_hasher = ctx.data::<Arc<PasswordHasherService>>()?;
        let jwt_service = ctx.data::<Arc<JwtService>>()?;

        let use_case = LoginUserUseCase::new(context.user_repo.clone(), password_hasher.clone(), jwt_service.clone());

        let (user, token) = use_case.execute(email, password).await.map_err(|e| Error::new(e.to_string()))?;

        Ok(AuthPayload { user: user.into(), token })
    }
}
