// src/tests/integration/api.rs
use actix_web::{test, web, App};
use crate::api;
use crate::db::Repository;
use crate::monitoring::MonitoringService;

#[actix_web::test]
async fn test_get_metrics() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Repository::new_mock()))
            .app_data(web::Data::new(MonitoringService::new_mock()))
            .configure(api::configure_routes)
    ).await;

    let req = test::TestRequest::get().uri("/api/v1/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
}