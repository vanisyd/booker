use crate::db::models::user::{CreateUser, User};
use crate::handlers::auth::{CreateUserRequest, LoginUserRequest};
use argon2;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header, encode, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::ops::Add;

const TOKEN_LIFETIME: i64 = 3600;

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub async fn register(data: CreateUserRequest, db_pool: &PgPool) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    User::create(db_pool, CreateUser {
            username: data.username,
            email: data.email,
            password_hash,
        },
    )
    .await
}

pub async fn authorize(data: LoginUserRequest, db_pool: &PgPool) -> Result<User, String> {
    let user = User::find_by_email(db_pool, data.email)
        .await
        .map_err(|_| "User not found")?;
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default().verify_password(data.password.as_bytes(), &parsed_hash).is_err() {
        return Err("Wrong password".to_string());
    }

    Ok(user)
}

pub fn get_token(user: &User) -> String {
    let claims = Claims {
        sub: user.id.to_string(),
        exp: Utc::now().add(TimeDelta::seconds(TOKEN_LIFETIME)).timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
        .unwrap()
}

pub fn decode_token(token: String) -> Claims {
    decode(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default()
    ).unwrap().claims
}
