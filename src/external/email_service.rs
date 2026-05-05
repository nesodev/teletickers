use crate::config::NotificacionConfig;
use crate::error::{AppError, AppResult};
use crate::ports::NotificationPort;
use async_trait::async_trait;
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType, transport::smtp::authentication::Credentials};

pub struct EmailService {
    config: NotificacionConfig,
}

impl EmailService {
    pub fn new(config: NotificacionConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl NotificationPort for EmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> AppResult<()> {
        let email = Message::builder()
            .from(
                format!("{} <{}>", self.config.from_name, self.config.from_email)
                    .parse()
                    .map_err(|e| AppError::InternalServerError(format!("Invalid from address: {}", e)))?,
            )
            .to(to.parse().map_err(|e| AppError::ValidationError(format!("Invalid to address: {}", e)))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| AppError::InternalServerError(format!("Failed to build email: {}", e)))?;

        let creds = Credentials::new(self.config.smtp_username.clone(), self.config.smtp_password.clone());

        let mailer = SmtpTransport::relay(&self.config.smtp_host)
            .map_err(|e| AppError::ExternalServiceError(format!("SMTP connection failed: {}", e)))?
            .credentials(creds)
            .build();

        mailer.send(&email).map_err(|e| AppError::ExternalServiceError(format!("Email sending failed: {}", e)))?;

        Ok(())
    }

    async fn send_sms(&self, _phone: &str, _message: &str) -> AppResult<()> {
        Err(AppError::InternalServerError("SMS service not implemented".to_string()))
    }
}
