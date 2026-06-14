use std::env;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
            frontend_url: env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
