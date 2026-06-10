use std::collections::HashMap;
use std::env;
use serde_json::Value;

use config_core::ConfigurationSource;
use config_core::ConfigError;

#[derive(Debug, Clone)]
pub struct AwsSource {
    optional: bool,
}

impl AwsSource {
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

        if let Ok(profile) = env::var("AWS_PROFILE") {
            config.insert("aws.profile".to_string(), Value::String(profile));
        }

        if let Ok(region) = env::var("AWS_REGION") {
            config.insert("aws.region".to_string(), Value::String(region));
        }

        if let Ok(access_key_id) = env::var("AWS_ACCESS_KEY_ID") {
            config.insert("aws.access_key_id".to_string(), Value::String(access_key_id));
        }

        if let Ok(secret_access_key) = env::var("AWS_SECRET_ACCESS_KEY") {
            config.insert("aws.secret_access_key".to_string(), Value::String(secret_access_key));
        }

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

#[derive(Debug, Clone)]
pub struct DigitalOceanSource {
    optional: bool,
}

impl DigitalOceanSource {
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

        if let Ok(token) = env::var("DO_TOKEN") {
            config.insert("digitalocean.token".to_string(), Value::String(token));
        }

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

#[derive(Debug, Clone)]
pub struct GcpSource {
    optional: bool,
}

impl GcpSource {
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

        if let Ok(project_id) = env::var("GOOGLE_CLOUD_PROJECT") {
            config.insert("gcp.project_id".to_string(), Value::String(project_id));
        }

        if let Ok(cred_file) = env::var("GOOGLE_APPLICATION_CREDENTIALS") {
            config.insert("gcp.credentials_file".to_string(), Value::String(cred_file));
        }

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

fn parse_key_value_map(map_str: &str) -> Result<HashMap<String, Value>, ConfigError> {
    let mut map = HashMap::new();

    if map_str.trim().is_empty() {
        return Ok(map);
    }

    for pair in map_str.split(',') {
        let trimmed_pair = pair.trim();
        if trimmed_pair.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed_pair.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                map.insert(key, json_value);
            } else {
                map.insert(key, Value::String(value.to_string()));
            }
        }
    }

    Ok(map)
}
