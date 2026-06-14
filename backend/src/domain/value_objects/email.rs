use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> AppResult<Self> {
        let value = value.into();
        Self::validate(&value)?;
        Ok(Email(value.to_lowercase()))
    }

    fn validate(value: &str) -> AppResult<()> {
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();

        if !email_regex.is_match(value) {
            return Err(AppError::EmailInvalid("Formato de email inválido".to_string()));
        }

        if value.len() > 255 {
            return Err(AppError::EmailInvalid("El email es demasiado largo".to_string()));
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

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Email {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
