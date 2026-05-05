use crate::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReniecResponse {
    pub dni: String,
    pub nombres: String,
    pub apellido_paterno: String,
    pub apellido_materno: String,
}

pub struct ReniecClient {
    api_url: String,
    api_token: String,
    client: Client,
}

impl ReniecClient {
    pub fn new(api_url: String, api_token: String) -> Self {
        Self {
            api_url,
            api_token,
            client: Client::new(),
        }
    }

    pub async fn validate_dni(&self, dni: &str) -> AppResult<ReniecResponse> {
        let url = format!("{}/dni/{}", self.api_url, dni);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await
            .map_err(|e| AppError::ReniecServiceError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ReniecServiceError(format!("API returned status: {}", response.status())));
        }

        response
            .json::<ReniecResponse>()
            .await
            .map_err(|e| AppError::ReniecServiceError(format!("Failed to parse response: {}", e)))
    }
}
