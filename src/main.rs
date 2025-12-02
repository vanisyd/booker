use sqlx::{PgPool};
use std::env;
pub mod db;
mod handlers;
mod services;
mod middleware;
mod config;

use crate::handlers::{user, auth};
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::config::Config;

struct AppState {
    app_name: String,
}

async fn create_db_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = Config::from_env().expect("Failed to load config");
    let db_pool = create_db_pool(&config).await.expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(AppState {
                app_name: "CMS".to_string(),
            }))
            .app_data(web::Data::new(config.clone()))
            .configure(user::config)
            .configure(auth::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
