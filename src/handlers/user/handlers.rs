use crate::db::models::user::User;
use actix_web::{HttpResponse, Responder, web, HttpRequest, HttpMessage};
use sqlx::PgPool;
use crate::services::auth::Claims;

pub async fn get_user(uid: web::Path<String>, db_pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    let user_claims = req.extensions().get::<Claims>().cloned().unwrap();
    let user = User::find_by_id(&db_pool, user_claims.sub.parse::<i32>().unwrap()).await;
    match user {
        Ok(user) => HttpResponse::Ok().body(format!("Username: {} Email: {}", user.username, user.email)),
        Err(error) => HttpResponse::NotFound().finish(),
    }
}
