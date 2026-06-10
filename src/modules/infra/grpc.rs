
use crate::validation::Validation;

/// gRPC configuration module
/// 
/// This module provides configuration for gRPC services including host, port,
/// TLS settings, and connection parameters.
/// 
/// # Examples
/// 
/// Creating a gRPC module with defaults:
/// ```
/// use kit_config::modules::infra::grpc::GrpcModule;
/// 
/// let grpc_config = GrpcModule::defaults();
/// ```
/// 
/// Creating a gRPC module with custom values:
/// ```
/// use kit_config::modules::infra::grpc::GrpcModule;
/// 
/// let grpc_config = GrpcModule {
///     host: "localhost".to_string(),
///     port: 50051,
///     tls_enabled: true,
///     tls_cert_path: Some("/path/to/cert.pem".to_string()),
///     tls_key_path: Some("/path/to/key.pem".to_string()),
///     keepalive_time: Some(30),
///     keepalive_timeout: Some(5),
/// };
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct GrpcModule {
    /// Host address to bind the gRPC server
    pub host: String,
    /// Port number to bind the gRPC server
    pub port: u16,
    /// Enable TLS for gRPC connections
    pub tls_enabled: bool,
    /// Path to TLS certificate file (required when TLS is enabled)
    pub tls_cert_path: Option<String>,
    /// Path to TLS key file (required when TLS is enabled)
    pub tls_key_path: Option<String>,
    /// Keepalive time in seconds (optional)
    pub keepalive_time: Option<u32>,
    /// Keepalive timeout in seconds (optional)
    pub keepalive_timeout: Option<u32>,
}

impl GrpcModule {
    /// Get default gRPC configuration values
    /// 
    /// # Returns
    /// * `GrpcModule` - A new instance with default values
    /// 
    /// # Examples
    /// 
    /// ```
/// use kit_config::modules::infra::grpc::GrpcModule;
/// 
/// let defaults = GrpcModule::defaults();
    /// ```
    pub fn defaults() -> Self {
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
    /// Validate the gRPC configuration
    /// 
    /// # Returns
    /// * `Result<(), crate::validation::ValidationReport>` - Ok if valid, Err with validation report if invalid
    /// 
    /// # Examples
    /// 
    /// ```
/// use kit_config::modules::infra::grpc::GrpcModule;
/// use kit_config::validation::Validation;
/// 
/// let config = GrpcModule::defaults();
/// let result = config.validate();
    /// ```
    fn validate(&self) -> Result<(), crate::validation::ValidationReport> {
        let mut report = crate::validation::ValidationReport::new();
        
        // Validate port range
        if self.port == 0 {
            report.add_domain_error(crate::validation::ValidationError::new("port", "Port cannot be 0"));
        }
        
        // Validate host is not empty
        if self.host.is_empty() {
            report.add_domain_error(crate::validation::ValidationError::new("host", "Host cannot be empty"));
        }
        
        // Validate TLS certificate paths if TLS is enabled
        if self.tls_enabled {
            if self.tls_cert_path.is_none() {
                report.add_domain_error(crate::validation::ValidationError::new("tls_cert_path", "TLS certificate path required when TLS is enabled"));
            }
            if self.tls_key_path.is_none() {
                report.add_domain_error(crate::validation::ValidationError::new("tls_key_path", "TLS key path required when TLS is enabled"));
            }
        }
        
        // Validate keepalive values if provided
        if let Some(time) = self.keepalive_time {
            if time == 0 {
                report.add_domain_error(crate::validation::ValidationError::new("keepalive_time", "Keepalive time cannot be 0"));
            }
        }
        
        if let Some(timeout) = self.keepalive_timeout {
            if timeout == 0 {
                report.add_domain_error(crate::validation::ValidationError::new("keepalive_timeout", "Keepalive timeout cannot be 0"));
            }
        }
        
        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}

/// gRPC client configuration module
/// 
/// This module provides configuration for gRPC clients including target address,
/// TLS settings, and connection parameters.
/// 
/// # Examples
/// 
/// Creating a gRPC client module with defaults:
/// ```
/// use kit_config::modules::infra::grpc::GrpcClientModule;
/// 
/// let grpc_client_config = GrpcClientModule::defaults();
/// ```
/// 
/// Creating a gRPC client module with custom values:
/// ```
/// use kit_config::modules::infra::grpc::GrpcClientModule;
/// 
/// let grpc_client_config = GrpcClientModule {
///     target: "localhost:50051".to_string(),
///     tls_enabled: true,
///     tls_cert_path: Some("/path/to/cert.pem".to_string()),
///     keepalive_time: Some(30),
///     keepalive_timeout: Some(5),
/// };
/// ```
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct GrpcClientModule {
    /// Target address for the gRPC connection
    pub target: String,
    /// Enable TLS for gRPC connections
    pub tls_enabled: bool,
    /// Path to TLS certificate file (optional)
    pub tls_cert_path: Option<String>,
    /// Keepalive time in seconds (optional)
    pub keepalive_time: Option<u32>,
    /// Keepalive timeout in seconds (optional)
    pub keepalive_timeout: Option<u32>,
}

impl GrpcClientModule {
    /// Get default gRPC client configuration values
    /// 
    /// # Returns
    /// * `GrpcClientModule` - A new instance with default values
    /// 
    /// # Examples
    /// 
    /// ```
/// use kit_config::modules::infra::grpc::GrpcClientModule;
/// 
/// let defaults = GrpcClientModule::defaults();
    /// ```
    pub fn defaults() -> Self {
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
    /// Validate the gRPC client configuration
    /// 
    /// # Returns
    /// * `Result<(), crate::validation::ValidationReport>` - Ok if valid, Err with validation report if invalid
    /// 
    /// # Examples
    /// 
    /// ```
/// use kit_config::modules::infra::grpc::GrpcClientModule;
/// use kit_config::validation::Validation;
/// 
/// let config = GrpcClientModule::defaults();
/// let result = config.validate();
    /// ```
    fn validate(&self) -> Result<(), crate::validation::ValidationReport> {
        let mut report = crate::validation::ValidationReport::new();
        
        // Validate target is not empty
        if self.target.is_empty() {
            report.add_domain_error(crate::validation::ValidationError::new("target", "Target cannot be empty"));
        }
        
        // Validate keepalive values if provided
        if let Some(time) = self.keepalive_time {
            if time == 0 {
                report.add_domain_error(crate::validation::ValidationError::new("keepalive_time", "Keepalive time cannot be 0"));
            }
        }
        
        if let Some(timeout) = self.keepalive_timeout {
            if timeout == 0 {
                report.add_domain_error(crate::validation::ValidationError::new("keepalive_timeout", "Keepalive timeout cannot be 0"));
            }
        }
        
        if report.is_valid {
            Ok(())
        } else {
            Err(report)
        }
    }
}