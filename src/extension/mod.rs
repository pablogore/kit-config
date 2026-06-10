//! Extension trait for configuration modules
pub trait Extension {
    /// Register custom source
    fn register_source(&mut self, source: Box<dyn crate::sources::ConfigurationSource>);
    
    /// Register custom validator
    fn register_validator(&mut self, validator: Box<dyn crate::validation::Validation>);
}