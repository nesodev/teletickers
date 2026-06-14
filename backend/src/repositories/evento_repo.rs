use crate::domain::{EstadoEvento, Evento, RestriccionEdad, Ubicacion};
use crate::error::{AppError, AppResult};
use crate::infrastructure::db::entities::evento;
use crate::ports::EventoPort;
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct EventoRepository {
    db: DatabaseConnection,
}

impl EventoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_domain(&self, model: evento::Model) -> AppResult<Evento> {
        let etiquetas = model.etiquetas.map(|e| e.split(',').map(|s| s.trim().to_string()).collect());

        let metodos_pago = model
            .metodos_pago_aceptados
            .and_then(|json| serde_json::from_value::<Vec<String>>(json.clone()).ok())
            .unwrap_or_else(|| vec!["yape".to_string(), "plin".to_string(), "tarjeta".to_string()]);

        let restriccion_edad = model.restriccion_edad.as_ref().and_then(|r| RestriccionEdad::from_str(r)).unwrap_or(RestriccionEdad::TodoPublico);

        let estado = model.estado.as_ref().and_then(|s| EstadoEvento::from_str(s)).unwrap_or(EstadoEvento::Borrador);

        Ok(Evento {
            id: model.id,
            titulo: model.titulo,
            descripcion: model.descripcion,
            fecha: model.fecha,
            hora: model.hora,
            ubicacion: Ubicacion {
                region: model.region,
                provincia: model.provincia,
                distrito: model.distrito,
            },
            organizador_id: model.organizador_id,
            categoria: model.categoria,
            aforo: model.aforo,
            etiquetas,
            restriccion_edad,
            miniatura: model.miniatura,
            metodos_pago_aceptados: metodos_pago,
            estado,
            fecha_creacion: model.fecha_creacion.ok_or_else(|| AppError::DatabaseError("Missing fecha_creacion".to_string()))?.and_utc(),
            fecha_actualizacion: model
                .fecha_actualizacion
                .ok_or_else(|| AppError::DatabaseError("Missing fecha_actualizacion".to_string()))?
                .and_utc(),
        })
    }

    fn domain_to_active_model(&self, evento: Evento) -> evento::ActiveModel {
        let etiquetas = evento.etiquetas.map(|e| e.join(", "));
        let metodos_pago = serde_json::to_value(&evento.metodos_pago_aceptados).ok();

        evento::ActiveModel {
            id: Set(evento.id),
            titulo: Set(evento.titulo),
            descripcion: Set(evento.descripcion),
            fecha: Set(evento.fecha),
            hora: Set(evento.hora),
            region: Set(evento.ubicacion.region),
            provincia: Set(evento.ubicacion.provincia),
            distrito: Set(evento.ubicacion.distrito),
            organizador_id: Set(evento.organizador_id),
            categoria: Set(evento.categoria),
            aforo: Set(evento.aforo),
            etiquetas: Set(etiquetas),
            restriccion_edad: Set(Some(evento.restriccion_edad.as_str().to_string())),
            miniatura: Set(evento.miniatura),
            metodos_pago_aceptados: Set(metodos_pago),
            estado: Set(Some(evento.estado.as_str().to_string())),
            fecha_creacion: Set(Some(evento.fecha_creacion.naive_utc())),
            fecha_actualizacion: Set(Some(evento.fecha_actualizacion.naive_utc())),
        }
    }
}

#[async_trait]
impl EventoPort for EventoRepository {
    async fn create(&self, evento: Evento) -> AppResult<Evento> {
        let active_model = self.domain_to_active_model(evento);
        let result = evento::Entity::insert(active_model).exec(&self.db).await?;

        self.find_by_id(result.last_insert_id)
            .await?
            .ok_or_else(|| AppError::DatabaseError("Failed to retrieve created evento".to_string()))
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Evento>> {
        let model = evento::Entity::find_by_id(id).one(&self.db).await?;
        match model {
            Some(m) => Ok(Some(self.model_to_domain(m)?)),
            None => Ok(None),
        }
    }

    async fn update(&self, evento: Evento) -> AppResult<Evento> {
        let active_model = self.domain_to_active_model(evento);
        let id = active_model.id.clone().unwrap();
        evento::Entity::update(active_model).exec(&self.db).await?;

        self.find_by_id(id).await?.ok_or_else(|| AppError::DatabaseError("Failed to retrieve updated evento".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        evento::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn list_by_organizador(&self, organizador_id: Uuid) -> AppResult<Vec<Evento>> {
        let models = evento::Entity::find().filter(evento::Column::OrganizadorId.eq(organizador_id)).all(&self.db).await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_publicados(&self) -> AppResult<Vec<Evento>> {
        let models = evento::Entity::find().filter(evento::Column::Estado.eq("publicado")).all(&self.db).await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_by_categoria(&self, categoria: &str) -> AppResult<Vec<Evento>> {
        let models = evento::Entity::find()
            .filter(evento::Column::Categoria.eq(categoria))
            .filter(evento::Column::Estado.eq("publicado"))
            .all(&self.db)
            .await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn list_by_region(&self, region: &str) -> AppResult<Vec<Evento>> {
        let models = evento::Entity::find()
            .filter(evento::Column::Region.eq(region))
            .filter(evento::Column::Estado.eq("publicado"))
            .all(&self.db)
            .await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }

    async fn search(&self, query: &str) -> AppResult<Vec<Evento>> {
        let search_pattern = format!("%{}%", query);
        let models = evento::Entity::find()
            .filter(
                Condition::any()
                    .add(evento::Column::Titulo.like(&search_pattern))
                    .add(evento::Column::Descripcion.like(&search_pattern)),
            )
            .filter(evento::Column::Estado.eq("publicado"))
            .all(&self.db)
            .await?;

        models.into_iter().map(|m| self.model_to_domain(m)).collect()
    }
}
