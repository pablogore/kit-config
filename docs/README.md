# Kit Config

A flexible, multi-source configuration loading library for Rust applications with a provider-agnostic architecture.

## Workspace Architecture

Kit Config is organized as a Rust workspace with four crates:

```text
kit-config/                    (workspace root)
└── crates/
    ├── config-core/           traits, errors, validation framework
    ├── config-models/         pure data structures (logging, infra)
    ├── config-loaders/        source loading, parsing, merging
    └── kit-config/            public facade (re-exports all)
```

**Dependency rules:**
- `config-models` → `config-core`
- `config-loaders` → `config-core`
- `kit-config` → all crates

### When to use each crate

| Use case | Crate |
|----------|-------|
| Application configuration loading | `kit-config` |
| Implementing a custom config source | `config-core` (for `ConfigurationSource` trait) |
| Defining a new configuration module | `config-models` + `config-core` |
| Validation-only dependency | `config-core` (for `Validation`, `ValidationReport`) |

### End users (recommended)

```toml
[dependencies]
kit-config = "0.1"
```

Default features include everything. All public types are re-exported through the facade. No need to track internal crate boundaries.

### Ecosystem crates (selective imports)

For crates that only need configuration contracts (e.g. kit-logger, ego-rs), disable default features and enable only what you need:

```toml
[dependencies]
kit-config = { version = "0.1", default-features = false, features = ["config-core", "config-models"] }
```

This avoids pulling in file I/O, environment parsing, and `toml` dependencies from `config-loaders`.

### Feature reference

| Feature | Provides | Implies |
|---------|----------|---------|
| `config-core` | `ConfigError`, `Validation`, `ConfigurationSource`, `ConfigModule` | — |
| `config-models` | `LoggingConfig`, infra modules (`GrpcModule`, `HttpModule`, etc.) | `config-core` |
| `config-loaders` | `ConfigLoader`, sources (TOML, dotenv, env, cloud), `AwsSource`, `GcpSource` | `config-core` |

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
