// src/api/health.rs
use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::db::Repository;

#[derive(Serialize)]
pub struct HealthStatus {
    status: String,
    database: bool,
    version: String,
    uptime: u64,
}

pub async fn health_check(repo: web::Data<Repository>) -> HttpResponse {
    let db_status = repo.check_connection().await.is_ok();

    HttpResponse::Ok().json(HealthStatus {
        status: "ok".to_string(),
        database: db_status,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}