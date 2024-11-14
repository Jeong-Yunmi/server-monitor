// src/error.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database error occurred",
                    "message": self.to_string()
                }))
            }
            AppError::AuthError(_) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Authentication failed",
                    "message": self.to_string()
                }))
            }
            AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation failed",
                    "message": self.to_string()
                }))
            }
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(json!({
                    "error": "Resource not found",
                    "message": self.to_string()
                }))
            }
            AppError::InternalError(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error",
                    "message": self.to_string()
                }))
            }
        }
    }
}