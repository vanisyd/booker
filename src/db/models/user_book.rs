use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reading_status", rename_all = "snake_case")]
pub enum ReadingStatus {
    WantToRead,
    Reading,
    Completed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserBook {
    pub id: i32,
    pub user_id: i32,
    pub book_id: i32,
    pub isbn_id: Option<i32>,
    pub status: ReadingStatus,
    pub progress: i16,
    pub current_page: Option<i32>,
    pub rating: Option<i16>,
    pub notes: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}