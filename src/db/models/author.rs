use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub photo_url: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub external_keys: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}