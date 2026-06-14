use crate::domain::User;
use crate::error::{AppError, AppResult};
use crate::infrastructure::auth::{JwtService, PasswordHasherService};
use crate::ports::UserPort;
use std::sync::Arc;

pub struct LoginUserUseCase {
    user_repo: Arc<dyn UserPort>,
    password_hasher: Arc<PasswordHasherService>,
    jwt_service: Arc<JwtService>,
}

impl LoginUserUseCase {
    pub fn new(user_repo: Arc<dyn UserPort>, password_hasher: Arc<PasswordHasherService>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repo,
            password_hasher,
            jwt_service,
        }
    }

    pub async fn execute(&self, email: String, password: String) -> AppResult<(User, String)> {
        let user = self.user_repo.find_by_email(&email).await?.ok_or(AppError::InvalidCredentials)?;

        if !user.is_active() {
            return Err(AppError::Unauthorized("Usuario baneado".to_string()));
        }

        let is_valid = self.password_hasher.verify_password(&password, &user.password_hash)?;

        if !is_valid {
            return Err(AppError::InvalidCredentials);
        }

        let token = self.jwt_service.generate_token(user.id)?;

        Ok((user, token))
    }
}
