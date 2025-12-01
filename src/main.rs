use sqlx::{PgPool};
use std::env;
pub mod db;
mod handlers;
mod services;
mod middleware;

use crate::handlers::{user, auth};
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

struct AppState {
    app_name: String,
}

async fn create_db_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = create_db_pool().await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(AppState {
                app_name: "CMS".to_string(),
            }))
            .configure(user::config)
            .configure(auth::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
