# Logging Configuration

Kit Config provides a comprehensive, strongly-typed logging configuration model that is provider-agnostic — it defines the configuration contract without depending on any specific logger implementation.

## Overview

The `LoggingConfig` struct is the top-level configuration type. It supports:

- Log levels and formats
- Multiple output targets
- Timestamp formatting
- Correlation / tracing metadata
- Structured logging
- Sensitive data redaction
- Sampling strategies (probabilistic, every-Nth, rate-limited)
- Async buffering
- Log rotation and retention
- Per-module log level overrides
- Category-level configuration (audit, security, metrics, etc.)

## Usage

### With the configuration loader

```rust
use kit_config::loader::ConfigLoader;
use kit_config::modules::logging::LoggingConfig;

let config: LoggingConfig = ConfigLoader::builder()
    .add_defaults()
    .build()
    .unwrap()
    .load_and_validate()
    .unwrap();

assert!(config.enabled);
assert_eq!(config.level, kit_config::LogLevel::Info);
```

### Direct instantiation

```rust
use kit_config::modules::logging::{
    LoggingConfig, LogLevel, LogFormat, OutputTarget, OutputConfig,
    SamplingConfig, SamplingStrategy, BufferingConfig,
    TimestampConfig, TimestampFormat, RedactionConfig,
    RotationConfig, RetentionConfig, CorrelationConfig,
    StructuredConfig, CategoriesConfig,
};
use std::collections::HashMap;

let config = LoggingConfig {
    enabled: true,
    level: LogLevel::Debug,
    format: LogFormat::Json,
    output: OutputConfig {
        targets: vec![OutputTarget::Stdout, OutputTarget::Stderr],
    },
    timestamp: TimestampConfig {
        enabled: true,
        format: TimestampFormat::Rfc3339Nano,
    },
    correlation: CorrelationConfig { enabled: true },
    structured: StructuredConfig { enabled: true },
    redact: RedactionConfig {
        enabled: true,
        fields: vec!["password".into(), "token".into()],
    },
    sampling: SamplingConfig {
        enabled: true,
        strategy: SamplingStrategy::Probabilistic,
        rate: 0.05,
        ..SamplingConfig::default()
    },
    buffering: BufferingConfig {
        enabled: true,
        batch_size: 200,
        flush_interval_ms: 500,
    },
    rotation: RotationConfig {
        enabled: true,
        max_size_mb: 500,
        max_backups: 20,
        max_age_days: 90,
    },
    retention: RetentionConfig {
        enabled: true,
        days: 60,
    },
    overrides: HashMap::from([
        ("ego_runtime".into(), LogLevel::Debug),
        ("sqlx".into(), LogLevel::Warn),
    ]),
    categories: CategoriesConfig::default(),
};
```

### YAML configuration example

```yaml
logging:
  enabled: true
  level: info
  format: json
  output:
    targets:
      - console
      - stderr
  timestamp:
    enabled: true
    format: rfc3339
  correlation:
    enabled: true
  structured:
    enabled: true
  redact:
    enabled: true
    fields:
      - password
      - token
      - secret
      - authorization
  sampling:
    enabled: true
    strategy: probabilistic
    rate: 0.10
  buffering:
    enabled: true
    batch_size: 100
    flush_interval_ms: 1000
  rotation:
    enabled: true
    max_size_mb: 100
    max_backups: 10
    max_age_days: 30
  retention:
    enabled: true
    days: 30
  overrides:
    ego_runtime: debug
    ego_scheduler: trace
    sqlx: warn
  categories:
    audit:
      enabled: true
    security:
      enabled: true
    metrics:
      enabled: false
    application:
      enabled: true
    access:
      enabled: false
```

## Types Reference

### `LogLevel`

| Variant | Serde value |
|---------|-------------|
| `Trace` | `"trace"` |
| `Debug` | `"debug"` |
| `Info`  | `"info"` |
| `Warn`  | `"warn"` |
| `Error` | `"error"` |

### `LogFormat`

| Variant   | Serde value   |
|-----------|---------------|
| `Json`    | `"json"`      |
| `Pretty`  | `"pretty"`    |
| `Compact` | `"compact"`   |
| `Text`    | `"text"`      |

### `OutputTarget`

| Variant  | Serde value  |
|----------|--------------|
| `Console`| `"console"`  |
| `Stdout` | `"stdout"`   |
| `Stderr` | `"stderr"`   |

### `TimestampFormat`

| Variant       | Serde value      |
|---------------|------------------|
| `Rfc3339`     | `"rfc3339"`      |
| `Rfc3339Nano` | `"rfc3339nano"`  |
| `Unix`        | `"unix"`         |
| `UnixMillis`  | `"unix_millis"`  |
| `Custom`      | `"custom"`       |

### `SamplingStrategy`

| Variant          | Serde value       |
|------------------|-------------------|
| `None`           | `"none"`          |
| `Probabilistic`  | `"probabilistic"` |
| `EveryNth`       | `"every_nth"`     |
| `RateLimit`      | `"rate_limit"`    |

## Validation

`LoggingConfig` implements the `Validation` trait. The following rules are enforced:

- **Probabilistic sampling**: `rate` must be between `0.0` and `1.0`
- **Every-Nth sampling**: `n` must be greater than `0`
- **Rate-limit sampling**: `max_events_per_second` must be greater than `0`
- **Output targets**: at least one target required when logging is enabled
- **Retention**: `days` must be greater than `0` when enabled
- **Rotation**: `max_size_mb` must be greater than `0` when enabled
- **Buffering**: `batch_size` must be greater than `0` when enabled

## Design Constraints

- No dependency on `tracing`, `slog`, `log`, or OpenTelemetry in the configuration domain
- Runtime-neutral
- Provider-agnostic — configuration stays valid regardless of the logging backend
- Designed for future OpenTelemetry, file outputs, rotation, and retention providers without changing the public configuration contract
