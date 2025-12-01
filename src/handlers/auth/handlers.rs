use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::{services, AppState};
use crate::handlers::auth::{CreateUserRequest, LoginUserRequest, LoginUserResponse};

pub async fn auth(
    web::Form(form): web::Form<LoginUserRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    match services::auth::authorize(form, &db_pool).await {
        Ok(user) => {
            let token = services::auth::get_token(&user);
            HttpResponse::Ok().json(LoginUserResponse {
                token
            })
        },
        Err(error) => HttpResponse::Unauthorized().body(error)
    }
}

pub async fn register(
    web::Form(form): web::Form<CreateUserRequest>,
    data: web::Data<AppState>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let res = services::auth::register(form, &db_pool).await;
    HttpResponse::Ok().body(res)
}