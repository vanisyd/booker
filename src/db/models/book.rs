use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub published_year: Option<i32>,
    pub page_count: Option<i32>,
    pub external_keys: Option<JsonValue>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

pub struct CreateBook {
    pub title: String,
    pub cover_url: Option<String>,
    pub published_year: Option<i32>,
    pub page_count: Option<i32>,
    pub external_keys: Option<JsonValue>,
}

impl Book {
    pub async fn create(db_pool: &PgPool, data: CreateBook) {

    }

    pub async fn find_by_title(db_pool: &PgPool, title: &String) -> Result<Vec<Book>, sqlx::Error> {
        let books = sqlx::query_as!(
            Book,
            "SELECT * FROM books WHERE title LIKE $1",
            title
        ).fetch_all(db_pool).await;

        books
    }

    pub async fn list(db_pool: &PgPool) {

    }
}