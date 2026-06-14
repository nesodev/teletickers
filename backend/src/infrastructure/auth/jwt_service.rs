use crate::config::JwtConfig;
use crate::error::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user_id: Uuid) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.config.secret.as_bytes())).map_err(|e| AppError::InternalServerError(format!("Error generating token: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(token, &DecodingKey::from_secret(self.config.secret.as_bytes()), &Validation::default()).map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::TokenInvalid,
        })?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id(&self, token: &str) -> AppResult<Uuid> {
        let claims = self.validate_token(token)?;
        Uuid::parse_str(&claims.sub).map_err(|_| AppError::TokenInvalid)
    }
}
