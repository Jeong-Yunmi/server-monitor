// src/api/servers.rs
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use uuid::Uuid;
use crate::db::{models::Server, repository::Repository};

pub async fn create_server(
    repo: web::Data<Repository>,
    server_info: web::Json<CreateServerRequest>,
) -> Result<HttpResponse> {
    let server = Server {
        id: Uuid::new_v4().to_string(),
        name: server_info.name.clone(),
        hostname: server_info.hostname.clone(),
        ip_address: server_info.ip_address.clone(),
        location: server_info.location.clone(),
        server_type: server_info.server_type.clone(),
        is_online: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let result = repo.create_server(server).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(result))
}

pub async fn get_servers(
    repo: web::Data<Repository>,
) -> Result<HttpResponse> {
    let servers = repo.list_servers().await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(servers))
}

pub async fn get_server(
    repo: web::Data<Repository>,
    server_id: web::Path<String>,
) -> Result<HttpResponse> {
    let server = repo.get_server(&server_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match server {
        Some(server) => Ok(HttpResponse::Ok().json(server)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn update_server_status(
    repo: web::Data<Repository>,
    server_id: web::Path<String>,
    status: web::Json<UpdateServerStatusRequest>,
) -> Result<HttpResponse> {
    repo.update_server_status(&server_id, status.is_online).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub location: String,
    pub server_type: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateServerStatusRequest {
    pub is_online: bool,
}