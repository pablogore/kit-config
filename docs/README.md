# Kit Config

A flexible, multi-source configuration loading library for Rust applications with a provider-agnostic architecture.

## Key Features

- **Multi-source loading**: Load configuration from multiple sources with deterministic precedence
- **Extensible architecture**: Easy to add new configuration sources
- **Cloud provider integration**: Native support for AWS, GCP, and DigitalOcean configurations
- **Flexible key-value maps**: Support for structured configuration data
- **Default values**: Set sensible defaults that can be overridden
- **Logging configuration**: Comprehensive, strongly-typed, provider-agnostic logging config
- **Type-safe**: Built with Rust's type system for compile-time safety

## Getting Started

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

println!("Log level: {:?}", config.level);
```

## Documentation

- [Basic Usage](./docs/defaults.md) — Default values and basic configuration
- [Sources](./docs/sources.md) — All available configuration sources
- [Cloud Providers](./docs/cloud.md) — AWS, GCP, and DigitalOcean configuration
- [Key-Value Maps](./docs/key_value_maps.md) — Structured key-value configuration
- **Logging Configuration** — [Full reference](./logging.md)
- [Extending the Framework](./docs/extending.md) — Custom sources and modules
