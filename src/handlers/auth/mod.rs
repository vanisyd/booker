use actix_web::{web};
use serde::{Deserialize, Serialize};
use crate::handlers::auth::handlers::auth;
use crate::handlers::auth::handlers::register;

mod handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/auth", web::post().to(auth))
            .route("/register", web::post().to(register))
    );
}