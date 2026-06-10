//! Kit Configuration Framework
//! 
//! A reusable configuration framework for the entire Kit ecosystem that provides
//! typed, validated, extensible, and production-ready configuration management.
//! 
//! The framework supports multi-source configuration loading (defaults, TOML, dotenv, 
//! environment variables) with deterministic precedence, reusable configuration modules 
//! (infrastructure, observability, runtime), and a layered validation pipeline 
//! (framework → application → domain).
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
//! 
//! # Features
//! 
//! - Multi-source configuration loading
//! - Typed configuration with serde support
//! - Validation framework with layered validation
//! - Extensible source system
//! - Environment profiles
//! - Error handling with detailed error types

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::cargo)]

pub mod loader;
pub mod sources;
pub mod profile;
pub mod validation;
pub mod modules;
pub mod extension;
pub mod errors;
pub mod cloud;

pub use loader::ConfigLoader;
pub use sources::ConfigurationSource;
pub use profile::ConfigurationProfile;
pub use validation::{ValidationReport, ValidationError};
pub use cloud::{AwsSource, DigitalOceanSource, GcpSource};

pub use modules::logging::{
    LoggingConfig, LogLevel, LogFormat, OutputTarget, OutputConfig,
    TimestampConfig, TimestampFormat, CorrelationConfig, StructuredConfig,
    RedactionConfig, SamplingConfig, SamplingStrategy, BufferingConfig,
    RotationConfig, RetentionConfig, CategoryConfig, CategoriesConfig,
};