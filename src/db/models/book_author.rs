use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookAuthor {
    pub id: i32,
    pub book_id: i32,
    pub author_id: i32,
    pub author_order: i16,
    pub created_at: DateTime<Utc>,
}