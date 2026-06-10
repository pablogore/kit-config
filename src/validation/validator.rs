use std::collections::HashMap;
use serde_json::Value;
use crate::validation::ValidationReport;

/// Validation trait for configuration structs
pub trait Validation {
    /// Validate the configuration and return errors if any
    fn validate(&self) -> Result<(), ValidationReport>;
}