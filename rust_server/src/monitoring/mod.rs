// src/monitoring/mod.rs
mod collector;
mod service;

use collector::{MetricsCollector, SystemMetrics};
use std::sync::Arc;
use tokio::sync::RwLock;
//pub use service::*;

#[derive(Clone)]
pub struct MonitoringService {
    collector: Arc<RwLock<MetricsCollector>>,
}

impl MonitoringService {
    pub fn new() -> Self {
        let collector = MetricsCollector::new();
        let service = Self {
            collector: Arc::new(RwLock::new(collector)),
        };
        
        // Start collecting metrics
        service.clone().start();
        service
    }

    // fn clone(&self) -> Self {
    //     Self {
    //         collector: self.collector.clone(),
    //     }
    // } // clone 함수 일단 추가

    fn start(self) {
        tokio::spawn(async move {
            let collector = self.collector.read().await;
            collector.start_collection().await;
        });
    }

    pub async fn get_current_metrics(&self) -> Option<SystemMetrics> {
        self.collector.read().await.get_current_metrics().await
    }

    pub async fn get_server_metrics(&self, _server_id: &str) -> Option<SystemMetrics> {
        self.get_current_metrics().await
    }

    pub async fn get_server_processes(&self, _server_id: &str) -> Option<Vec<collector::ProcessInfo>> {
        self.collector
            .read()
            .await
            .get_current_metrics()
            .await
            .map(|m| m.processes)
    }
}