// src/monitoring/service.rs
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::models::metrics::{ServerMetrics, ProcessMetrics};
use crate::db::Repository;

pub struct MonitoringService {
    collector: Arc<RwLock<MetricsCollector>>,
    repository: Repository,
}

impl MonitoringService {
    pub fn new(repository: Repository) -> Self {
        let collector = MetricsCollector::new();
        let service = Self {
            collector: Arc::new(RwLock::new(collector)),
            repository,
        };
        
        service.clone().start();
        service
    }

    pub fn clone(&self) -> Self {
        Self {
            collector: Arc::clone(&self.collector),
            repository: self.repository.clone(),
        }
    }

    async fn start(self) {
        let collector = self.collector.write().await;
        collector.start_collection().await;
    }

    pub async fn get_current_metrics(&self) -> Option<ServerMetrics> {
        self.collector.read().await.get_current_metrics().await
    }
    
    // Add more methods...
}