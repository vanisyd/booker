use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub published_year: Option<i32>,
    pub page_count: Option<i32>,
    pub external_keys: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}