// src/lib.rs

pub mod api;
pub mod auth;
pub mod db;
pub mod error;
pub mod monitoring;
pub mod websocket;

pub use error::AppError;