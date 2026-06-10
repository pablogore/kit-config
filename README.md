# Kit Config

A flexible, multi-source configuration loading library for Rust applications.

## Overview

Kit Config provides a robust solution for loading configuration from multiple sources with a clean, extensible API. It supports various configuration sources including environment variables, dotenv files, key-value maps, cloud provider configurations, and a comprehensive provider-agnostic logging configuration model.

## Key Features

- **Multi-source loading**: Load configuration from multiple sources with deterministic precedence
- **Extensible architecture**: Easy to add new configuration sources
- **Cloud provider integration**: Native support for AWS, GCP, and DigitalOcean configurations
- **Flexible key-value maps**: Support for structured configuration data
- **Type-safe**: Built with Rust's type system for compile-time safety
- **Logging configuration**: Fully typed, provider-agnostic logging config with support for levels, formats, outputs, sampling, buffering, rotation, retention, redaction, and per-module overrides

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kit-config = "0.1"
```

### Basic Usage

```rust
use kit_config::loader::ConfigLoader;

let config = ConfigLoader::builder()
    .add_defaults()
    .add_toml("config.toml")
    .add_environment()
    .build()
    .unwrap();
```

### Logging Configuration

```rust
use kit_config::loader::ConfigLoader;
use kit_config::modules::logging::LoggingConfig;

let config: LoggingConfig = ConfigLoader::builder()
    .add_defaults()
    .build()
    .unwrap()
    .load_and_validate()
    .unwrap();
```

### YAML Logging Example

```yaml
logging:
  enabled: true
  level: info
  format: json
  output:
    targets:
      - console
      - stderr
  sampling:
    enabled: true
    strategy: probabilistic
    rate: 0.10
  overrides:
    ego_runtime: debug
    sqlx: warn
  categories:
    audit:
      enabled: true
    security:
      enabled: true
    metrics:
      enabled: false
```

## Documentation

For comprehensive documentation, see the [docs](./docs/README.md).

## Use Cases

This library is ideal for:
- Application configuration management
- Cloud-native application deployment
- Multi-environment configuration handling
- Structured configuration with fallback mechanisms
- Provider-agnostic logging configuration
