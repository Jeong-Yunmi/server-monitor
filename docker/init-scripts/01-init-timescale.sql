-- init-scripts/01-init-timescale.sql
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create hypertable for metrics
SELECT create_hypertable('metrics_snapshots', 'timestamp',
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- Create retention policy (keep data for 30 days)
SELECT add_retention_policy('metrics_snapshots', INTERVAL '30 days');

-- Create compression policy (compress chunks older than 1 day)
SELECT add_compression_policy('metrics_snapshots', INTERVAL '1 day');

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_metrics_server_timestamp 
ON metrics_snapshots (server_id, timestamp DESC);

-- Set parallel workers for better query performance
ALTER TABLE metrics_snapshots SET (
    timescaledb.max_parallel_chunk_scan = 2
);