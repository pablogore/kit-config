use config_core::{ConfigModule, Validation};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RedisModule {
    pub connection_string: String,
    pub pool_size: u32,
    pub tls_enabled: bool,
}

impl ConfigModule for RedisModule {
    const NAME: &'static str = "redis";

    fn defaults() -> Self {
        Self {
            connection_string: "redis://localhost:6379".to_string(),
            pool_size: 5,
            tls_enabled: false,
        }
    }
}

impl Validation for RedisModule {
    fn validate(&self) -> Result<(), config_core::ValidationReport> {
        let mut report = config_core::ValidationReport::new();

        if self.connection_string.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("connection_string", "Connection string cannot be empty"));
        }

        if self.pool_size == 0 {
            report.add_domain_error(config_core::ValidationError::new("pool_size", "Pool size must be greater than 0"));
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
    fn test_config_module_name() {
        assert_eq!(RedisModule::NAME, "redis");
    }
}
