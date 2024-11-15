#[cfg(test)]
mod tests {
    use crate::db::{create_pool, test_connection};
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_database_connection() {
        dotenv().ok();
        
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
            
        // Test connection
        assert!(test_connection(&database_url).await.is_ok());
        
        // Test pool creation
        let pool = create_pool().await;
        assert!(pool.is_ok());
        
        // Test simple query
        let pool = pool.unwrap();
        let result = sqlx::query!("SELECT 1 as one")
            .fetch_one(&pool)
            .await;
            
        assert!(result.is_ok());
        assert_eq!(result.unwrap().one, Some(1));
    }
}