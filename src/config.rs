use std::env;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_lifetime_seconds: i64
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL not set")?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or("secret".to_string()),
            jwt_lifetime_seconds: env::var("JWT_LIFETIME_SECONDS")
                .unwrap_or("3600".to_string())
                .parse()
                .map_err(|_| "Failed to parse JWT lifetime")?,
        })
    }
}