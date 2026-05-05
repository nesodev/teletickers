use crate::domain::{Notificacion, TipoNotificacion};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::notificacion;
use crate::ports::NotificacionPort;
use async_trait::async_trait;
use sea_orm::*;
use sea_orm_migration::prelude::Expr;
use uuid::Uuid;

pub struct NotificacionRepository {
    db: DatabaseConnection,
}

impl NotificacionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: notificacion::Model) -> AppResult<Notificacion> {
        Ok(Notificacion {
            id: model.id,
            usuario_id: model.usuario_id,
            tipo: TipoNotificacion::from_str(&model.tipo).ok_or_else(|| AppError::DatabaseError("Invalid tipo".to_string()))?,
            titulo: model.titulo,
            mensaje: model.mensaje,
            leido: model.leido.unwrap_or(false),
            fecha_creacion: model.fecha_creacion.ok_or_else(|| AppError::DatabaseError("Missing fecha_creacion".to_string()))?.and_utc(),
            evento_id: model.evento_id,
        })
    }

    fn domain_to_active_model(&self, notif: Notificacion) -> notificacion::ActiveModel {
        notificacion::ActiveModel {
            id: Set(notif.id),
            usuario_id: Set(notif.usuario_id),
            tipo: Set(notif.tipo.as_str().to_string()),
            titulo: Set(notif.titulo),
            mensaje: Set(notif.mensaje),
            leido: Set(Some(notif.leido)),
            fecha_creacion: Set(Some(notif.fecha_creacion.naive_utc())),
            evento_id: Set(notif.evento_id),
        }
    }
}

#[async_trait]
impl NotificacionPort for NotificacionRepository {
    async fn create(&self, notificacion: Notificacion) -> AppResult<Notificacion> {
        let active_model = self.domain_to_active_model(notificacion);
        let result = notificacion::Entity::insert(active_model).exec(&self.db).await?;

        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created notificacion".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Notificacion>> {
        let model = notificacion::Entity::find_by_id(id).one(&self.db).await?;
        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, notificacion: Notificacion) -> AppResult<Notificacion> {
        let active_model = self.domain_to_active_model(notificacion);
        let id = active_model.id.clone().unwrap();
        notificacion::Entity::update(active_model).exec(&self.db).await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated notificacion".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        notificacion::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_by_usuario(&self, usuario_id: Uuid) -> AppResult<Vec<Notificacion>> {
        let models = notificacion::Entity::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .order_by_desc(notificacion::Column::FechaCreacion)
            .all(&self.db)
            .await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_no_leidas(&self, usuario_id: Uuid) -> AppResult<Vec<Notificacion>> {
        let models = notificacion::Entity::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leido.eq(false))
            .order_by_desc(notificacion::Column::FechaCreacion)
            .all(&self.db)
            .await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn marcar_todas_leidas(&self, usuario_id: Uuid) -> AppResult<()> {
        notificacion::Entity::update_many()
            .col_expr(notificacion::Column::Leido, Expr::value(true))
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn count_no_leidas(&self, usuario_id: Uuid) -> AppResult<i64> {
        let count = notificacion::Entity::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leido.eq(false))
            .count(&self.db)
            .await?;

        Ok(count as i64)
    }
}
