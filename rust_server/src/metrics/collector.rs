// src/metrics/collector.rs
use prometheus::{
    register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram,
};

pub struct MetricsCollector {
    pub http_requests_total: Counter,
    pub http_request_duration_seconds: Histogram,
    pub connected_clients: Gauge,
    pub system_memory_usage: Gauge,
    pub system_cpu_usage: Gauge,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let http_requests_total = register_counter!(
            "http_requests_total",
            "Total number of HTTP requests made."
        )
        .unwrap();

        let http_request_duration_seconds = register_histogram!(
            "http_request_duration_seconds",
            "HTTP request duration in seconds.",
            vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
        )
        .unwrap();

        let connected_clients = register_gauge!(
            "connected_clients",
            "Number of currently connected WebSocket clients"
        )
        .unwrap();

        let system_memory_usage = register_gauge!(
            "system_memory_usage",
            "Current system memory usage in percentage"
        )
        .unwrap();

        let system_cpu_usage = register_gauge!(
            "system_cpu_usage",
            "Current system CPU usage in percentage"
        )
        .unwrap();

        Self {
            http_requests_total,
            http_request_duration_seconds,
            connected_clients,
            system_memory_usage,
            system_cpu_usage,
        }
    }
}