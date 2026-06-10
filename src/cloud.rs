use std::collections::HashMap;
use std::env;
use serde_json::Value;

use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;

/// AWS configuration source
#[derive(Debug, Clone)]
pub struct AwsSource {
    /// Whether this source is optional
    optional: bool,
}

impl AwsSource {
    /// Creates a new AWS source
    pub fn new(optional: bool) -> Self {
        Self { optional }
    }
}

impl ConfigurationSource for AwsSource {
    fn name(&self) -> &'static str {
        "aws"
    }

    fn priority(&self) -> u8 {
        150
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Load AWS profile
        if let Ok(profile) = env::var("AWS_PROFILE") {
            config.insert("aws.profile".to_string(), Value::String(profile));
        }
        
        // Load AWS region
        if let Ok(region) = env::var("AWS_REGION") {
            config.insert("aws.region".to_string(), Value::String(region));
        }
        
        // Load AWS access key ID
        if let Ok(access_key_id) = env::var("AWS_ACCESS_KEY_ID") {
            config.insert("aws.access_key_id".to_string(), Value::String(access_key_id));
        }
        
        // Load AWS secret access key
        if let Ok(secret_access_key) = env::var("AWS_SECRET_ACCESS_KEY") {
            config.insert("aws.secret_access_key".to_string(), Value::String(secret_access_key));
        }
        
        // Load AWS config map from environment variable
        if let Ok(config_map_str) = env::var("AWS_CONFIG_MAP") {
            if let Ok(map) = parse_key_value_map(&config_map_str) {
                for (key, value) in map {
                    config.insert(key, value);
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// DigitalOcean configuration source
#[derive(Debug, Clone)]
pub struct DigitalOceanSource {
    /// Whether this source is optional
    optional: bool,
}

impl DigitalOceanSource {
    /// Creates a new DigitalOcean source
    pub fn new(optional: bool) -> Self {
        Self { optional }
    }
}

impl ConfigurationSource for DigitalOceanSource {
    fn name(&self) -> &'static str {
        "digitalocean"
    }

    fn priority(&self) -> u8 {
        150
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Load DigitalOcean token
        if let Ok(token) = env::var("DO_TOKEN") {
            config.insert("digitalocean.token".to_string(), Value::String(token));
        }
        
        // Load DigitalOcean config map from environment variable
        if let Ok(config_map_str) = env::var("DO_CONFIG_MAP") {
            if let Ok(map) = parse_key_value_map(&config_map_str) {
                for (key, value) in map {
                    config.insert(key, value);
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// GCP configuration source
#[derive(Debug, Clone)]
pub struct GcpSource {
    /// Whether this source is optional
    optional: bool,
}

impl GcpSource {
    /// Creates a new GCP source
    pub fn new(optional: bool) -> Self {
        Self { optional }
    }
}

impl ConfigurationSource for GcpSource {
    fn name(&self) -> &'static str {
        "gcp"
    }

    fn priority(&self) -> u8 {
        150
    }

    fn is_optional(&self) -> bool {
        self.optional
    }

    fn load(&self) -> Result<Value, ConfigError> {
        let mut config = HashMap::new();
        
        // Load GCP project ID
        if let Ok(project_id) = env::var("GOOGLE_CLOUD_PROJECT") {
            config.insert("gcp.project_id".to_string(), Value::String(project_id));
        }
        
        // Load GCP credentials file path
        if let Ok(cred_file) = env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            config.insert("gcp.credentials_file".to_string(), Value::String(cred_file));
        }
        
        // Load GCP config map from environment variable
        if let Ok(config_map_str) = env::var("GCP_CONFIG_MAP") {
            if let Ok(map) = parse_key_value_map(&config_map_str) {
                for (key, value) in map {
                    config.insert(key, value);
                }
            }
        }
        
        Ok(serde_json::Value::Object(config.into_iter().collect()))
    }
}

/// Parse a key-value map string into a HashMap
fn parse_key_value_map(map_str: &str) -> Result<HashMap<String, Value>, ConfigError> {
    let mut map = HashMap::new();
    
    // Handle empty string
    if map_str.trim().is_empty() {
        return Ok(map);
    }
    
    // Parse key-value pairs
    for pair in map_str.split(',') {
        let trimmed_pair = pair.trim();
        if trimmed_pair.is_empty() {
            continue;
        }
        
        if let Some((key, value)) = trimmed_pair.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim();
            
            // Try to parse as JSON for better type handling
            if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                map.insert(key, json_value);
            } else {
                // If not valid JSON, treat as string
                map.insert(key, Value::String(value.to_string()));
            }
        }
    }
    
    Ok(map)
}