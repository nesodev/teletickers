use crate::domain::TipoEntrada;
use crate::domain::value_objects::Money;
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::tipo_entrada;
use crate::ports::TipoEntradaPort;
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct TipoEntradaRepository {
    db: DatabaseConnection,
}

impl TipoEntradaRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: tipo_entrada::Model) -> AppResult<TipoEntrada> {
        Ok(TipoEntrada {
            id: model.id,
            evento_id: model.evento_id,
            nombre: model.nombre,
            precio: Money::new(model.precio)?,
            max_por_compra: model.max_por_compra,
            cantidad_disponible: model.cantidad_disponible,
            descripcion: model.descripcion,
        })
    }

    fn domain_to_active_model(&self, tipo: TipoEntrada) -> tipo_entrada::ActiveModel {
        tipo_entrada::ActiveModel {
            id: Set(tipo.id),
            evento_id: Set(tipo.evento_id),
            nombre: Set(tipo.nombre),
            precio: Set(tipo.precio.amount()),
            max_por_compra: Set(tipo.max_por_compra),
            cantidad_disponible: Set(tipo.cantidad_disponible),
            descripcion: Set(tipo.descripcion),
        }
    }
}

#[async_trait]
impl TipoEntradaPort for TipoEntradaRepository {
    async fn create(&self, domain_tipo: TipoEntrada) -> AppResult<TipoEntrada> {
        let active_model = self.domain_to_active_model(domain_tipo);
        let result = tipo_entrada::Entity::insert(active_model).exec(&self.db).await?;

        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created tipo_entrada".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<TipoEntrada>> {
        let model = tipo_entrada::Entity::find_by_id(id).one(&self.db).await?;
        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, domain_tipo: TipoEntrada) -> AppResult<TipoEntrada> {
        let active_model = self.domain_to_active_model(domain_tipo);
        tipo_entrada::Entity::update(active_model.clone()).exec(&self.db).await?;

        self.find_by_id(active_model.id.unwrap())
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated tipo_entrada".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        tipo_entrada::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_by_evento(&self, evento_id: Uuid) -> AppResult<Vec<TipoEntrada>> {
        let models = tipo_entrada::Entity::find().filter(tipo_entrada::Column::EventoId.eq(evento_id)).all(&self.db).await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn get_disponibilidad(&self, id: Uuid) -> AppResult<i32> {
        let model = tipo_entrada::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("TipoEntrada".to_string()))?;

        Ok(model.cantidad_disponible)
    }
}
