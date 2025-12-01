use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Isbn {
    pub id: i32,
    pub isbn: String,
    pub book_id: i32,
    pub isbn_type: Option<String>,
    pub created_at: DateTime<Utc>,
}