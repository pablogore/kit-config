use std::collections::HashMap;
use std::env;
use std::fs;

use serde_json::Value;

use crate::errors::ConfigError;

pub mod key_value_map;

/// Configuration source trait
pub trait ConfigurationSource: Send + Sync {
    /// Returns the name of this source
    fn name(&self) -> &'static str;
    
    /// Returns the priority of this source (higher priority sources override lower priority ones)
    fn priority(&self) -> u8;
    
    /// Returns whether this source is optional (if false, loading will fail if source is not available)
    fn is_optional(&self) -> bool;
    
    /// Loads configuration from this source
    fn load(&self) -> Result<Value, ConfigError>;
}

/// Default configuration source
#[derive(Debug, Clone)]
pub struct DefaultsSource;

impl DefaultsSource {
    /// Creates a new defaults source
    pub fn new() -> Self {
        Self
    }
}

impl ConfigurationSource for DefaultsSource {
    fn name(&self) -> &'static str {
        "defaults"
    }

    fn priority(&self) -> u8 {
        0
    }

    fn is_optional(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Value, ConfigError> {
        // Default configuration values
        let config = HashMap::new();
        
        // Add default values here if needed
        // For now, return empty config
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// Environment variable configuration source
#[derive(Debug, Clone)]
pub struct EnvironmentSource {
    prefix: Option<String>,
}

impl EnvironmentSource {
    /// Creates a new environment source
    pub fn new(prefix: Option<String>) -> Self {
        Self { prefix }
    }
}

impl ConfigurationSource for EnvironmentSource {
    fn name(&self) -> &'static str {
        "environment"
    }

    fn priority(&self) -> u8 {
        50
    }

    fn is_optional(&self) -> bool {
        true
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Load environment variables
        for (key, value) in env::vars() {
            // If prefix is set, only load variables with that prefix
            if let Some(ref prefix) = self.prefix {
                if key.starts_with(prefix) {
                    let config_key = &key[prefix.len()..].to_lowercase();
                    config.insert(config_key.to_string(), Value::String(value));
                }
            } else {
                // No prefix, load all environment variables
                let config_key = key.to_lowercase();
                config.insert(config_key, Value::String(value));
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// Dotenv file configuration source
#[derive(Debug, Clone)]
pub struct DotenvSource {
    path: String,
    optional: bool,
}

impl DotenvSource {
    /// Creates a new dotenv source
    pub fn new<P: Into<String>>(path: P, optional: bool) -> Self {
        Self {
            path: path.into(),
            optional,
        }
    }
}

impl ConfigurationSource for DotenvSource {
    fn name(&self) -> &'static str {
        "dotenv"
    }

    fn priority(&self) -> u8 {
        100
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Try to read the dotenv file
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                // Parse dotenv content using dotenvy
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"');
                        // Preserve original case of keys (fix for the issue)
                        config.insert(key.to_string(), Value::String(value.to_string()));
                    }
                }
            }
            Err(e) => {
                if !self.optional {
                    return Err(ConfigError::SourceError(format!(
                        "Failed to read dotenv file {}: {}",
                        self.path, e
                    )));
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// TOML file configuration source
#[derive(Debug, Clone)]
pub struct TomlFileSource {
    path: String,
    optional: bool,
}

impl TomlFileSource {
    /// Creates a new TOML file source
    pub fn new<P: Into<String>>(path: P, optional: bool) -> Self {
        Self {
            path: path.into(),
            optional,
        }
    }
}

impl ConfigurationSource for TomlFileSource {
    fn name(&self) -> &'static str {
        "toml"
    }

    fn priority(&self) -> u8 {
        200
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Try to read the TOML file
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                // Parse TOML content
                if let Ok(toml) = toml::from_str::<toml::Value>(&content) {
                    if let Ok(converted) = convert_toml_value(&toml) {
                        if let Value::Object(map) = converted {
                            config.extend(map);
                        }
                    }
                }
            }
            Err(e) => {
                if !self.optional {
                    return Err(ConfigError::SourceError(format!(
                        "Failed to read TOML file {}: {}",
                        self.path, e
                    )));
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// Convert TOML value to JSON value
fn convert_toml_value(value: &toml::Value) -> Result<Value, ConfigError> {
    match value {
        toml::Value::String(s) => Ok(Value::String(s.clone())),
        toml::Value::Integer(i) => Ok(Value::Number((*i).into())),
        toml::Value::Float(f) => Ok(Value::Number(serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)))),
        toml::Value::Boolean(b) => Ok(Value::Bool(*b)),
        toml::Value::Datetime(d) => Ok(Value::String(d.to_string())),
        toml::Value::Array(a) => {
            let mut vec = Vec::new();
            for item in a {
                vec.push(convert_toml_value(item)?);
            }
            Ok(Value::Array(vec))
        }
        toml::Value::Table(t) => {
            let mut map = HashMap::new();
            for (key, value) in t {
                map.insert(key.clone(), convert_toml_value(value)?);
            }
            Ok(Value::Object(map.into_iter().collect()))
        }
    }
}