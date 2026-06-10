#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: Option<String>,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub framework_errors: Vec<ValidationError>,
    pub application_errors: Vec<ValidationError>,
    pub domain_errors: Vec<ValidationError>,
    pub is_valid: bool,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            framework_errors: Vec::new(),
            application_errors: Vec::new(),
            domain_errors: Vec::new(),
            is_valid: true,
        }
    }

    pub fn add_framework_error(&mut self, error: ValidationError) {
        self.framework_errors.push(error);
        self.is_valid = false;
    }

    pub fn add_application_error(&mut self, error: ValidationError) {
        self.application_errors.push(error);
        self.is_valid = false;
    }

    pub fn add_domain_error(&mut self, error: ValidationError) {
        self.domain_errors.push(error);
        self.is_valid = false;
    }

    pub fn all_errors(&self) -> Vec<&ValidationError> {
        let mut errors = Vec::new();
        errors.extend(self.framework_errors.iter());
        errors.extend(self.application_errors.iter());
        errors.extend(self.domain_errors.iter());
        errors
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Validation {
    fn validate(&self) -> Result<(), ValidationReport>;
}
