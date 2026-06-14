use crate::domain::value_objects::{Dni, Email};
use crate::domain::{EstadoUsuario, User};
use crate::error::{AppError, AppResult};
use crate::infrastructure::auth::PasswordHasherService;
use crate::ports::UserPort;
use std::sync::Arc;

pub struct RegisterUserUseCase {
    user_repo: Arc<dyn UserPort>,
    password_hasher: Arc<PasswordHasherService>,
}

impl RegisterUserUseCase {
    pub fn new(user_repo: Arc<dyn UserPort>, password_hasher: Arc<PasswordHasherService>) -> Self {
        Self { user_repo, password_hasher }
    }

    pub async fn execute(&self, nombre: String, email: String, numero_cel: Option<String>, password: String, dni: String) -> AppResult<User> {
        let email_vo = Email::new(email)?;
        let dni_vo = Dni::new(dni)?;

        if self.user_repo.find_by_email(email_vo.value()).await?.is_some() {
            return Err(AppError::AlreadyExists("Email".to_string()));
        }

        if self.user_repo.find_by_dni(dni_vo.value()).await?.is_some() {
            return Err(AppError::AlreadyExists("DNI".to_string()));
        }

        let hashed_password = self.password_hasher.hash_password(&password)?;

        let user = User {
            id: uuid::Uuid::new_v4(),
            nombre,
            email: email_vo,
            numero_cel,
            password_hash: hashed_password,
            estado: EstadoUsuario::Activo,
            dni: dni_vo,
            fecha_registro: chrono::Utc::now(),
        };

        self.user_repo.create(user).await
    }
}
