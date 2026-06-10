use std::collections::HashMap;
use serde_json::Value;
use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;
use crate::validation::Validation;

/// Redis configuration module
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RedisModule {
    pub connection_string: String,
    pub pool_size: u32,
    pub tls_enabled: bool,
}

impl RedisModule {
    pub fn defaults() -> Self {
        Self {
            connection_string: "redis://localhost:6379".to_string(),
            pool_size: 5,
            tls_enabled: false,
        }
    }
}

impl Validation for RedisModule {
    fn validate(&self) -> Result<(), crate::validation::ValidationReport> {
        let mut report = crate::validation::ValidationReport::new();
        
        // Validate connection string is not empty
        if self.connection_string.is_empty() {
            report.add_domain_error(crate::validation::ValidationError::new("connection_string", "Connection string cannot be empty"));
        }
        
        // Validate pool size is reasonable
        if self.pool_size == 0 {
            report.add_domain_error(crate::validation::ValidationError::new("pool_size", "Pool size must be greater than 0"));
        }
        
        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}