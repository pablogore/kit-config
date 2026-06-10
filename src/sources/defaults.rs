use std::collections::HashMap;
use serde_json::Value;
use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;

/// Default configuration source with built-in defaults
pub struct DefaultsSource;

impl ConfigurationSource for DefaultsSource {
    fn load(&self) -> Result<HashMap<String, Value>, ConfigError> {
        // For now, return empty defaults - in a real implementation, this would
        // return the framework's built-in default values
        Ok(HashMap::new())
    }
    
    fn priority(&self) -> u8 {
        0 // Highest priority
    }
    
    fn is_optional(&self) -> bool {
        true
    }
}