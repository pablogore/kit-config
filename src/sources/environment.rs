use std::collections::HashMap;
use serde_json::Value;
use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;

/// Environment variable configuration source
pub struct EnvironmentSource {
    prefix: Option<String>,
}

impl EnvironmentSource {
    pub fn new(prefix: Option<impl Into<String>>) -> Self {
        Self {
            prefix: prefix.map(Into::into),
        }
    }
}

impl ConfigurationSource for EnvironmentSource {
    fn load(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut result = HashMap::new();
        
        // Get all environment variables
        for (key, value) in std::env::vars() {
            // If we have a prefix, only include variables with that prefix
            if let Some(ref prefix) = self.prefix {
                if key.starts_with(prefix) {
                    // Remove prefix from key
                    let clean_key = key[prefix.len()..].to_string();
                    result.insert(clean_key, Value::String(value));
                }
            } else {
                // No prefix, include all environment variables
                result.insert(key, Value::String(value));
            }
        }
        
        Ok(result)
    }
    
    fn priority(&self) -> u8 {
        30 // Lowest priority
    }
    
    fn is_optional(&self) -> bool {
        true
    }
}