# Data Model: Configuration Framework

## Core Entities

### ConfigLoader
**Description**: Responsible for loading configuration from multiple sources and producing a merged configuration value
**Fields**:
- `sources: Vec<Box<dyn ConfigurationSource>>` - Ordered list of configuration sources
- `profile: Option<String>` - Active environment profile (local, dev, test, staging, production)
- `prefix: Option<String>` - Environment variable prefix (e.g., "KIT_", "EGO_")
- `strict_mode: bool` - Whether to fail on unknown fields (true by default)

**Relationships**: 
- Composes multiple ConfigurationSource instances
- Uses Profile for profile-specific loading
- Generates ValidationReport after validation

### ConfigurationSource
**Description**: Represents a configuration provider with priority and read method
**Fields**:
- `priority: u8` - Source precedence (lower = higher priority)
- `optional: bool` - Whether source is optional (missing files don't cause errors)

**Implementations**:
- `DefaultsSource` - Built-in defaults
- `TomlFileSource` - TOML file loading
- `DotenvSource` - dotenv file loading  
- `EnvironmentSource` - Environment variable loading

### ConfigurationProfile
**Description**: Environment-specific configuration management
**Fields**:
- `name: String` - Profile name (local, dev, test, staging, production)
- `active: bool` - Whether this profile is currently active

**Relationships**:
- Used by ConfigLoader to determine profile-specific file loading
- Profile files override base configuration

### ValidationReport
**Description**: Single data structure containing all validation errors from the pipeline
**Fields**:
- `framework_errors: Vec<ValidationError>` - Errors from reusable modules
- `application_errors: Vec<ValidationError>` - Errors from application-specific rules  
- `domain_errors: Vec<ValidationError>` - Errors from custom extensions
- `is_valid: bool` - Whether all validation stages passed

**Relationships**:
- Produced by ValidationPipeline
- Returned to application for error handling

### ValidationError
**Description**: Individual validation error with context
**Fields**:
- `field: String` - Field path that failed validation
- `message: String` - Human-readable error message
- `source: ValidationSource` - Which validation stage produced this error
- `code: Option<String>` - Machine-readable error code

### ValidationSource (enum)
**Variants**:
- `Framework` - Reusable kit-config modules
- `Application` - Application-specific validation  
- `Domain` - Custom extension validation

## Reusable Configuration Modules

### Infrastructure Modules

#### HttpModule
**Fields**:
- `host: String` (default: "0.0.0.0")
- `port: u16` (default: 8080, range: 1-65535)
- `tls_enabled: bool` (default: false)
- `tls_cert_path: Option<String>`
- `tls_key_path: Option<String>`

#### PostgresModule  
**Fields**:
- `connection_string: String`
- `pool_size: u32` (default: 10, min: 1)
- `ssl_mode: String` (default: "prefer", allowed: ["disable", "allow", "prefer", "require", "verify-ca", "verify-full"])

#### RedisModule
**Fields**:
- `connection_string: String`  
- `pool_size: u32` (default: 5, min: 1)
- `tls_enabled: bool` (default: false)

#### KafkaModule
**Fields**:
- `bootstrap_servers: Vec<String>`
- `group_id: String`
- `auto_offset_reset: String` (default: "earliest", allowed: ["earliest", "latest", "none"])

### Observability Modules

#### LoggerModule
**Fields**:
- `level: String` (default: "info", allowed: ["trace", "debug", "info", "warn", "error"])
- `format: String` (default: "json", allowed: ["json", "text"])
- `output: String` (default: "stdout", allowed: ["stdout", "stderr", "file:<path>"])

#### MetricsModule
**Fields**:
- `enabled: bool` (default: true)
- `otlp_endpoint: Option<String>` (URL validation)
- `collection_interval: Duration` (default: "30s")

#### TracingModule  
**Fields**:
- `enabled: bool` (default: true)
- `sampling_rate: f64` (default: 0.1, range: 0.0-1.0)
- `otlp_endpoint: Option<String>` (URL validation)

### Runtime Modules

#### RetryModule
**Fields**:
- `max_retries: u32` (default: 3, min: 0)
- `base_delay: Duration` (default: "1s")
- `max_delay: Duration` (default: "30s")
- `backoff_multiplier: f64` (default: 2.0, min: 1.0)

#### CircuitBreakerModule
**Fields**:
- `enabled: bool` (default: true)
- `failure_threshold: u32` (default: 5, min: 1)
- `reset_timeout: Duration` (default: "30s")

#### WorkerPoolModule
**Fields**:
- `min_workers: u32` (default: 1, min: 1)
- `max_workers: u32` (default: 10, min: 1)
- `queue_size: u32` (default: 100, min: 1)

## Validation Rules

### Field-Level Validation
- Required fields must be present (non-Option types or Option with validation)
- Numeric ranges enforced (ports: 1-65535, percentages: 0.0-1.0, etc.)
- String length limits where applicable
- Enum values restricted to allowed variants
- URL fields validated as proper URLs
- Duration fields parsed from human-readable strings ("30s", "5m", "1h")
- Size fields parsed from byte strings ("10MB", "1.5GiB")

### Structure-Level Validation  
- Cross-field dependencies (TLS enabled requires certificate paths)
- Business logic rules (min_workers <= max_workers, base_delay <= max_delay)
- Profile-specific validation rules

### Cross-Field Validation
- Conditional requirements based on boolean flags
- Mutual exclusivity constraints where applicable
- Dependency validation between related fields

## State Transitions

### ConfigLoader States
1. **Initialized** - Created with sources and configuration
2. **Loading** - Actively reading configuration sources  
3. **Merged** - Sources successfully merged into unified config
4. **Validated** - Validation pipeline completed, ValidationReport available
5. **Error** - Loading or validation failed

### ValidationPipeline States  
1. **Ready** - Pipeline initialized with validators
2. **FrameworkValidating** - Framework validators executing
3. **ApplicationValidating** - Application validators executing  
4. **DomainValidating** - Domain validators executing
5. **Complete** - All stages completed, ValidationReport ready

## Relationships Summary

- **ConfigLoader** → uses → **ConfigurationSource** (1-to-many)
- **ConfigLoader** → manages → **ConfigurationProfile** (1-to-1 active)
- **ConfigLoader** → produces → **ValidationReport** (1-to-1)  
- **ValidationReport** → contains → **ValidationError** (1-to-many)
- **ValidationError** → references → **ValidationSource** (many-to-1 enum)
- Reusable **Module** types → implement → validation and helper functions
- All entities → support → serialization/deserialization via serde