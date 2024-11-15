# Getting Started

## Prerequisites

### Backend Development
- Rust 1.75 or later
- PostgreSQL 15+ with TimescaleDB extension
- Docker and Docker Compose (optional)

### Frontend Development
- Flutter 3.19+ (required for null safety features)
- Dart SDK 3.3.0+
- iOS development tools (for iOS development)
- Android Studio & SDK (for Android development)

## Initial Setup

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/flick.git
cd flick
```

### 2. Database Setup

#### Using Docker (Recommended)
```bash
cd docker
docker-compose up -d
```

#### Manual Setup
1. Install PostgreSQL and TimescaleDB
2. Create a new database
```sql
CREATE DATABASE server_monitoring;
```
3. Apply TimescaleDB extension
```sql
\c server_monitoring
CREATE EXTENSION IF NOT EXISTS timescaledb;
```
4. Run initialization scripts
```bash
psql -U postgres -d server_monitoring -f docker/init-scripts/01-init-timescale.sql
```

### 3. Backend Setup

#### Environment Configuration
Create a `.env` file in the `rust_server` directory:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/server_monitoring
RUST_LOG=debug
JWT_SECRET=your-secret-key
```

#### Build and Run
```bash
cd rust_server
cargo build
cargo run
```

The server will start on `http://localhost:8080`

### 4. Flutter Client Setup

#### Dependencies Installation
```bash
cd flutter_client
flutter pub get
```

#### Run the Application
```bash
# For development
flutter run -d chrome  # Web
flutter run -d ios     # iOS
flutter run -d android # Android
```

## Development Workflow

### Backend Development

1. Code Organization
```
rust_server/
├── src/
│   ├── api/        # REST API endpoints
│   ├── auth/       # Authentication logic
│   ├── db/         # Database operations
│   ├── monitoring/ # System monitoring
│   └── websocket/  # WebSocket handlers
```

2. Database Migrations
```bash
cd rust_server
sqlx migrate run
```

3. Running Tests
```bash
cargo test
```

### Frontend Development

1. Code Organization
```
flutter_client/
├── lib/
│   ├── core/      # Core functionality
│   ├── features/  # Feature modules
│   └── shared/    # Shared components
```

2. Running Tests
```bash
flutter test
```

## Common Development Tasks

### Creating a New API Endpoint
1. Add route in `rust_server/src/api/routes.rs`
2. Implement handler in appropriate module
3. Add tests in `tests` directory
4. Update API documentation

### Adding a New Feature
1. Create feature directory in `flutter_client/lib/features`
2. Implement UI components
3. Add state management
4. Create tests
5. Update documentation

## Troubleshooting

### Common Issues

1. Database Connection Issues
```bash
# Check PostgreSQL status
sudo service postgresql status

# Verify connection string
psql "postgresql://postgres:postgres@localhost:5432/server_monitoring"
```

2. Flutter Build Issues
```bash
# Clean and rebuild
flutter clean
flutter pub get
flutter run
```

### Getting Help
- Check the GitHub issues
- Review the documentation in `/docs`
- Contact the development team

## Next Steps
- Review the [Architecture Overview](../architecture/overview.md)
- Check the [API Documentation](../api/rest.md)
- Read the [Contributing Guidelines](contributing.md)