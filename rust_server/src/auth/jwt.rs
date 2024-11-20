// src/auth/jwt.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,    // User ID
    pub email: String,
    pub role: String,
    pub exp: i64,       // Expiration time
    pub iat: i64,       // Issued at
}

#[allow(dead_code)]
pub fn create_token(user_id: &str, email: &str, role: &str) -> Result<String, AppError> {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: expiration.timestamp(),
        iat: Utc::now().timestamp(),
    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::AuthError("JWT_SECRET not set".to_string()))?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .map_err(|e| AppError::AuthError(format!("Token creation failed: {}", e)))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::AuthError("JWT_SECRET not set".to_string()))?;

    let validation = Validation::default();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| AppError::AuthError(format!("Token verification failed: {}", e)))
}