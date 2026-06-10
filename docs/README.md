# Kit Config

A flexible, multi-source configuration loading library for Rust applications.

## Overview

Kit Config provides a robust solution for loading configuration from multiple sources with a clean, extensible API. It supports various configuration sources including environment variables, dotenv files, key-value maps, and cloud provider configurations.

## Key Features

- **Multi-source loading**: Load configuration from multiple sources with deterministic precedence
- **Extensible architecture**: Easy to add new configuration sources
- **Cloud provider integration**: Native support for AWS, GCP, and DigitalOcean configurations
- **Flexible key-value maps**: Support for structured configuration data
- **Default values**: Set sensible defaults that can be overridden
- **Type-safe**: Built with Rust's type system for compile-time safety

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kit-config = "0.1"
```

### Basic Usage

```rust
use kit_config::{ConfigLoader, ConfigurationSource};

let config = ConfigLoader::builder()
    .add_source(kit_config::sources::DotenvSource::new())
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();

let value = config.get("MY_CONFIG_KEY").unwrap();
```

## Documentation

For comprehensive documentation, please see the [Kit Config documentation](./docs/README.md).

## Use Cases

This library is ideal for:
- Application configuration management
- Cloud-native application deployment
- Multi-environment configuration handling
- Structured configuration with fallback mechanisms