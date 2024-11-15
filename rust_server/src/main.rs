// src/main.rs
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, FmtSubscriber};

use rust_server::{
    api,
    auth::AuthenticationMiddleware,
    db,
    monitoring::MonitoringService,
    websocket::{self, WebSocketServer},
};
use crate::error::AppError;

mod api;
mod auth;
mod db;
mod models;
mod monitoring;
mod websocket;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Environment setup
    dotenv().ok();
    setup_logging();
    
    // Database setup
    let db_pool = setup_database().await?;
    let repository = Repository::new(db_pool);
    
    // Services setup
    let monitoring_service = MonitoringService::new(repository.clone());
    let ws_server = WebSocketServer::new();

    // Server setup
    HttpServer::new(move || {
        App::new()
            .wrap(setup_cors())
            .wrap(middleware::Logger::default())
            .wrap(AuthenticationMiddleware)
            .app_data(web::Data::new(repository.clone()))
            .app_data(web::Data::new(monitoring_service.clone()))
            .app_data(web::Data::new(ws_server.clone()))
            .configure(api::configure_routes)
            .configure(websocket::configure_routes)
    })
    .bind("0.0.0.0:8080")?  // Changed from 127.0.0.1 to 0.0.0.0 for Docker
    .run()
    .await
}



fn setup_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}

async fn setup_database() -> Result<DbPool, AppError> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    db::test_connection(&database_url).await?;
    db::create_pool().await.map_err(AppError::from)
}

fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}