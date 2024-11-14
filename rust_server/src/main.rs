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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // 로깅 설정
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // 데이터베이스 연결 테스트
    db::test_connection(&database_url)
        .await
        .expect("Failed to connect to database");


    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database pool
    let db_pool = db::create_pool()
        .await
        .expect("Failed to create database pool");
    
    // Create repository
    let repository = db::repository::Repository::new(db_pool.clone());
    
    // Initialize monitoring service
    let monitoring_service = monitoring::MonitoringService::new(repository.clone());
    monitoring_service.start().await;

    // Create WebSocket server
    let ws_server = websocket::WebSocketServer::new();

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(auth::AuthenticationMiddleware)
            .app_data(web::Data::new(repository.clone()))
            .app_data(web::Data::new(monitoring_service.clone()))
            .app_data(web::Data::new(ws_server.clone()))
            .configure(api::configure_routes)
            .configure(websocket::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

