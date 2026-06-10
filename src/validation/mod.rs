/// Validation error structure
/// 
/// Represents a single validation error with field information and error message.
/// 
/// # Examples
/// 
/// Creating a validation error:
/// ```
/// use kit_config::validation::ValidationError;
/// 
/// let error = ValidationError::new("port", "Port must be between 1 and 65535");
/// ```
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl ValidationError {
    /// Creates a new validation error
    /// 
    /// # Arguments
    /// * `field` - The field name that failed validation
    /// * `message` - The error message
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::ValidationError;
    /// 
    /// let error = ValidationError::new("port", "Port must be between 1 and 65535");
    /// ```
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: None,
        }
    }
    
    /// Sets an error code for this validation error
    /// 
    /// # Arguments
    /// * `code` - Error code to associate with this validation error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::ValidationError;
    /// 
    /// let error = ValidationError::new("port", "Port must be between 1 and 65535")
    ///     .with_code("INVALID_PORT");
    /// ```
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// Validation report containing all validation errors
/// 
/// This structure collects all validation errors from different validation layers
/// (framework, application, domain) and provides methods to access them.
/// 
/// # Examples
/// 
/// Creating a validation report:
/// ```
/// use kit_config::validation::ValidationReport;
/// 
/// let report = ValidationReport::new();
/// ```
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub framework_errors: Vec<ValidationError>,
    pub application_errors: Vec<ValidationError>,
    pub domain_errors: Vec<ValidationError>,
    pub is_valid: bool,
}

impl ValidationReport {
    /// Creates a new empty validation report
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::ValidationReport;
    /// 
    /// let report = ValidationReport::new();
    /// ```
    pub fn new() -> Self {
        Self {
            framework_errors: Vec::new(),
            application_errors: Vec::new(),
            domain_errors: Vec::new(),
            is_valid: true,
        }
    }
    
    /// Adds a framework validation error to the report
    /// 
    /// # Arguments
    /// * `error` - The validation error to add
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::{ValidationReport, ValidationError};
    /// 
    /// let mut report = ValidationReport::new();
    /// let error = ValidationError::new("field", "Framework error");
    /// report.add_framework_error(error);
    /// ```
    pub fn add_framework_error(&mut self, error: ValidationError) {
        self.framework_errors.push(error);
        self.is_valid = false;
    }
    
    /// Adds an application validation error to the report
    /// 
    /// # Arguments
    /// * `error` - The validation error to add
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::{ValidationReport, ValidationError};
    /// 
    /// let mut report = ValidationReport::new();
    /// let error = ValidationError::new("field", "Application error");
    /// report.add_application_error(error);
    /// ```
    pub fn add_application_error(&mut self, error: ValidationError) {
        self.application_errors.push(error);
        self.is_valid = false;
    }
    
    /// Adds a domain validation error to the report
    /// 
    /// # Arguments
    /// * `error` - The validation error to add
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::{ValidationReport, ValidationError};
    /// 
    /// let mut report = ValidationReport::new();
    /// let error = ValidationError::new("field", "Domain error");
    /// report.add_domain_error(error);
    /// ```
    pub fn add_domain_error(&mut self, error: ValidationError) {
        self.domain_errors.push(error);
        self.is_valid = false;
    }
    
    /// Gets all validation errors from all layers
    /// 
    /// # Returns
    /// * `Vec<&ValidationError>` - Vector of references to all validation errors
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::ValidationReport;
    /// 
    /// let report = ValidationReport::new();
    /// let errors = report.all_errors();
    /// ```
    pub fn all_errors(&self) -> Vec<&ValidationError> {
        let mut errors = Vec::new();
        errors.extend(self.framework_errors.iter());
        errors.extend(self.application_errors.iter());
        errors.extend(self.domain_errors.iter());
        errors
    }
}

/// Validation trait for configuration structs
/// 
/// Implement this trait to add validation capabilities to your configuration structs.
/// The validation is layered: framework, application, and domain validation.
/// 
/// # Examples
/// 
/// Implementing validation for a configuration struct:
/// ```
/// use kit_config::validation::{Validation, ValidationReport, ValidationError};
/// 
/// #[derive(Debug, serde::Deserialize, serde::Serialize)]
/// struct MyConfig {
///     port: u16,
/// }
/// 
/// impl Validation for MyConfig {
///     fn validate(&self) -> Result<(), ValidationReport> {
///         let mut report = ValidationReport::new();
///         
///         if self.port == 0 {
///             report.add_domain_error(ValidationError::new("port", "Port cannot be 0"));
///         }
///         
///         if report.is_valid {
///             Ok(())
///         } else {
///             Err(report)
///         }
///     }
/// }
/// ```
pub trait Validation {
    /// Validate the configuration and return errors if any
    /// 
    /// # Returns
    /// * `Result<(), ValidationReport>` - Ok if valid, Err with validation report if invalid
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::validation::Validation;
    /// 
    /// // Assuming you have a struct that implements Validation
    /// // let config = MyConfig::defaults();
    /// // let result = config.validate();
    /// ```
    fn validate(&self) -> Result<(), ValidationReport>;
}