use std::collections::HashMap;
use serde_json::Value;
use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;
use crate::validation::Validation;

/// Postgres configuration module
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PostgresModule {
    pub connection_string: String,
    pub pool_size: u32,
    pub ssl_mode: String,
}

impl PostgresModule {
    pub fn defaults() -> Self {
        Self {
            connection_string: "postgresql://localhost/postgres".to_string(),
            pool_size: 10,
            ssl_mode: "prefer".to_string(),
        }
    }
}

impl Validation for PostgresModule {
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
        
        // Validate SSL mode is one of the allowed values
        let allowed_ssl_modes = ["disable", "allow", "prefer", "require", "verify-ca", "verify-full"];
        if !allowed_ssl_modes.contains(&self.ssl_mode.as_str()) {
            report.add_domain_error(crate::validation::ValidationError::new("ssl_mode", "SSL mode must be one of: disable, allow, prefer, require, verify-ca, verify-full"));
        }
        
        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}