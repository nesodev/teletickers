use std::env;

pub struct PaymentConfig {
    pub yape_api_key: String,
    pub plin_api_key: String,
    pub stripe_secret_key: String,
    pub webhook_secret: String,
    pub callback_url: String,
}

impl PaymentConfig {
    pub fn from_env() -> Self {
        Self {
            yape_api_key: env::var("YAPE_API_KEY").unwrap_or_default(),
            plin_api_key: env::var("PLIN_API_KEY").unwrap_or_default(),
            stripe_secret_key: env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set"),
            webhook_secret: env::var("PAYMENT_WEBHOOK_SECRET").expect("PAYMENT_WEBHOOK_SECRET must be set"),
            callback_url: env::var("PAYMENT_CALLBACK_URL").expect("PAYMENT_CALLBACK_URL must be set"),
        }
    }
}
