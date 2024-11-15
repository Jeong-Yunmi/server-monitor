// src/websocket/mod.rs
mod handlers;
mod server;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[derive(Clone)]
pub struct WebSocketServer {}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    monitoring_service: web::Data<crate::monitoring::MonitoringService>,
) -> Result<HttpResponse, Error> {
    let ws = server::WebSocketConnection::new(monitoring_service);
    ws::start(ws, &req, stream)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
}

