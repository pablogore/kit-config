use thiserror::Error;

use crate::validation::ValidationReport;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration source error: {0}")]
    SourceError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration not found: {0}")]
    NotFound(String),

    #[error("Load error: {0}")]
    Load(String),

    #[error("Configuration validation failed: {0:?}")]
    Validation(ValidationReport),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::{ValidationError as DomainValidationError, ValidationReport};

    #[test]
    fn validation_report_wraps_into_config_error_validation_variant() {
        let mut report = ValidationReport::new();
        report.add_domain_error(DomainValidationError::new(
            "sampling.rate",
            "Sampling rate must be between 0.0 and 1.0",
        ));

        let error = ConfigError::Validation(report);

        match error {
            ConfigError::Validation(report) => {
                assert!(!report.is_valid);
                assert_eq!(report.domain_errors.len(), 1);
                assert_eq!(report.domain_errors[0].field, "sampling.rate");
            }
            other => panic!("expected ConfigError::Validation, got {other:?}"),
        }
    }
}
