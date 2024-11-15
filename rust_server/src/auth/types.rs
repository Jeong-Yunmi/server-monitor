// src/auth/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // User ID
    pub email: String,
    pub role: String,
    pub exp: i64,       // Expiration time
    pub iat: i64,       // Issued at
}