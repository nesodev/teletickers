use crate::error::{AppError, AppResult};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

pub struct PasswordHasherService;

impl PasswordHasherService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalServerError(format!("Error hashing password: {}", e)))?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::InternalServerError(format!("Invalid hash format: {}", e)))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
