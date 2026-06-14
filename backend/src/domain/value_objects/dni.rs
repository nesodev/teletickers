use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dni(String);

impl Dni {
    pub fn new(value: impl Into<String>) -> AppResult<Self> {
        let value = value.into();
        Self::validate(&value)?;
        Ok(Dni(value))
    }

    fn validate(value: &str) -> AppResult<()> {
        if value.len() != 8 {
            return Err(AppError::DniInvalid("El DNI debe tener 8 dígitos".to_string()));
        }

        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::DniInvalid("El DNI solo debe contener números".to_string()));
        }

        Ok(())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for Dni {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Dni {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Dni {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
