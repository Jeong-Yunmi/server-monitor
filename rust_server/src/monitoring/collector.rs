// server/src/monitoring/collector.rs

use sysinfo::{CpuExt, NetworkExt, PidExt, ProcessExt, System, SystemExt};
use tokio::time::{self, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::iter::Iterator;
use sysinfo::DiskExt;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_rx: u64,
    pub network_tx: u64,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

pub struct MetricsCollector {
    system: Arc<Mutex<System>>,
    last_metrics: Arc<Mutex<Option<SystemMetrics>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system: Arc::new(Mutex::new(system)),
            last_metrics: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_collection(&self) {
        let system = self.system.clone();
        let last_metrics = self.last_metrics.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                let mut system = system.lock().await;
                system.refresh_all();

                let metrics = SystemMetrics {
                    cpu_usage: system.global_cpu_info().cpu_usage(),
                    memory_usage: (system.used_memory() as f32 / system.total_memory() as f32) * 100.0,
                    disk_usage: Self::calculate_disk_usage(&system),
                    network_rx: system.networks().into_iter().map(|(_, data)| data.received()).sum(),
                    network_tx: system.networks().into_iter().map(|(_, data)| data.transmitted()).sum(),
                    processes: Self::collect_processes(&system),
                };

                *last_metrics.lock().await = Some(metrics);
            }
        });
    }

    fn calculate_disk_usage(system: &System) -> f32 {
        let disks = system.disks();
        if disks.is_empty() {
            return 0.0;
        }

        let total: u64 = disks.iter().map(|disk| disk.total_space()).sum();
        let used: u64 = disks.iter().map(|disk| disk.total_space() - disk.available_space()).sum();
        
        (used as f32 / total as f32) * 100.0
    }

    fn collect_processes(system: &System) -> Vec<ProcessInfo> {
        system.processes()
            .iter()
            .take(10) // Top 10 processes by CPU usage
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
            })
            .collect()
    }

    pub async fn get_current_metrics(&self) -> Option<SystemMetrics> {
        self.last_metrics.lock().await.clone()
    }
}