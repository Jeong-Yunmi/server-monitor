// src/lib.rs

pub mod api;
pub mod auth;
pub mod db;
pub mod models;
pub mod error;
pub mod monitoring;
pub mod websocket;

pub use error::AppError;

// API 응답 타입 지정
pub type ApiResponse<T> = crate::api::response::ApiResponse<T>;