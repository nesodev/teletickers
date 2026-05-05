use crate::domain::value_objects::Money;
use crate::domain::{Compra, EstadoPago, MetodoPago};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::compra;
use crate::ports::CompraPort;
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use sea_orm::*;
use uuid::Uuid;

pub struct CompraRepository {
    db: DatabaseConnection,
}

impl CompraRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: compra::Model) -> AppResult<Compra> {
        let fecha_compra = match model.fecha_compra {
            Some(naive) => Utc.from_utc_datetime(&naive),
            None => return Err(AppError::DatabaseError("fecha_compra is None".to_string())),
        };

        let metodo_pago_str = &model.metodo_pago;
        let estado_pago_str = model.estado_pago.as_ref().ok_or_else(|| AppError::DatabaseError("estado_pago is None".to_string()))?;

        Ok(Compra {
            id: model.id,
            usuario_id: model.usuario_id,
            evento_id: model.evento_id,
            comprobante_id: model.comprobante_id,
            fecha_compra,
            monto_total: Money::new(model.monto_total)?,
            metodo_pago: MetodoPago::from_str(metodo_pago_str).ok_or_else(|| AppError::DatabaseError("Invalid metodo_pago".to_string()))?,
            estado_pago: EstadoPago::from_str(estado_pago_str).ok_or_else(|| AppError::DatabaseError("Invalid estado_pago".to_string()))?,
            codigo_operacion: model.codigo_operacion,
        })
    }

    fn domain_to_active_model(&self, compra: Compra) -> compra::ActiveModel {
        compra::ActiveModel {
            id: Set(compra.id),
            usuario_id: Set(compra.usuario_id),
            evento_id: Set(compra.evento_id),
            comprobante_id: Set(compra.comprobante_id),
            fecha_compra: Set(Some(compra.fecha_compra.naive_utc())),
            monto_total: Set(compra.monto_total.amount()),
            metodo_pago: Set(compra.metodo_pago.as_str().to_string()),
            estado_pago: Set(Some(compra.estado_pago.as_str().to_string())),
            codigo_operacion: Set(compra.codigo_operacion),
        }
    }
}

#[async_trait]
impl CompraPort for CompraRepository {
    async fn create(&self, compra: Compra) -> AppResult<Compra> {
        let active_model = self.domain_to_active_model(compra);
        let result = compra::Entity::insert(active_model).exec(&self.db).await?;
        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created compra".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Compra>> {
        let model = compra::Entity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| self.model_to_domain(m)).transpose()?)
    }

    async fn update(&self, compra: Compra) -> AppResult<Compra> {
        let active_model = self.domain_to_active_model(compra);
        compra::Entity::update(active_model.clone()).exec(&self.db).await?;
        self.find_by_id(active_model.id.unwrap())
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated compra".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        compra::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_by_usuario(&self, usuario_id: Uuid) -> AppResult<Vec<Compra>> {
        let models = compra::Entity::find().filter(compra::Column::UsuarioId.eq(usuario_id)).all(&self.db).await?;
        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_by_evento(&self, evento_id: Uuid) -> AppResult<Vec<Compra>> {
        let models = compra::Entity::find().filter(compra::Column::EventoId.eq(evento_id)).all(&self.db).await?;
        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn count_by_evento(&self, evento_id: Uuid) -> AppResult<i64> {
        let count = compra::Entity::find()
            .filter(compra::Column::EventoId.eq(evento_id))
            .filter(compra::Column::EstadoPago.eq("pagado"))
            .count(&self.db)
            .await?;
        Ok(count as i64)
    }
}
