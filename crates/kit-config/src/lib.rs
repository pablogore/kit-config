//! Kit Configuration Framework
//!
//! A reusable configuration framework for the entire Kit ecosystem that provides
//! typed, validated, extensible, and production-ready configuration management.
//!
//! The framework supports multi-source configuration loading (defaults, TOML, dotenv,
//! environment variables) with deterministic precedence, reusable configuration modules
//! (infrastructure, observability, runtime), and a layered validation pipeline
//! (framework -> application -> domain).
//!
//! # Examples
//!
//! Basic usage:
//! ```
//! use kit_config::loader::ConfigLoader;
//! use kit_config::sources::ConfigurationSource;
//!
//! let config = ConfigLoader::builder()
//!     .add_defaults()
//!     .add_environment()
//!     .build()
//!     .unwrap();
//! ```
//!
//! Cloud provider usage:
//! ```
//! use kit_config::loader::ConfigLoader;
//!
//! let config = ConfigLoader::builder()
//!     .add_defaults()
//!     .add_aws(true)
//!     .add_digitalocean(true)
//!     .add_environment()
//!     .build()
//!     .unwrap();
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::cargo)]

pub use config_core::errors;
pub use config_core::validation;
pub use config_core::profile;
pub use config_core::extension;

pub use config_loaders::loader;
pub use config_loaders::cloud;

pub mod sources;
pub mod modules;

pub use config_core::{ConfigError, ConfigurationSource, ConfigurationProfile};
pub use config_core::{Validation, ValidationReport, ValidationError};
pub use config_core::{ConfigModule, Extension};

pub use config_loaders::loader::ConfigLoader;
pub use config_loaders::cloud::{AwsSource, DigitalOceanSource, GcpSource};

pub use config_models::logging::{
    LoggingConfig, LogLevel, LogFormat, OutputTarget, OutputConfig,
    TimestampConfig, TimestampFormat, CorrelationConfig, StructuredConfig,
    RedactionConfig, SamplingConfig, SamplingStrategy, BufferingConfig,
    RotationConfig, RetentionConfig, CategoryConfig, CategoriesConfig,
};
