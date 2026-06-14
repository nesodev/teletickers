use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoNotificacion {
    Compra,
    Evento,
    Recordatorio,
    Sistema,
}

impl TipoNotificacion {
    pub fn as_str(&self) -> &str {
        match self {
            TipoNotificacion::Compra => "compra",
            TipoNotificacion::Evento => "evento",
            TipoNotificacion::Recordatorio => "recordatorio",
            TipoNotificacion::Sistema => "sistema",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "compra" => Some(TipoNotificacion::Compra),
            "evento" => Some(TipoNotificacion::Evento),
            "recordatorio" => Some(TipoNotificacion::Recordatorio),
            "sistema" => Some(TipoNotificacion::Sistema),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notificacion {
    pub id: Uuid,
    pub usuario_id: Uuid,
    pub tipo: TipoNotificacion,
    pub titulo: String,
    pub mensaje: String,
    pub leido: bool,
    pub fecha_creacion: DateTime<Utc>,
    pub evento_id: Option<Uuid>,
}

impl Notificacion {
    pub fn new(usuario_id: Uuid, tipo: TipoNotificacion, titulo: String, mensaje: String, evento_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            usuario_id,
            tipo,
            titulo,
            mensaje,
            leido: false,
            fecha_creacion: Utc::now(),
            evento_id,
        }
    }

    pub fn marcar_como_leido(&mut self) {
        self.leido = true;
    }

    pub fn esta_leido(&self) -> bool {
        self.leido
    }
}

// Constructores específicos para diferentes tipos de notificaciones
impl Notificacion {
    pub fn compra_exitosa(usuario_id: Uuid, evento_id: Uuid, nombre_evento: &str) -> Self {
        Self::new(
            usuario_id,
            TipoNotificacion::Compra,
            "Compra exitosa".to_string(),
            format!("Tu compra para el evento '{}' ha sido confirmada", nombre_evento),
            Some(evento_id),
        )
    }

    pub fn recordatorio_evento(usuario_id: Uuid, evento_id: Uuid, nombre_evento: &str, dias: i32) -> Self {
        Self::new(
            usuario_id,
            TipoNotificacion::Recordatorio,
            "Recordatorio de evento".to_string(),
            format!("El evento '{}' será en {} días", nombre_evento, dias),
            Some(evento_id),
        )
    }

    pub fn evento_cancelado(usuario_id: Uuid, evento_id: Uuid, nombre_evento: &str) -> Self {
        Self::new(
            usuario_id,
            TipoNotificacion::Evento,
            "Evento cancelado".to_string(),
            format!("El evento '{}' ha sido cancelado. Se procesará tu reembolso", nombre_evento),
            Some(evento_id),
        )
    }

    pub fn sistema(usuario_id: Uuid, titulo: String, mensaje: String) -> Self {
        Self::new(usuario_id, TipoNotificacion::Sistema, titulo, mensaje, None)
    }
}
