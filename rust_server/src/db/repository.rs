// server/src/db/repository.rs

use chrono::{DateTime, Utc};
use super::models::*;
use super::DbPool;
use anyhow::Result;

pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_server(&self, server: Server) -> Result<Server> {
        let result = sqlx::query_as!(
            Server,
            r#"
            INSERT INTO servers 
            (id, name, hostname, ip_address, location, server_type, is_online, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id as "id!", 
                      name as "name!", 
                      hostname as "hostname!", 
                      ip_address as "ip_address!", 
                      location as "location!", 
                      server_type as "server_type!", 
                      is_online as "is_online!", 
                      created_at as "created_at!", 
                      updated_at as "updated_at!"
            "#,
            server.id,
            server.name,
            server.hostname,
            server.ip_address,
            server.location,
            server.server_type,
            server.is_online,
            server.created_at,
            server.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
    
    pub async fn get_server(&self, id: &str) -> Result<Option<Server>> {
        let result = sqlx::query_as!(
            Server,
            r#"
            SELECT id as "id!", 
                   name as "name!", 
                   hostname as "hostname!", 
                   ip_address as "ip_address!", 
                   location as "location!", 
                   server_type as "server_type!",
                   is_online as "is_online!",
                   created_at as "created_at!",
                   updated_at as "updated_at!"
            FROM servers WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn list_servers(&self) -> Result<Vec<Server>> {
        let results = sqlx::query_as!(
            Server,
            r#"
            SELECT id as "id!", 
                   name as "name!", 
                   hostname as "hostname!", 
                   ip_address as "ip_address!", 
                   location as "location!", 
                   server_type as "server_type!",
                   is_online as "is_online!",
                   created_at as "created_at!",
                   updated_at as "updated_at!"
            FROM servers 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn update_server_status(&self, id: &str, is_online: bool) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE servers
            SET is_online = $1, updated_at = $2
            WHERE id = $3
            "#,
            is_online,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_metrics(&self, snapshot: MetricsSnapshot) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO metrics_snapshots 
            (server_id, cpu_usage, memory_usage, disk_usage, network_rx, network_tx, processes, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            snapshot.server_id,
            snapshot.cpu_usage,
            snapshot.memory_usage,
            snapshot.disk_usage,
            snapshot.network_rx,
            snapshot.network_tx,
            snapshot.processes,
            snapshot.timestamp
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }

    pub async fn get_server_metrics(
        &self,
        server_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<MetricsSnapshot>> {
        let results = sqlx::query_as!(
            MetricsSnapshot,
            r#"
            SELECT id as "id!",
                   server_id as "server_id!", 
                   cpu_usage as "cpu_usage!", 
                   memory_usage as "memory_usage!", 
                   disk_usage as "disk_usage!",
                   network_rx as "network_rx!", 
                   network_tx as "network_tx!",
                   processes as "processes!",
                   timestamp as "timestamp!"
            FROM metrics_snapshots
            WHERE server_id = $1 AND timestamp BETWEEN $2 AND $3
            ORDER BY timestamp DESC
            "#,
            server_id,
            from,
            to
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn create_alert(&self, alert: Alert) -> Result<Alert> {
        let result = sqlx::query_as!(
            Alert,
            r#"
            INSERT INTO alerts 
            (server_id, alert_type, severity, message, created_at, acknowledged_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id as "id!", 
                      server_id as "server_id!", 
                      alert_type as "alert_type!", 
                      severity as "severity!", 
                      message as "message!",
                      created_at as "created_at!",
                      acknowledged_at
            "#,
            alert.server_id,
            alert.alert_type,
            alert.severity,
            alert.message,
            alert.created_at,
            alert.acknowledged_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_unacknowledged_alerts(&self) -> Result<Vec<Alert>> {
        let results = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", 
                   server_id as "server_id!", 
                   alert_type as "alert_type!", 
                   severity as "severity!", 
                   message as "message!",
                   created_at as "created_at!",
                   acknowledged_at
            FROM alerts
            WHERE acknowledged_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn acknowledge_alert(&self, alert_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE alerts
            SET acknowledged_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            alert_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users 
            (id, email, password_hash, name, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id as "id!", 
                      email as "email!", 
                      password_hash as "password_hash!", 
                      name as "name!", 
                      role as "role!",
                      created_at as "created_at!",
                      updated_at as "updated_at!"
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.name,
            user.role,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let result = sqlx::query_as!(
            User,
            r#"
            SELECT id as "id!", 
                   email as "email!", 
                   password_hash as "password_hash!", 
                   name as "name!", 
                   role as "role!",
                   created_at as "created_at!",
                   updated_at as "updated_at!"
            FROM users 
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }
}