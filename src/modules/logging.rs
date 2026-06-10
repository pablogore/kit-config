use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::modules::ConfigModule;
use crate::validation::{Validation, ValidationReport, ValidationError};

/// Log level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level (least verbose)
    Error,
}

/// Log output format enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// JSON structured format
    Json,
    /// Pretty-printed human-readable format
    Pretty,
    /// Compact format
    Compact,
    /// Plain text format
    Text,
}

/// Output target enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum OutputTarget {
    /// Standard output
    Console,
    /// Standard output (alias)
    Stdout,
    /// Standard error
    Stderr,
}

/// Timestamp format enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TimestampFormat {
    /// RFC 3339 format (e.g., 2024-01-01T00:00:00Z)
    Rfc3339,
    /// RFC 3339 with nanosecond precision
    #[serde(rename = "rfc3339nano")]
    Rfc3339Nano,
    /// Unix timestamp in seconds
    Unix,
    /// Unix timestamp in milliseconds
    #[serde(rename = "unix_millis")]
    UnixMillis,
    /// Custom format string
    Custom,
}

/// Sampling strategy enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SamplingStrategy {
    /// No sampling (log everything)
    None,
    /// Probabilistic sampling based on a rate
    Probabilistic,
    /// Log every Nth event
    EveryNth,
    /// Rate-limited sampling
    RateLimit,
}

/// Timestamp configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct TimestampConfig {
    /// Whether timestamps are enabled
    pub enabled: bool,
    /// Timestamp format
    pub format: TimestampFormat,
}

impl Default for TimestampConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: TimestampFormat::Rfc3339,
        }
    }
}

/// Correlation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CorrelationConfig {
    /// Whether correlation/tracing is enabled
    pub enabled: bool,
}

impl Default for CorrelationConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Structured logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct StructuredConfig {
    /// Whether structured logging is enabled
    pub enabled: bool,
}

impl Default for StructuredConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Sensitive data redaction configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct RedactionConfig {
    /// Whether redaction is enabled
    pub enabled: bool,
    /// List of field names to redact
    #[serde(default)]
    pub fields: Vec<String>,
}

impl Default for RedactionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            fields: vec![
                "password".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "authorization".to_string(),
            ],
        }
    }
}

/// Sampling configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct SamplingConfig {
    /// Whether sampling is enabled
    pub enabled: bool,
    /// Sampling strategy
    pub strategy: SamplingStrategy,
    /// Sampling rate (0.0 to 1.0) for probabilistic strategy
    pub rate: f64,
    /// Nth value for every_nth strategy
    pub n: u32,
    /// Maximum events per second for rate_limit strategy
    pub max_events_per_second: u32,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: SamplingStrategy::None,
            rate: 0.1,
            n: 100,
            max_events_per_second: 500,
        }
    }
}

/// Async buffering configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct BufferingConfig {
    /// Whether buffering is enabled
    pub enabled: bool,
    /// Maximum batch size before flush
    pub batch_size: usize,
    /// Flush interval in milliseconds
    pub flush_interval_ms: u64,
}

impl Default for BufferingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            batch_size: 100,
            flush_interval_ms: 1000,
        }
    }
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct RotationConfig {
    /// Whether rotation is enabled
    pub enabled: bool,
    /// Maximum file size in megabytes before rotation
    pub max_size_mb: u32,
    /// Maximum number of backup files to retain
    pub max_backups: u32,
    /// Maximum age of backup files in days
    pub max_age_days: u32,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_size_mb: 100,
            max_backups: 10,
            max_age_days: 30,
        }
    }
}

/// Retention configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct RetentionConfig {
    /// Whether retention is enabled
    pub enabled: bool,
    /// Number of days to retain logs
    pub days: u32,
}

impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            days: 30,
        }
    }
}

/// Category-level configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CategoryConfig {
    /// Whether this log category is enabled
    pub enabled: bool,
}

impl Default for CategoryConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Log categories configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CategoriesConfig {
    /// Audit log category
    pub audit: CategoryConfig,
    /// Security log category
    pub security: CategoryConfig,
    /// Metrics log category
    pub metrics: CategoryConfig,
    /// Application log category
    pub application: CategoryConfig,
    /// Access log category
    pub access: CategoryConfig,
}

