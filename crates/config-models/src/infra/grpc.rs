use config_core::{ConfigModule, Validation};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct GrpcModule {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub keepalive_time: Option<u32>,
    pub keepalive_timeout: Option<u32>,
}

impl ConfigModule for GrpcModule {
    const NAME: &'static str = "grpc";

    fn defaults() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 50051,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            keepalive_time: None,
            keepalive_timeout: None,
        }
    }
}

impl Validation for GrpcModule {
    fn validate(&self) -> Result<(), config_core::ValidationReport> {
        let mut report = config_core::ValidationReport::new();

        if self.port == 0 {
            report.add_domain_error(config_core::ValidationError::new("port", "Port cannot be 0"));
        }

        if self.host.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("host", "Host cannot be empty"));
        }

        if self.tls_enabled {
            if self.tls_cert_path.is_none() {
                report.add_domain_error(config_core::ValidationError::new("tls_cert_path", "TLS certificate path required when TLS is enabled"));
            }
            if self.tls_key_path.is_none() {
                report.add_domain_error(config_core::ValidationError::new("tls_key_path", "TLS key path required when TLS is enabled"));
            }
        }

        if let Some(time) = self.keepalive_time {
            if time == 0 {
                report.add_domain_error(config_core::ValidationError::new("keepalive_time", "Keepalive time cannot be 0"));
            }
        }

        if let Some(timeout) = self.keepalive_timeout {
            if timeout == 0 {
                report.add_domain_error(config_core::ValidationError::new("keepalive_timeout", "Keepalive timeout cannot be 0"));
            }
        }

        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct GrpcClientModule {
    pub target: String,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub keepalive_time: Option<u32>,
    pub keepalive_timeout: Option<u32>,
}

impl ConfigModule for GrpcClientModule {
    const NAME: &'static str = "grpc-client";

    fn defaults() -> Self {
        Self {
            target: "localhost:50051".to_string(),
            tls_enabled: false,
            tls_cert_path: None,
            keepalive_time: None,
            keepalive_timeout: None,
        }
    }
}

impl Validation for GrpcClientModule {
    fn validate(&self) -> Result<(), config_core::ValidationReport> {
        let mut report = config_core::ValidationReport::new();

        if self.target.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("target", "Target cannot be empty"));
        }

        if let Some(time) = self.keepalive_time {
            if time == 0 {
                report.add_domain_error(config_core::ValidationError::new("keepalive_time", "Keepalive time cannot be 0"));
            }
        }

        if let Some(timeout) = self.keepalive_timeout {
            if timeout == 0 {
                report.add_domain_error(config_core::ValidationError::new("keepalive_timeout", "Keepalive timeout cannot be 0"));
            }
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
    fn test_grpc_config_module_name() {
        assert_eq!(GrpcModule::NAME, "grpc");
    }

    #[test]
    fn test_grpc_client_config_module_name() {
        assert_eq!(GrpcClientModule::NAME, "grpc-client");
    }
}
