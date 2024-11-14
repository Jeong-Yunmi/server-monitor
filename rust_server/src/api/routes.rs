// server/src/api/routes.rs

use actix_web::web;
use super::servers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/servers")
                    .route("", web::post().to(servers::create_server))
                    .route("", web::get().to(servers::get_servers))
                    .route("/{id}", web::get().to(servers::get_server))
                    .route("/{id}/status", web::put().to(servers::update_server_status))
            )
    );
}