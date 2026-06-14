use crate::domain::value_objects::{Dni, Email};
use crate::domain::{EstadoUsuario, User};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::{prelude::*, usuario};
use crate::ports::UserPort;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::*;
use uuid::Uuid;

pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: usuario::Model) -> AppResult<User> {
        Ok(User {
            id: model.id,
            nombre: model.nombre,
            email: Email::new(model.email)?,
            numero_cel: model.numero_cel,
            password_hash: model.contraseña,
            estado: EstadoUsuario::from_str(model.estado.as_deref().ok_or_else(|| AppError::DatabaseError("Estado de usuario nulo".to_string()))?)?,
            dni: Dni::new(model.dni)?,
            fecha_registro: model
                .fecha_registro
                .map(|f| DateTime::<Utc>::from_naive_utc_and_offset(f, Utc))
                .ok_or_else(|| AppError::DatabaseError("Fecha de registro nula".to_string()))?,
        })
    }

    fn domain_to_active_model(&self, user: User) -> usuario::ActiveModel {
        usuario::ActiveModel {
            id: Set(user.id),
            nombre: Set(user.nombre),
            email: Set(user.email.value().to_string()),
            numero_cel: Set(user.numero_cel),
            contraseña: Set(user.password_hash),
            estado: Set(Some(user.estado.as_str().to_string())),
            dni: Set(user.dni.value().to_string()),
            fecha_registro: Set(Some(user.fecha_registro.naive_utc())),
        }
    }
}

#[async_trait]
impl UserPort for UserRepository {
    async fn create(&self, user: User) -> AppResult<User> {
        let active_model = self.domain_to_active_model(user);
        let result = Usuario::insert(active_model).exec(&self.db).await?;

        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created user".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let model = Usuario::find_by_id(id).one(&self.db).await?;
        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let model = Usuario::find().filter(usuario::Column::Email.eq(email)).one(&self.db).await?;

        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn find_by_dni(&self, dni: &str) -> AppResult<Option<User>> {
        let model = Usuario::find().filter(usuario::Column::Dni.eq(dni)).one(&self.db).await?;

        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, user: User) -> AppResult<User> {
        let active_model = self.domain_to_active_model(user);
        Usuario::update(active_model.clone()).exec(&self.db).await?;

        self.find_by_id(active_model.id.unwrap())
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated user".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        Usuario::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_all(&self) -> AppResult<Vec<User>> {
        let models = Usuario::find().all(&self.db).await?;
        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }
}
