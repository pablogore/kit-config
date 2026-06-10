use config_core::Validation;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct HttpModule {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

impl HttpModule {
    pub fn defaults() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

impl Validation for HttpModule {
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

        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct HttpsModule {
    pub host: String,
    pub port: u16,
    pub cert_path: String,
    pub key_path: String,
}

impl HttpsModule {
    pub fn defaults() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8443,
            cert_path: "/etc/ssl/certs/server.crt".to_string(),
            key_path: "/etc/ssl/private/server.key".to_string(),
        }
    }
}

impl Validation for HttpsModule {
    fn validate(&self) -> Result<(), config_core::ValidationReport> {
        let mut report = config_core::ValidationReport::new();

        if self.port == 0 {
            report.add_domain_error(config_core::ValidationError::new("port", "Port cannot be 0"));
        }

        if self.host.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("host", "Host cannot be empty"));
        }

        if self.cert_path.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("cert_path", "Certificate path cannot be empty"));
        }

        if self.key_path.is_empty() {
            report.add_domain_error(config_core::ValidationError::new("key_path", "Key path cannot be empty"));
        }

        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}
