# Kit Config

A flexible, multi-source configuration loading library for Rust applications.

## Overview

Kit Config provides a robust solution for loading configuration from multiple sources with a clean, extensible API. It supports various configuration sources including environment variables, dotenv files, key-value maps, cloud provider configurations, and a comprehensive provider-agnostic logging configuration model.

## Architecture

Kit Config is a Rust workspace composed of four crates with clear ownership boundaries:

| Crate | Role | Depends On |
|-------|------|------------|
| `kit-config` | Public facade (re-exports everything) | all crates |
| `config-core` | Traits, errors, validation framework | — |
| `config-models` | Pure configuration data structures | `config-core` |
| `config-loaders` | Source loading, parsing, merging | `config-core` |

For most users, depending on `kit-config` is sufficient — it re-exports the full public API:

```toml
[dependencies]
kit-config = "0.1"
```

Ecosystem crates (e.g. kit-logger, ego-rs, atlas) can depend on individual crates for more targeted dependencies:

```toml
[dependencies]
config-core = { path = "crates/config-core" }
config-models = { path = "crates/config-models" }
```

This avoids pulling in loading infrastructure when only the configuration contracts are needed.

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
