// src/api/response.rs
use serde::Serialize;
use actix_web::HttpResponse;


#[derive(Serialize)]
pub struct ApiResponse<T> 
where
    T: Serialize,
{
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            success: true,
            data: Some(data),
            error: None,
        })
    }

    pub fn error(err: &str) -> HttpResponse {
        HttpResponse::BadRequest().json(Self {
            success: false,
            data: None,
            error: Some(err.to_string()),
        })
    }
}