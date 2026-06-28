use std::collections::HashMap;
use std::env;
use std::fs;

use serde_json::Value;

use config_core::ConfigError;
use config_core::ConfigurationSource;

pub mod key_value_map;

#[derive(Debug, Clone)]
pub struct DefaultsSource;

impl DefaultsSource {
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
        let config = HashMap::new();
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentSource {
    prefix: Option<String>,
}

impl EnvironmentSource {
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

        for (key, value) in env::vars() {
            if let Some(ref prefix) = self.prefix {
                if key.starts_with(prefix) {
                    let config_key = &key[prefix.len()..].to_lowercase();
                    config.insert(config_key.to_string(), Value::String(value));
                }
            } else {
                let config_key = key.to_lowercase();
                config.insert(config_key, Value::String(value));
            }
        }

        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

#[derive(Debug, Clone)]
pub struct DotenvSource {
    path: String,
    optional: bool,
}

impl DotenvSource {
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

        match fs::read_to_string(&self.path) {
            Ok(content) => {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"');
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

#[derive(Debug, Clone)]
pub struct TomlFileSource {
    path: String,
    optional: bool,
}

impl TomlFileSource {
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

        match fs::read_to_string(&self.path) {
            Ok(content) => {
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

impl Default for DefaultsSource {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EnvironmentSource {
    fn default() -> Self {
        Self::new(None)
    }
}

#[derive(Debug, Clone)]
pub struct YamlFileSource {
    path: String,
    optional: bool,
}

impl YamlFileSource {
    pub fn new<P: Into<String>>(path: P, optional: bool) -> Self {
        Self { path: path.into(), optional }
    }
}

impl ConfigurationSource for YamlFileSource {
    fn name(&self) -> &'static str {
        "yaml"
    }

    fn priority(&self) -> u8 {
        200
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                let value: Value = serde_yml::from_str(&content)
                    .map_err(|e| ConfigError::SourceError(format!("Failed to parse YAML file {}: {}", self.path, e)))?;
                Ok(value)
            }
            Err(e) => {
                if self.optional {
                    Ok(Value::Object(serde_json::Map::new()))
                } else {
                    Err(ConfigError::SourceError(format!("Failed to read YAML file {}: {}", self.path, e)))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonFileSource {
    path: String,
    optional: bool,
}

impl JsonFileSource {
    pub fn new<P: Into<String>>(path: P, optional: bool) -> Self {
        Self { path: path.into(), optional }
    }
}

impl ConfigurationSource for JsonFileSource {
    fn name(&self) -> &'static str {
        "json"
    }

    fn priority(&self) -> u8 {
        200
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                let value: Value = serde_json::from_str(&content)
                    .map_err(|e| ConfigError::SourceError(format!("Failed to parse JSON file {}: {}", self.path, e)))?;
                Ok(value)
            }
            Err(e) => {
                if self.optional {
                    Ok(Value::Object(serde_json::Map::new()))
                } else {
                    Err(ConfigError::SourceError(format!("Failed to read JSON file {}: {}", self.path, e)))
                }
            }
        }
    }
}
