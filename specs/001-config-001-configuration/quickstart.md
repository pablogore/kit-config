# Quickstart: Configuration Framework

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kit-config = { path = "../kit-config" } # or version from crates.io
```

## Basic Usage

### 1. Define Your Configuration Structure

```rust
use kit_config::modules::infra::HttpModule;
use kit_config::modules::observability::LoggerModule;

#[derive(Debug, serde::Deserialize)]
struct MyApplicationConfig {
    http: HttpModule,
    logger: LoggerModule,
    // Add your domain-specific configuration here
    database_url: String,
    max_connections: u32,
}
```

### 2. Create Configuration Files

**config.toml** (base configuration):
```toml
[http]
host = "0.0.0.0"
port = 8080

[logger]
level = "info"
format = "json"

database_url = "postgresql://localhost/myapp"
max_connections = 10
```

**config.local.toml** (local development overrides):
```toml
[logger]
level = "debug"

database_url = "postgresql://localhost/myapp_dev"
```

**.env** (environment variables):
```env
KIT_HTTP_PORT=3000
KIT_DATABASE_URL=postgresql://localhost/myapp_local
```

### 3. Load and Validate Configuration

```rust
use kit_config::{ConfigLoader, ValidationReport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create loader with default sources
    let loader = ConfigLoader::builder()
        .add_defaults()           // Built-in defaults
        .add_toml("config.toml")  // Base config
        .add_profile_toml()       // Profile-specific config (config.{profile}.toml)
        .add_dotenv(".env")       // Dotenv file
        .add_environment()        // Environment variables
        .profile("local")         // Set active profile
        .prefix("KIT_")           // Environment variable prefix
        .strict_mode(true)        // Fail on unknown fields (default)
        .build()?;
    
    // Load and validate configuration
    let config: MyApplicationConfig = loader.load_and_validate()?;
    
    println!("HTTP server: {}:{}", config.http.host, config.http.port);
    println!("Logger level: {}", config.logger.level);
    println!("Database: {}", config.database_url);
    
    Ok(())
}
```

### 4. Handle Validation Errors

```rust
match loader.load_and_validate::<MyApplicationConfig>() {
    Ok(config) => {
        // Use validated config
        run_application(config);
    }
    Err(e) => {
        // Handle validation errors
        if let Some(report) = e.downcast_ref::<ValidationReport>() {
            eprintln!("Configuration validation failed:");
            for error in report.all_errors() {
                eprintln!("  - {}: {}", error.field, error.message);
            }
        }
        std::process::exit(1);
    }
}
```

## Advanced Usage

### Custom Validation

Implement application-specific validation:

```rust
use kit_config::validation::{ApplicationValidator, ValidationError};

impl ApplicationValidator for MyApplicationConfig {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        if self.max_connections == 0 {
            errors.push(ValidationError::new(
                "max_connections", 
                "must be greater than 0"
            ));
        }
        
        if self.database_url.starts_with("sqlite") && self.max_connections > 1 {
            errors.push(ValidationError::new(
                "max_connections", 
                "SQLite only supports single connection"
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Custom Configuration Modules

Create reusable domain-specific modules:

```rust
use kit_config::modules::{ConfigModule, Validation};

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseModule {
    pub url: String,
    pub pool_size: u32,
    pub ssl_mode: String,
}

impl ConfigModule for DatabaseModule {
    fn defaults() -> Self {
        Self {
            url: "postgresql://localhost/postgres".to_string(),
            pool_size: 10,
            ssl_mode: "prefer".to_string(),
        }
    }
}

impl Validation for DatabaseModule {
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        if self.pool_size == 0 {
            errors.push(ValidationError::new("pool_size", "must be > 0"));
        }
        
        if !["disable", "allow", "prefer", "require", "verify-ca", "verify-full"]
            .contains(&self.ssl_mode.as_str()) {
            errors.push(ValidationError::new(
                "ssl_mode", 
                "must be one of: disable, allow, prefer, require, verify-ca, verify-full"
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Permissive Mode for Unknown Fields

When working with evolving schemas, enable permissive mode:

```rust
let loader = ConfigLoader::builder()
    .strict_mode(false) // Warn instead of fail on unknown fields
    // ... other configuration
    .build()?;
```

## Default Profiles

The framework supports these built-in profiles:
- `local` (default)
- `development` 
- `test`
- `staging`
- `production`

Set the active profile via:
- `ConfigLoader::profile("production")`
- Environment variable: `KIT_PROFILE=production`
- Command line argument (if your application supports it)

## Environment Variable Prefixing

Environment variables are automatically mapped based on your prefix:
- `KIT_HTTP_PORT=8080` → `http.port = 8080`
- `KIT_LOGGER_LEVEL=debug` → `logger.level = "debug"`
- `KIT_DATABASE_URL=...` → `database_url = "..."`

Nested structures use underscore separation:
- `KIT_DATABASE_CONNECTION_POOL_SIZE=20` → `database.connection.pool_size = 20`

## Built-in Modules Reference

### Infrastructure
- `HttpModule`, `HttpsModule`, `GrpcModule`
- `PostgresModule`, `MySqlModule`, `RedisModule`  
- `KafkaModule`, `RedpandaModule`, `NatsModule`, `S3Module`

### Observability
- `LoggerModule`, `MetricsModule`, `TracingModule`, `OpenTelemetryModule`

### Runtime
- `RetryModule`, `BackoffModule`, `CircuitBreakerModule`, `WorkerPoolModule`

All modules provide:
- Built-in defaults (required but overridable)
- Type-safe configuration structs
- Automatic validation
- Helper functions for common operations

## Error Handling Best Practices

1. **Always validate before use**: Never assume configuration is valid
2. **Fail fast**: Exit early with clear error messages
3. **Collect all errors**: Don't stop at the first validation failure
4. **Provide actionable feedback**: Error messages should guide users to fix issues
5. **Log validation errors**: Even in production, validation failures should be logged

## Testing Configuration

Use the test utilities for comprehensive testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use kit_config::test_utils::TestConfigLoader;
    
    #[test]
    fn test_valid_config() {
        let loader = TestConfigLoader::new()
            .with_toml(r#"
                [http]
                port = 3000
                [logger]  
                level = "debug"
                database_url = "test_db"
                max_connections = 5
            "#)
            .build();
            
        let config: MyApplicationConfig = loader.load_and_validate().unwrap();
        assert_eq!(config.http.port, 3000);
        assert_eq!(config.logger.level, "debug");
    }
    
    #[test]
    fn test_invalid_config() {
        let loader = TestConfigLoader::new()
            .with_toml(r#"
                [http]
                port = 0  # Invalid port
                database_url = "test_db"  
                max_connections = 0  # Invalid connections
            "#)
            .build();
            
        let result: Result<MyApplicationConfig, _> = loader.load_and_validate();
        assert!(result.is_err());
        
        let report = result.unwrap_err().downcast::<ValidationReport>().unwrap();
        assert_eq!(report.all_errors().len(), 2);
    }
}
```