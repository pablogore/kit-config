use std::collections::HashMap;
use std::fs;
use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;
use serde_json::Value;

/// TOML file configuration source
pub struct TomlFileSource {
    file_path: String,
    optional: bool,
}

impl TomlFileSource {
    pub fn new(file_path: impl Into<String>, optional: bool) -> Self {
        Self {
            file_path: file_path.into(),
            optional,
        }
    }
}

impl ConfigurationSource for TomlFileSource {
    fn load(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| ConfigError::Load(format!("Failed to read TOML file: {}", e)))?;
        
        let parsed: toml::Value = toml::from_str(&content)
            .map_err(|e| ConfigError::Load(format!("Failed to parse TOML file: {}", e)))?;
        
        // Convert to HashMap<String, Value>
        let mut result = HashMap::new();
        if let toml::Value::Table(table) = parsed {
            for (key, value) in table {
                result.insert(key, convert_toml_value(value));
            }
        }
        
        Ok(result)
    }
    
    fn priority(&self) -> u8 {
        2 // Lower priority than defaults
    }
    
    fn is_optional(&self) -> bool {
        self.optional
    }
}

fn convert_toml_value(value: toml::Value) -> Value {
    match value {
        toml::Value::String(s) => Value::String(s),
        toml::Value::Integer(i) => Value::Number(i.into()),
        toml::Value::Float(f) => Value::Number(serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0))),
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Datetime(d) => Value::String(d.to_string()),
        toml::Value::Array(a) => {
            let mut vec = Vec::new();
            for item in a {
                vec.push(convert_toml_value(item));
            }
            Value::Array(vec)
        }
        toml::Value::Table(t) => {
            let mut map = serde_json::Map::new();
            for (key, value) in t {
                map.insert(key, convert_toml_value(value));
            }
            Value::Object(map)
        }
    }
}