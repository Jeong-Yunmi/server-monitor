// server/src/db/models.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
//use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub location: String,
    pub server_type: String,
    pub is_online: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MetricsSnapshot {
    pub id: i64,
    pub server_id: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_rx: i64,
    pub network_tx: i64,
    pub processes: JsonValue,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Alert {
    pub id: i64,
    pub server_id: String,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}