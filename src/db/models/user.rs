use log::{error, warn};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use sqlx::types::time::OffsetDateTime;
use thiserror::Error;
use sqlx::Error as SqlxError;
use crate::db::pg_errors::PgErrorExt;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("email already exists")]
    EmailExists,
    #[error("username already exists")]
    UsernameExists,
    #[error("{0}")]
    Unknown(String)
}

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
    pub async fn create(pool: &PgPool, data: CreateUser) -> Result<User, UserError> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
            data.username, data.email, data.password_hash
        ).fetch_one(pool)
        .await;

        match user {
            Ok(user) => Ok(user),
            Err(e) if e.is_unique_violation() => {
                if e.constraint_name_like("username") {
                    Err(UserError::UsernameExists)
                } else if e.constraint_name_like("email") {
                    Err(UserError::EmailExists)
                } else {
                    error!("Unexpected unique violation: {}", e);
                    Err(UserError::Unknown(e.to_string()))
                }
            },
            Err(e) => {
                error!("Unexpected error when create user: {}", e);
                Err(UserError::Unknown(e.to_string()))
            }
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