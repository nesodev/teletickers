use crate::error::{AppError, AppResult};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SunatResponse {
    pub ruc: String,
    pub razon_social: String,
    pub estado: String,
    pub condicion: String,
    pub direccion: String,
}

pub struct SunatClient {
    api_url: String,
    api_token: String,
    client: Client,
}

impl SunatClient {
    pub fn new(api_url: String, api_token: String) -> Self {
        Self {
            api_url,
            api_token,
            client: Client::new(),
        }
    }

    pub async fn validate_ruc(&self, ruc: &str) -> AppResult<SunatResponse> {
        let url = format!("{}/ruc/{}", self.api_url, ruc);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(format!("SUNAT request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ExternalServiceError(format!("SUNAT API returned status: {}", response.status())));
        }

        response
            .json::<SunatResponse>()
            .await
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse SUNAT response: {}", e)))
    }
}
