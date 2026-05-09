use std::env;

pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self { secret, expiration_hours }
    }
    pub fn from_env() -> Self {
        Self {
            secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a number"),
        }
    }
}
