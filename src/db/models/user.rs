use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub updated_at: Option<OffsetDateTime>,
    pub created_at: Option<OffsetDateTime>
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String
}
impl User {
    pub async fn create(pool: &PgPool, data: CreateUser) -> String {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
            data.username, data.email, data.password_hash
        ).fetch_one(pool).await;

        match user {
            Ok(user) => format!("User id {} name {} created", user.id, user.username),
            Err(error) => format!("{}", error)
        }
    }

    pub async fn find_by_email(db_pool: &PgPool, email: String) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(db_pool).await
    }

    pub async fn find_by_id(db_pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(db_pool).await
    }
}