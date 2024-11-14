// server/src/api/handlers.rs

use actix_web::{web, HttpResponse, Result};
use crate::monitoring::MonitoringService;

pub async fn get_metrics(
    monitoring: web::Data<MonitoringService>,
) -> Result<HttpResponse> {
    if let Some(metrics) = monitoring.get_current_metrics().await {
        Ok(HttpResponse::Ok().json(metrics))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

pub async fn get_servers() -> Result<HttpResponse> {
    // TODO: Implement server list retrieval
    Ok(HttpResponse::Ok().json(vec![]))
}

pub async fn get_server_metrics(
    monitoring: web::Data<MonitoringService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let server_id = path.into_inner();
    if let Some(metrics) = monitoring.get_server_metrics(&server_id).await {
        Ok(HttpResponse::Ok().json(metrics))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn get_server_processes(
    monitoring: web::Data<MonitoringService>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let server_id = path.into_inner();
    if let Some(processes) = monitoring.get_server_processes(&server_id).await {
        Ok(HttpResponse::Ok().json(processes))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}