impl Default for CategoriesConfig {
    fn default() -> Self {
        Self {
            audit: CategoryConfig { enabled: true },
            security: CategoryConfig { enabled: true },
            metrics: CategoryConfig { enabled: false },
            application: CategoryConfig { enabled: true },
            access: CategoryConfig { enabled: false },
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct OutputConfig {
    /// List of output targets
    #[serde(default)]
    pub targets: Vec<OutputTarget>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            targets: vec![OutputTarget::Stdout],
        }
    }
}

/// Main logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LoggingConfig {
    /// Whether logging is enabled
    pub enabled: bool,
    /// Default log level
    pub level: LogLevel,
    /// Log output format
    pub format: LogFormat,
    /// Output configuration
    pub output: OutputConfig,
    /// Timestamp configuration
    pub timestamp: TimestampConfig,
    /// Correlation configuration
    pub correlation: CorrelationConfig,
    /// Structured logging configuration
    pub structured: StructuredConfig,
    /// Redaction configuration
    pub redact: RedactionConfig,
    /// Sampling configuration
    pub sampling: SamplingConfig,
    /// Buffering configuration
    pub buffering: BufferingConfig,
    /// Rotation configuration
    pub rotation: RotationConfig,
    /// Retention configuration
    pub retention: RetentionConfig,
    /// Per-module log level overrides
    #[serde(default)]
    pub overrides: HashMap<String, LogLevel>,
    /// Category-level configuration
    pub categories: CategoriesConfig,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            format: LogFormat::Json,
            output: OutputConfig::default(),
            timestamp: TimestampConfig::default(),
            correlation: CorrelationConfig::default(),
            structured: StructuredConfig::default(),
            redact: RedactionConfig::default(),
            sampling: SamplingConfig::default(),
            buffering: BufferingConfig::default(),
            rotation: RotationConfig::default(),
            retention: RetentionConfig::default(),
            overrides: HashMap::new(),
            categories: CategoriesConfig::default(),
        }
    }
}

impl ConfigModule for LoggingConfig {
    fn defaults() -> Self {
        Self::default()
    }
}

