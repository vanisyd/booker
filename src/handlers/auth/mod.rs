use actix_web::{web};
use serde::{Deserialize, Serialize};
use crate::handlers::auth::handlers::auth;
use crate::handlers::auth::handlers::register;

mod handlers;

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    pub token: String
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/auth", web::post().to(auth))
            .route("/register", web::post().to(register))
    );
}