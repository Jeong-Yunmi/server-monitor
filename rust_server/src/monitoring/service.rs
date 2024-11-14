// src/monitoring/service.rs
use crate::db::{repository::Repository, models::{Alert, MetricsSnapshot}};
use chrono::{DateTime, Utc};
use sqlx::types::JsonValue;
use std::collections::HashMap;
use tokio::time::{self, Duration};
use crate::monitoring::collector::{SystemMetrics, MetricsCollector};

pub struct MonitoringService {
    collector: MetricsCollector,
    repository: Repository,
    alert_thresholds: HashMap<String, AlertThresholds>,
}

#[derive(Debug, Clone)]
struct AlertThresholds {
    cpu_warning: f32,
    cpu_critical: f32,
    memory_warning: f32,
    memory_critical: f32,
    disk_warning: f32,
    disk_critical: f32,
}

impl MonitoringService {
    pub fn new(repository: Repository) -> Self {
        Self {
            collector: MetricsCollector::new(),
            repository,
            alert_thresholds: HashMap::new(),
        }
    }

    pub async fn start(&self) {
        let repo = self.repository.clone();
        let collector = self.collector.clone();
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                
                if let Some(metrics) = collector.get_current_metrics().await {
                    // Get all servers
                    if let Ok(servers) = repo.list_servers().await {
                        for server in servers {
                            // Save metrics for each server
                            let snapshot = MetricsSnapshot {
                                id: 0, // Auto-generated
                                server_id: server.id.clone(),
                                cpu_usage: metrics.cpu_usage,
                                memory_usage: metrics.memory_usage,
                                disk_usage: metrics.disk_usage,
                                network_rx: metrics.network_rx as i64,
                                network_tx: metrics.network_tx as i64,
                                processes: serde_json::to_value(&metrics.processes).unwrap(),
                                timestamp: Utc::now(),
                            };

                            if let Ok(_) = repo.save_metrics(snapshot).await {
                                // Check for alerts
                                Self::check_alerts(&repo, &server.id, &metrics).await;
                            }
                        }
                    }
                }
            }
        });
    }

    async fn check_alerts(repo: &Repository, server_id: &str, metrics: &SystemMetrics) {
        // CPU Usage Alert
        if metrics.cpu_usage > 90.0 {
            let alert = Alert {
                id: 0, // Auto-generated
                server_id: server_id.to_string(),
                alert_type: "cpu_usage".to_string(),
                severity: "critical".to_string(),
                message: format!("CPU usage is critically high: {:.2}%", metrics.cpu_usage),
                created_at: Utc::now(),
                acknowledged_at: None,
            };
            let _ = repo.create_alert(alert).await;
        }

        // Memory Usage Alert
        if metrics.memory_usage > 90.0 {
            let alert = Alert {
                id: 0,
                server_id: server_id.to_string(),
                alert_type: "memory_usage".to_string(),
                severity: "critical".to_string(),
                message: format!("Memory usage is critically high: {:.2}%", metrics.memory_usage),
                created_at: Utc::now(),
                acknowledged_at: None,
            };
            let _ = repo.create_alert(alert).await;
        }

        // Disk Usage Alert
        if metrics.disk_usage > 90.0 {
            let alert = Alert {
                id: 0,
                server_id: server_id.to_string(),
                alert_type: "disk_usage".to_string(),
                severity: "critical".to_string(),
                message: format!("Disk usage is critically high: {:.2}%", metrics.disk_usage),
                created_at: Utc::now(),
                acknowledged_at: None,
            };
            let _ = repo.create_alert(alert).await;
        }
    }

    // Add methods for fetching historical data and analysis
    pub async fn get_server_metrics_history(
        &self,
        server_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<MetricsSnapshot>, anyhow::Error> {
        self.repository.get_server_metrics(server_id, from, to).await
    }
}