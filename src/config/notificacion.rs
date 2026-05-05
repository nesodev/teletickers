use std::env;

pub struct NotificacionConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

impl NotificacionConfig {
    pub fn from_env() -> Self {
        Self {
            smtp_host: env::var("SMTP_HOST").expect("SMTP_HOST must be set"),
            smtp_port: env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string()).parse().expect("SMTP_PORT must be a number"),
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            from_email: env::var("FROM_EMAIL").expect("FROM_EMAIL must be set"),
            from_name: env::var("FROM_NAME").unwrap_or_else(|_| "Ticky".to_string()),
        }
    }
}
