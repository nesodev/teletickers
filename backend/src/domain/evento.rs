use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestriccionEdad {
    TodoPublico,
    ConAdulto,
    SoloMayores,
}

impl RestriccionEdad {
    pub fn as_str(&self) -> &str {
        match self {
            RestriccionEdad::TodoPublico => "todo_publico",
            RestriccionEdad::ConAdulto => "con_adulto",
            RestriccionEdad::SoloMayores => "solo_mayores",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "todo_publico" => Some(RestriccionEdad::TodoPublico),
            "con_adulto" => Some(RestriccionEdad::ConAdulto),
            "solo_mayores" => Some(RestriccionEdad::SoloMayores),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EstadoEvento {
    Borrador,
    Publicado,
    Cancelado,
    Finalizado,
}

impl EstadoEvento {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoEvento::Borrador => "borrador",
            EstadoEvento::Publicado => "publicado",
            EstadoEvento::Cancelado => "cancelado",
            EstadoEvento::Finalizado => "finalizado",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "borrador" => Some(EstadoEvento::Borrador),
            "publicado" => Some(EstadoEvento::Publicado),
            "cancelado" => Some(EstadoEvento::Cancelado),
            "finalizado" => Some(EstadoEvento::Finalizado),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ubicacion {
    pub region: String,
    pub provincia: String,
    pub distrito: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evento {
    pub id: Uuid,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub fecha: NaiveDate,
    pub hora: NaiveTime,
    pub ubicacion: Ubicacion,
    pub organizador_id: Uuid,
    pub categoria: String,
    pub aforo: i32,
    pub etiquetas: Option<Vec<String>>,
    pub restriccion_edad: RestriccionEdad,
    pub miniatura: Option<String>,
    pub metodos_pago_aceptados: Vec<String>,
    pub estado: EstadoEvento,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_actualizacion: DateTime<Utc>,
}

impl Evento {
    pub fn new(
        titulo: String,
        descripcion: Option<String>,
        fecha: NaiveDate,
        hora: NaiveTime,
        ubicacion: Ubicacion,
        organizador_id: Uuid,
        categoria: String,
        aforo: i32,
        etiquetas: Option<Vec<String>>,
        restriccion_edad: RestriccionEdad,
        miniatura: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            titulo,
            descripcion,
            fecha,
            hora,
            ubicacion,
            organizador_id,
            categoria,
            aforo,
            etiquetas,
            restriccion_edad,
            miniatura,
            metodos_pago_aceptados: vec!["yape".to_string(), "plin".to_string(), "tarjeta".to_string()],
            estado: EstadoEvento::Borrador,
            fecha_creacion: Utc::now(),
            fecha_actualizacion: Utc::now(),
        }
    }

    pub fn puede_publicarse(&self) -> bool {
        matches!(self.estado, EstadoEvento::Borrador)
    }

    pub fn publicar(&mut self) -> Result<(), String> {
        if !self.puede_publicarse() {
            return Err("El evento no puede ser publicado".to_string());
        }
        self.estado = EstadoEvento::Publicado;
        self.fecha_actualizacion = Utc::now();
        Ok(())
    }

    pub fn cancelar(&mut self) -> Result<(), String> {
        if matches!(self.estado, EstadoEvento::Cancelado | EstadoEvento::Finalizado) {
            return Err("El evento no puede ser cancelado".to_string());
        }
        self.estado = EstadoEvento::Cancelado;
        self.fecha_actualizacion = Utc::now();
        Ok(())
    }

    pub fn finalizar(&mut self) {
        self.estado = EstadoEvento::Finalizado;
        self.fecha_actualizacion = Utc::now();
    }

    pub fn esta_publicado(&self) -> bool {
        matches!(self.estado, EstadoEvento::Publicado)
    }

    pub fn esta_activo(&self) -> bool {
        matches!(self.estado, EstadoEvento::Publicado)
    }

    pub fn fecha_paso(&self) -> bool {
        let now = chrono::Local::now().naive_local().date();
        self.fecha < now
    }
}
