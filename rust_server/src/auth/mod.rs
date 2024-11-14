// src/auth/mod.rs
mod jwt;
mod middleware;
mod error;

pub use jwt::{create_token, verify_token, Claims};
pub use middleware::AuthenticationMiddleware;