impl Validation for LoggingConfig {
    fn validate(&self) -> Result<(), ValidationReport> {
        let mut report = ValidationReport::new();

        // Validate sampling rate is between 0 and 1 for probabilistic strategy
        if self.sampling.enabled && self.sampling.strategy == SamplingStrategy::Probabilistic {
            if !(0.0..=1.0).contains(&self.sampling.rate) {
                report.add_domain_error(ValidationError::new(
                    "sampling.rate",
                    "Sampling rate must be between 0.0 and 1.0 for probabilistic strategy",
                ));
            }
        }

        // Validate that n > 0 for every_nth strategy
        if self.sampling.enabled && self.sampling.strategy == SamplingStrategy::EveryNth {
            if self.sampling.n == 0 {
                report.add_domain_error(ValidationError::new(
                    "sampling.n",
                    "N must be greater than 0 for every_nth strategy",
                ));
            }
        }

        // Validate that max_events_per_second > 0 for rate_limit strategy
        if self.sampling.enabled && self.sampling.strategy == SamplingStrategy::RateLimit {
            if self.sampling.max_events_per_second == 0 {
                report.add_domain_error(ValidationError::new(
                    "sampling.max_events_per_second",
                    "max_events_per_second must be greater than 0 for rate_limit strategy",
                ));
            }
        }

        // Validate output targets are not empty
        if self.enabled && self.output.targets.is_empty() {
            report.add_domain_error(ValidationError::new(
                "output.targets",
                "At least one output target must be configured when logging is enabled",
            ));
        }

        // Validate retention days
        if self.retention.enabled && self.retention.days == 0 {
            report.add_domain_error(ValidationError::new(
                "retention.days",
                "Retention days must be greater than 0",
            ));
        }

        // Validate rotation max_size_mb
        if self.rotation.enabled && self.rotation.max_size_mb == 0 {
            report.add_domain_error(ValidationError::new(
                "rotation.max_size_mb",
                "Rotation max_size_mb must be greater than 0",
            ));
        }

        // Validate buffering batch_size
        if self.buffering.enabled && self.buffering.batch_size == 0 {
            report.add_domain_error(ValidationError::new(
                "buffering.batch_size",
                "Buffering batch_size must be greater than 0",
            ));
        }

        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_defaults() {
        let config = LoggingConfig::defaults();
        assert!(config.enabled);
        assert_eq!(config.level, LogLevel::Info);
        assert_eq!(config.format, LogFormat::Json);
        assert_eq!(config.output.targets, vec![OutputTarget::Stdout]);
        assert!(config.timestamp.enabled);
        assert_eq!(config.timestamp.format, TimestampFormat::Rfc3339);
        assert!(config.correlation.enabled);
        assert!(config.structured.enabled);
        assert!(!config.redact.enabled);
        assert_eq!(config.redact.fields.len(), 4);
        assert!(!config.sampling.enabled);
        assert!(!config.buffering.enabled);
        assert!(!config.rotation.enabled);
        assert!(!config.retention.enabled);
        assert!(config.overrides.is_empty());
        assert!(config.categories.audit.enabled);
        assert!(config.categories.security.enabled);
        assert!(!config.categories.metrics.enabled);
    }

    #[test]
    fn test_log_level_serde() {
        let levels = vec![
            (r#""trace""#, LogLevel::Trace),
            (r#""debug""#, LogLevel::Debug),
            (r#""info""#, LogLevel::Info),
            (r#""warn""#, LogLevel::Warn),
            (r#""error""#, LogLevel::Error),
        ];

        for (json, expected) in levels {
            let deserialized: LogLevel = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected);
            let serialized = serde_json::to_string(&expected).unwrap();
            assert_eq!(serialized, json);
        }
    }

    #[test]
    fn test_log_format_serde() {
        let formats = vec![
            (r#""json""#, LogFormat::Json),
            (r#""pretty""#, LogFormat::Pretty),
            (r#""compact""#, LogFormat::Compact),
            (r#""text""#, LogFormat::Text),
        ];

        for (json, expected) in formats {
            let deserialized: LogFormat = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected);
            let serialized = serde_json::to_string(&expected).unwrap();
            assert_eq!(serialized, json);
        }
    }

    #[test]
    fn test_output_target_serde() {
        let targets = vec![
            (r#""console""#, OutputTarget::Console),
            (r#""stdout""#, OutputTarget::Stdout),
            (r#""stderr""#, OutputTarget::Stderr),
        ];

        for (json, expected) in targets {
            let deserialized: OutputTarget = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected);
            let serialized = serde_json::to_string(&expected).unwrap();
            assert_eq!(serialized, json);
        }
    }

    #[test]
    fn test_timestamp_format_serde() {
        let formats = vec![
            (r#""rfc3339""#, TimestampFormat::Rfc3339),
            (r#""rfc3339nano""#, TimestampFormat::Rfc3339Nano),
            (r#""unix""#, TimestampFormat::Unix),
            (r#""unix_millis""#, TimestampFormat::UnixMillis),
            (r#""custom""#, TimestampFormat::Custom),
        ];

        for (json, expected) in formats {
            let deserialized: TimestampFormat = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected);
            let serialized = serde_json::to_string(&expected).unwrap();
            assert_eq!(serialized, json);
        }
    }

    #[test]
    fn test_sampling_strategy_serde() {
        let strategies = vec![
            (r#""none""#, SamplingStrategy::None),
            (r#""probabilistic""#, SamplingStrategy::Probabilistic),
            (r#""every_nth""#, SamplingStrategy::EveryNth),
            (r#""rate_limit""#, SamplingStrategy::RateLimit),
        ];

        for (json, expected) in strategies {
            let deserialized: SamplingStrategy = serde_json::from_str(json).unwrap();
            assert_eq!(deserialized, expected);
            let serialized = serde_json::to_string(&expected).unwrap();
            assert_eq!(serialized, json);
        }
    }

    #[test]
    fn test_yaml_parsing_full_config() {
        let yaml = r#"
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
  sampling:
    enabled: true
    strategy: probabilistic
    rate: 0.05
  buffering:
    enabled: true
    batch_size: 200
    flush_interval_ms: 500
  rotation:
    enabled: true
    max_size_mb: 500
    max_backups: 20
    max_age_days: 90
  retention:
    enabled: true
    days: 60
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
    application:
      enabled: true
    access:
      enabled: true
"#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            logging: LoggingConfig,
        }

        let wrapper: Wrapper = serde_yaml::from_str(yaml).unwrap();
        let config = wrapper.logging;

        assert!(config.enabled);
        assert_eq!(config.level, LogLevel::Info);
        assert_eq!(config.format, LogFormat::Json);
        assert_eq!(config.output.targets, vec![OutputTarget::Console, OutputTarget::Stderr]);
        assert!(config.timestamp.enabled);
        assert_eq!(config.timestamp.format, TimestampFormat::Rfc3339);
        assert!(config.correlation.enabled);
        assert!(config.structured.enabled);
        assert!(config.redact.enabled);
        assert_eq!(config.redact.fields, vec!["password", "token"]);
        assert!(config.sampling.enabled);
        assert_eq!(config.sampling.strategy, SamplingStrategy::Probabilistic);
        assert!((config.sampling.rate - 0.05).abs() < f64::EPSILON);
        assert!(config.buffering.enabled);
        assert_eq!(config.buffering.batch_size, 200);
        assert_eq!(config.buffering.flush_interval_ms, 500);
        assert!(config.rotation.enabled);
        assert_eq!(config.rotation.max_size_mb, 500);
        assert_eq!(config.rotation.max_backups, 20);
        assert_eq!(config.rotation.max_age_days, 90);
        assert!(config.retention.enabled);
        assert_eq!(config.retention.days, 60);
        assert_eq!(config.overrides.get("ego_runtime"), Some(&LogLevel::Debug));
        assert_eq!(config.overrides.get("sqlx"), Some(&LogLevel::Warn));
        assert!(config.categories.audit.enabled);
        assert!(config.categories.security.enabled);
        assert!(!config.categories.metrics.enabled);
        assert!(config.categories.application.enabled);
        assert!(config.categories.access.enabled);
    }

    #[test]
    fn test_yaml_sampling_every_nth() {
        let yaml = r#"
logging:
  enabled: true
  level: debug
  format: text
  sampling:
    enabled: true
    strategy: every_nth
    n: 100
"#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            logging: LoggingConfig,
        }

        let wrapper: Wrapper = serde_yaml::from_str(yaml).unwrap();
        let config = wrapper.logging;

        assert!(config.sampling.enabled);
        assert_eq!(config.sampling.strategy, SamplingStrategy::EveryNth);
        assert_eq!(config.sampling.n, 100);
    }

    #[test]
    fn test_yaml_sampling_rate_limit() {
        let yaml = r#"
logging:
  enabled: true
  level: warn
  format: json
  sampling:
    enabled: true
    strategy: rate_limit
    max_events_per_second: 1000
"#;

        #[derive(Debug, Deserialize)]
        struct Wrapper {
            logging: LoggingConfig,
        }

        let wrapper: Wrapper = serde_yaml::from_str(yaml).unwrap();
        let config = wrapper.logging;

        assert!(config.sampling.enabled);
        assert_eq!(config.sampling.strategy, SamplingStrategy::RateLimit);
        assert_eq!(config.sampling.max_events_per_second, 1000);
    }

    #[test]
    fn test_validation_probabilistic_rate() {
        let config = LoggingConfig {
            sampling: SamplingConfig {
                enabled: true,
                strategy: SamplingStrategy::Probabilistic,
                rate: 1.5,
                ..SamplingConfig::default()
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_every_nth_zero() {
        let config = LoggingConfig {
            sampling: SamplingConfig {
                enabled: true,
                strategy: SamplingStrategy::EveryNth,
                n: 0,
                ..SamplingConfig::default()
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_rate_limit_zero() {
        let config = LoggingConfig {
            sampling: SamplingConfig {
                enabled: true,
                strategy: SamplingStrategy::RateLimit,
                max_events_per_second: 0,
                ..SamplingConfig::default()
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_empty_outputs() {
        let config = LoggingConfig {
            enabled: true,
            output: OutputConfig {
                targets: vec![],
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_retention_days_zero() {
        let config = LoggingConfig {
            retention: RetentionConfig {
                enabled: true,
                days: 0,
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_rotation_size_zero() {
        let config = LoggingConfig {
            rotation: RotationConfig {
                enabled: true,
                max_size_mb: 0,
                ..RotationConfig::default()
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_buffering_batch_zero() {
        let config = LoggingConfig {
            buffering: BufferingConfig {
                enabled: true,
                batch_size: 0,
                ..BufferingConfig::default()
            },
            ..LoggingConfig::default()
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_success_defaults() {
        let config = LoggingConfig::defaults();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_module_trait() {
        let config = LoggingConfig::defaults();
        assert!(config.enabled);
    }
}
