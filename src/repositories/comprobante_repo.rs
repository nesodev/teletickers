use crate::domain::value_objects::Money;
use crate::domain::{Comprobante, TipoComprobante};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::comprobante;
use crate::ports::ComprobantePort;
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct ComprobanteRepository {
    db: DatabaseConnection,
}

impl ComprobanteRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: comprobante::Model) -> AppResult<Comprobante> {
        let igv_money = model.igv.map(|v| Money::new(v)).transpose()?;

        Ok(Comprobante {
            id: model.id,
            tipo: TipoComprobante::from_str(&model.tipo).ok_or_else(|| AppError::DatabaseError("Invalid tipo".to_string()))?,
            fecha_emision: model.fecha_emision.ok_or_else(|| AppError::DatabaseError("Missing fecha_emision".to_string()))?.and_utc(),
            ruc_empresa: model.ruc_empresa,
            serie: model.serie,
            correlativo: model.correlativo.unwrap_or(0),
            monto_total: Money::new(model.monto_total)?,
            igv: igv_money,
            archivo_pdf: model.archivo_pdf,
        })
    }

    fn domain_to_active_model(&self, comprobante: Comprobante) -> comprobante::ActiveModel {
        let igv_decimal = comprobante.igv.map(|m| m.amount());

        comprobante::ActiveModel {
            id: Set(comprobante.id),
            tipo: Set(comprobante.tipo.as_str().to_string()),
            fecha_emision: Set(Some(comprobante.fecha_emision.naive_utc())),
            ruc_empresa: Set(comprobante.ruc_empresa),
            serie: Set(comprobante.serie),
            correlativo: Set(Some(comprobante.correlativo)),
            monto_total: Set(comprobante.monto_total.amount()),
            igv: Set(igv_decimal),
            archivo_pdf: Set(comprobante.archivo_pdf),
        }
    }
}

#[async_trait]
impl ComprobantePort for ComprobanteRepository {
    async fn create(&self, comprobante: Comprobante) -> AppResult<Comprobante> {
        let active_model = self.domain_to_active_model(comprobante);
        let result = comprobante::Entity::insert(active_model).exec(&self.db).await?;

        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created comprobante".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Comprobante>> {
        let model = comprobante::Entity::find_by_id(id).one(&self.db).await?;
        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, comprobante: Comprobante) -> AppResult<Comprobante> {
        let active_model = self.domain_to_active_model(comprobante);
        let id = active_model.id.clone().unwrap();
        comprobante::Entity::update(active_model).exec(&self.db).await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated comprobante".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        comprobante::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn get_next_correlativo(&self, serie: &str) -> AppResult<i32> {
        let last = comprobante::Entity::find()
            .filter(comprobante::Column::Serie.eq(serie))
            .order_by_desc(comprobante::Column::Correlativo)
            .one(&self.db)
            .await?;

        match last {
            Some(model) => Ok(model.correlativo.unwrap_or(0) + 1),
            None => Ok(1),
        }
    }
}
