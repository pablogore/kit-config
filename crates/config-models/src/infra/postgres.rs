use config_core::Validation;

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
    fn validate(&self) -> Result<(), config_core::ValidationReport> {
        let mut report = config_core::ValidationReport::new();

        if self.connection_string.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("connection_string", "Connection string cannot be empty"));
        }

        if self.pool_size == 0 {
            report.add_domain_error(config_core::ValidationError::new("pool_size", "Pool size must be greater than 0"));
        }

        let allowed_ssl_modes = ["disable", "allow", "prefer", "require", "verify-ca", "verify-full"];
        if !allowed_ssl_modes.contains(&self.ssl_mode.as_str()) {
            report.add_domain_error(config_core::ValidationError::new("ssl_mode", "SSL mode must be one of: disable, allow, prefer, require, verify-ca, verify-full"));
        }

        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}
