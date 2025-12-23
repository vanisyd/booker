use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::{services, AppState};
use crate::config::Config;
use crate::db::models::user::UserError;
use booker_api::auth::{CreateUserRequest, LoginUserRequest, LoginUserResponse};

pub async fn auth(
    web::Form(form): web::Form<LoginUserRequest>,
    db_pool: web::Data<PgPool>,
    config: web::Data<Config>
) -> impl Responder {
    match services::auth::authorize(form, &db_pool).await {
        Ok(user) => {
            let token = services::auth::get_token(&user, &config);
            HttpResponse::Ok().json(LoginUserResponse {
                token
            })
        },
        Err(error) => HttpResponse::Unauthorized().body(error)
    }
}

pub async fn register(
    web::Form(form): web::Form<CreateUserRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let res = services::auth::register(form, &db_pool).await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) if !matches!(e, UserError::Unknown(_)) => {
            HttpResponse::BadRequest().body(e.to_string())
        },
        _ => HttpResponse::InternalServerError().finish()
    }
}