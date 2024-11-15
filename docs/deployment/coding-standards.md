# Coding Standards

## General Guidelines

### Code Organization
- Follow single responsibility principle
- Keep functions and methods focused
- Use clear and meaningful names
- Document complex logic
- Write self-documenting code

### Version Control
```bash
# Branch Naming
feature/feature-name
fix/bug-description
refactor/component-name
docs/documentation-update

# Commit Messages
<type>: <description>

Types:
- feat: New feature
- fix: Bug fix
- refactor: Code refactoring
- docs: Documentation
- test: Testing
- chore: Maintenance

Example:
feat: Add server metrics collection
```

## Rust Backend Standards

### Code Style
```rust
// File Organization
use statements
constants
structs/enums
implementations
functions

// Naming Conventions
struct ServerMetrics {}  // Pascal case for types
fn collect_metrics() {}  // Snake case for functions
const MAX_CONNECTIONS: i32 = 100;  // Screaming snake case for constants

// Error Handling
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
}

// Documentation
/// Collects system metrics from the server
/// 
/// # Arguments
/// * `server_id` - The ID of the server to monitor
/// 
/// # Returns
/// * `Result<ServerMetrics, AppError>` - The collected metrics or an error
pub async fn collect_metrics(server_id: &str) -> Result<ServerMetrics, AppError> {
    // Implementation
}
```

### Project Structure
```
src/
├── api/
│   ├── handlers.rs   # Request handlers
│   ├── routes.rs     # Route definitions
│   └── mod.rs        # Module exports
├── models/
│   ├── entity.rs     # Data structures
│   └── mod.rs
├── services/
│   ├── auth.rs       # Business logic
│   └── mod.rs
└── lib.rs            # Public API
```

### Testing Standards
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collect_metrics() {
        // Arrange
        let server_id = "test-server";
        
        // Act
        let result = collect_metrics(server_id).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

## Flutter Client Standards

### Code Organization
```dart
// File Organization
import statements
constants
widgets/classes
private methods
public methods

// Widget Structure
class ServerMetricsWidget extends StatelessWidget {
  final String serverId;
  
  const ServerMetricsWidget({
    Key? key,
    required this.serverId,
  }) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    return // Widget tree
  }
}
```

### State Management
```dart
// Provider Pattern
class MetricsProvider extends ChangeNotifier {
  ServerMetrics? _metrics;
  
  ServerMetrics? get metrics => _metrics;
  
  Future<void> updateMetrics(String serverId) async {
    // Update logic
    notifyListeners();
  }
}

// Usage
class MetricsView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<MetricsProvider>(
      builder: (context, provider, child) {
        // UI construction
      },
    );
  }
}
```

### UI/UX Standards
```dart
// Constants
class AppColors {
  static const primary = Color(0xFF1976D2);
  static const error = Color(0xFFD32F2F);
}

// Styling
class AppTheme {
  static ThemeData get light => ThemeData(
    primaryColor: AppColors.primary,
    // Theme configuration
  );
}

// Responsive Design
class ResponsiveBuilder extends StatelessWidget {
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        if (constraints.maxWidth < 600) {
          return MobileLayout();
        }
        return DesktopLayout();
      },
    );
  }
}
```

### Testing
```dart
// Widget Tests
testWidgets('MetricsWidget displays data correctly',
    (WidgetTester tester) async {
  await tester.pumpWidget(
    MaterialApp(
      home: MetricsWidget(serverId: 'test'),
    ),
  );
  
  expect(find.byType(MetricsWidget), findsOneWidget);
});

// Unit Tests
test('MetricsProvider updates state correctly', () {
  final provider = MetricsProvider();
  provider.updateMetrics('test');
  expect(provider.metrics, isNotNull);
});
```

## API Standards

### REST Endpoints
```
# URL Structure
/api/v1/resource
/api/v1/resource/{id}
/api/v1/resource/{id}/subresource

# HTTP Methods
GET: Retrieve
POST: Create
PUT: Update (full)
PATCH: Update (partial)
DELETE: Remove

# Status Codes
200: Success
201: Created
400: Bad Request
401: Unauthorized
404: Not Found
500: Server Error
```

### Response Format
```json
{
  "success": true,
  "data": {
    // Response data
  },
  "error": null
}

{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description"
  }
}
```

## Database Standards

### SQL Queries
```sql
-- Table Names: Plural, snake_case
CREATE TABLE server_metrics (
    id SERIAL PRIMARY KEY,
    server_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_server_metrics_server_id 
ON server_metrics (server_id);

-- Foreign Keys
ALTER TABLE server_metrics
ADD CONSTRAINT fk_server
FOREIGN KEY (server_id)
REFERENCES servers(id);
```

### Migrations
```sql
-- Up Migration
CREATE TABLE servers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Down Migration
DROP TABLE servers;
```

## Documentation Standards

### Code Documentation
- Use doc comments for public APIs
- Include examples in documentation
- Document error conditions
- Keep documentation up to date

### API Documentation
- Use OpenAPI/Swagger
- Include request/response examples
- Document error responses
- Maintain versioning information

## Security Standards

### Authentication
- Use secure password hashing
- Implement proper token management
- Apply rate limiting
- Log security events

### Data Protection
- Sanitize user inputs
- Use prepared statements
- Encrypt sensitive data
- Apply proper access controls