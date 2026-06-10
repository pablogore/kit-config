use std::collections::HashMap;

use serde_json::Value;

use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;

/// Key-value map configuration source
#[derive(Debug, Clone)]
pub struct KeyValueMapSource {
    /// Prefix for configuration keys
    prefix: String,
    /// Key-value map string
    map_str: String,
    /// Whether this source is optional
    optional: bool,
}

impl KeyValueMapSource {
    /// Creates a new key-value map source
    pub fn new(prefix: &str, map_str: &str, optional: bool) -> Self {
        Self {
            prefix: prefix.to_lowercase(),
            map_str: map_str.to_string(),
            optional,
        }
    }
}

impl ConfigurationSource for KeyValueMapSource {
    fn name(&self) -> &'static str {
        "key_value_map"
    }

    fn priority(&self) -> u8 {
        100
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Parse key-value pairs from the map string
        if !self.map_str.trim().is_empty() {
            for pair in self.map_str.split(',') {
                let trimmed_pair = pair.trim();
                if trimmed_pair.is_empty() {
                    continue;
                }
                
                if let Some((key, value)) = trimmed_pair.split_once('=') {
                    let key = format!("{}.{}", self.prefix, key.trim().to_lowercase());
                    let value = value.trim();
                    
                    // Try to parse as JSON for better type handling
                    if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                        config.insert(key, json_value);
                    } else {
                        // If not valid JSON, treat as string
                        config.insert(key, Value::String(value.to_string()));
                    }
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}