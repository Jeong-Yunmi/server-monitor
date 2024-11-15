-- migrations/20240314000000_create_base_tables.sql
CREATE TYPE server_type AS ENUM ('physical', 'virtual', 'container');
CREATE TYPE alert_severity AS ENUM ('info', 'warning', 'critical');
CREATE TYPE user_role AS ENUM ('admin', 'user', 'viewer');

-- Users table first
CREATE TABLE users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Then servers table
CREATE TABLE servers (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    location VARCHAR(255) NOT NULL,
    server_type server_type NOT NULL,
    is_online BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Then metrics
CREATE TABLE metrics_snapshots (
    id BIGSERIAL PRIMARY KEY,
    server_id VARCHAR(36) NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    cpu_usage FLOAT NOT NULL,
    memory_usage FLOAT NOT NULL,
    disk_usage FLOAT NOT NULL,
    network_rx BIGINT NOT NULL,
    network_tx BIGINT NOT NULL,
    processes JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT metrics_cpu_usage_check CHECK (cpu_usage >= 0 AND cpu_usage <= 100),
    CONSTRAINT metrics_memory_usage_check CHECK (memory_usage >= 0 AND memory_usage <= 100),
    CONSTRAINT metrics_disk_usage_check CHECK (disk_usage >= 0 AND disk_usage <= 100)
);

-- Then alerts table (which references both users and servers)
CREATE TABLE alerts (
    id BIGSERIAL PRIMARY KEY,
    server_id VARCHAR(36) NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    alert_type VARCHAR(50) NOT NULL,
    severity alert_severity NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by VARCHAR(36) REFERENCES users(id)
);

-- Finally alert thresholds
CREATE TABLE alert_thresholds (
    id BIGSERIAL PRIMARY KEY,
    server_id VARCHAR(36) NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    metric_type VARCHAR(50) NOT NULL,
    warning_threshold FLOAT NOT NULL,
    critical_threshold FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (server_id, metric_type)
);

-- Create indexes
CREATE INDEX idx_metrics_server_timestamp ON metrics_snapshots(server_id, timestamp);
CREATE INDEX idx_alerts_server_created ON alerts(server_id, created_at);
CREATE INDEX idx_alerts_unacknowledged ON alerts(acknowledged_at) WHERE acknowledged_at IS NULL;

-- Create function for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updating timestamps
CREATE TRIGGER update_servers_modtime
    BEFORE UPDATE ON servers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_modtime
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();