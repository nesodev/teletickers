use crate::domain::value_objects::QrCode;
use crate::domain::{Entrada as DomainEntrada, EstadoEntrada};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::entrada;
use crate::infrastructure::db::entities::entrada::Entity as EntradaEntity;
use crate::ports::EntradaPort;
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct EntradaRepository {
    db: DatabaseConnection,
}

impl EntradaRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: entrada::Model) -> AppResult<DomainEntrada> {
        Ok(DomainEntrada {
            id: model.id,
            tipo_entrada_id: model.tipo_entrada_id,
            compra_id: model.compra_id,
            qr_code: QrCode::from_string(model.qr_code)?,
            estado: model
                .estado
                .as_deref()
                .and_then(EstadoEntrada::from_str)
                .ok_or_else(|| AppError::DatabaseError("Invalid estado".to_string()))?,
            fecha_uso: model.fecha_uso.map(|dt| dt.and_utc()),
        })
    }

    fn domain_to_active_model(&self, domain: DomainEntrada) -> entrada::ActiveModel {
        entrada::ActiveModel {
            id: Set(domain.id),
            tipo_entrada_id: Set(domain.tipo_entrada_id),
            compra_id: Set(domain.compra_id),
            qr_code: Set(domain.qr_code.value().to_string()),
            estado: Set(Some(domain.estado.as_str().to_string())),
            fecha_uso: Set(domain.fecha_uso.map(|dt| dt.naive_utc())),
        }
    }
}

#[async_trait]
impl EntradaPort for EntradaRepository {
    async fn create(&self, entrada: DomainEntrada) -> AppResult<DomainEntrada> {
        let id = entrada.id;
        let active_model = self.domain_to_active_model(entrada);
        EntradaEntity::insert(active_model).exec(&self.db).await?;
        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created entrada".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<DomainEntrada>> {
        let model = EntradaEntity::find_by_id(id).one(&self.db).await?;
        Ok(model.map(|m| self.model_to_domain(m)).transpose()?)
    }

    async fn find_by_qr(&self, qr_code: &str) -> AppResult<Option<DomainEntrada>> {
        let model = EntradaEntity::find().filter(entrada::Column::QrCode.eq(qr_code)).one(&self.db).await?;
        Ok(model.map(|m| self.model_to_domain(m)).transpose()?)
    }

    async fn update(&self, entrada: DomainEntrada) -> AppResult<DomainEntrada> {
        let id = entrada.id;
        let active_model = self.domain_to_active_model(entrada);
        EntradaEntity::update(active_model).exec(&self.db).await?;
        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated entrada".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        EntradaEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_by_compra(&self, compra_id: Uuid) -> AppResult<Vec<DomainEntrada>> {
        let models = EntradaEntity::find().filter(entrada::Column::CompraId.eq(compra_id)).all(&self.db).await?;
        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_by_tipo_entrada(&self, tipo_entrada_id: Uuid) -> AppResult<Vec<DomainEntrada>> {
        let models = EntradaEntity::find().filter(entrada::Column::TipoEntradaId.eq(tipo_entrada_id)).all(&self.db).await?;
        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }
}
