mod handlers;

use actix_web::{web, HttpResponse};
use crate::handlers::user::handlers::{get_user};
use crate::middleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route(
                "",
                web::get().to(|| async { HttpResponse::Ok().body("Index page") }),
            )
            .route("/{uid}", web::get().to(get_user))
            .wrap(middleware::permission::Permission)
    );
}