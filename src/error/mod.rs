use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use async_graphql::ErrorExtensions;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    InvalidCredentials,
    TokenExpired,
    TokenInvalid,

    ValidationError(String),
    DniInvalid(String),
    EmailInvalid(String),

    EventNotFound,
    EventAlreadyPublished,
    EventCancelled,
    InsufficientStock,
    MaxTicketsExceeded,
    EventDatePassed,

    PurchaseNotFound,
    PaymentFailed(String),
    PaymentPending,

    DatabaseError(String),
    NotFound(String),
    AlreadyExists(String),

    ExternalServiceError(String),
    ReniecServiceError(String),
    StorageError(String),

    InternalServerError(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized(msg) => write!(f, "No autorizado: {}", msg),
            AppError::InvalidCredentials => write!(f, "Credenciales inválidas"),
            AppError::TokenExpired => write!(f, "Token expirado"),
            AppError::TokenInvalid => write!(f, "Token inválido"),

            AppError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            AppError::DniInvalid(dni) => write!(f, "DNI inválido: {}", dni),
            AppError::EmailInvalid(email) => write!(f, "Email inválido: {}", email),

            AppError::EventNotFound => write!(f, "Evento no encontrado"),
            AppError::EventAlreadyPublished => write!(f, "El evento ya está publicado"),
            AppError::EventCancelled => write!(f, "El evento está cancelado"),
            AppError::InsufficientStock => write!(f, "No hay suficientes entradas disponibles"),
            AppError::MaxTicketsExceeded => write!(f, "Excede el máximo de entradas por compra"),
            AppError::EventDatePassed => write!(f, "La fecha del evento ya pasó"),

            AppError::PurchaseNotFound => write!(f, "Compra no encontrada"),
            AppError::PaymentFailed(msg) => write!(f, "Pago fallido: {}", msg),
            AppError::PaymentPending => write!(f, "Pago pendiente de confirmación"),

            AppError::DatabaseError(msg) => write!(f, "Error de base de datos: {}", msg),
            AppError::NotFound(entity) => write!(f, "{} no encontrado", entity),
            AppError::AlreadyExists(entity) => write!(f, "{} ya existe", entity),

            AppError::ExternalServiceError(msg) => write!(f, "Error de servicio externo: {}", msg),
            AppError::ReniecServiceError(msg) => write!(f, "Error en servicio RENIEC: {}", msg),
            AppError::StorageError(msg) => write!(f, "Error de almacenamiento: {}", msg),

            AppError::InternalServerError(msg) => write!(f, "Error interno: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Petición inválida: {}", msg),
        }
    }
}

impl AppError {
    pub fn error_code(&self) -> &str {
        match self {
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::InvalidCredentials => "INVALID_CREDENTIALS",
            AppError::TokenExpired => "TOKEN_EXPIRED",
            AppError::TokenInvalid => "TOKEN_INVALID",

            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::DniInvalid(_) => "DNI_INVALID",
            AppError::EmailInvalid(_) => "EMAIL_INVALID",

            AppError::EventNotFound => "EVENT_NOT_FOUND",
            AppError::EventAlreadyPublished => "EVENT_ALREADY_PUBLISHED",
            AppError::EventCancelled => "EVENT_CANCELLED",
            AppError::InsufficientStock => "INSUFFICIENT_STOCK",
            AppError::MaxTicketsExceeded => "MAX_TICKETS_EXCEEDED",
            AppError::EventDatePassed => "EVENT_DATE_PASSED",

            AppError::PurchaseNotFound => "PURCHASE_NOT_FOUND",
            AppError::PaymentFailed(_) => "PAYMENT_FAILED",
            AppError::PaymentPending => "PAYMENT_PENDING",

            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::AlreadyExists(_) => "ALREADY_EXISTS",

            AppError::ExternalServiceError(_) => "EXTERNAL_SERVICE_ERROR",
            AppError::ReniecServiceError(_) => "RENIEC_ERROR",
            AppError::StorageError(_) => "STORAGE_ERROR",

            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
        }
    }

    pub fn to_gql(self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string()).extend_with(|_, e| e.set("code", self.error_code()))
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized(_) | AppError::InvalidCredentials | AppError::TokenExpired | AppError::TokenInvalid => StatusCode::UNAUTHORIZED,

            AppError::ValidationError(_) | AppError::DniInvalid(_) | AppError::EmailInvalid(_) | AppError::BadRequest(_) | AppError::MaxTicketsExceeded | AppError::EventDatePassed => {
                StatusCode::BAD_REQUEST
            }

            AppError::EventNotFound | AppError::PurchaseNotFound | AppError::NotFound(_) => StatusCode::NOT_FOUND,

            AppError::EventAlreadyPublished | AppError::AlreadyExists(_) => StatusCode::CONFLICT,

            AppError::InsufficientStock => StatusCode::UNPROCESSABLE_ENTITY,

            AppError::EventCancelled => StatusCode::GONE,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            code: self.error_code().to_string(),
            message: self.to_string(),
            details: None,
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
}

// Conversión desde SeaORM
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(_) => AppError::NotFound("Registro".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
