// server/src/db/mod.rs

pub mod models;
pub mod repository;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> anyhow::Result<DbPool> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 데이터베이스 마이그레이션 실행
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
pub async fn test_connection(database_url: &str) -> anyhow::Result<()> {
    info!("Testing database connection...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // 간단한 쿼리 실행으로 연결 테스트
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await?;

    info!("Database connection successful!");
    Ok(())
}