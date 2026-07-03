use std::collections::HashMap;

use config_core::ConfigurationSource;
use config_core::ConfigError;
use config_core::Validation;
use serde_json::Value;

pub struct ConfigLoaderBuilder {
    sources: Vec<Box<dyn ConfigurationSource>>,
}

impl ConfigLoaderBuilder {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_defaults(mut self) -> Self {
        self.sources.push(Box::new(crate::sources::DefaultsSource));
        self
    }

    pub fn add_toml(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::TomlFileSource::new(file_path, false)));
        self
    }

    pub fn add_yaml(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::YamlFileSource::new(file_path, false)));
        self
    }

    pub fn add_json(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::JsonFileSource::new(file_path, false)));
        self
    }

    pub fn add_dotenv(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::DotenvSource::new(file_path, false)));
        self
    }

    pub fn add_environment(mut self) -> Self {
        self.sources.push(Box::new(crate::sources::EnvironmentSource::new(None::<String>)));
        self
    }

    pub fn add_aws(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::AwsSource::new(optional)));
        self
    }

    pub fn add_digitalocean(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::DigitalOceanSource::new(optional)));
        self
    }

    pub fn add_gcp(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::GcpSource::new(optional)));
        self
    }

    pub fn add_key_value_map(mut self, prefix: &str, map_str: &str, optional: bool) -> Self {
        self.sources.push(Box::new(crate::sources::key_value_map::KeyValueMapSource::new(prefix, map_str, optional)));
        self
    }

    pub fn add_source(mut self, source: Box<dyn ConfigurationSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn build(self) -> Result<ConfigLoader, ConfigError> {
        Ok(ConfigLoader {
            sources: self.sources,
        })
    }
}

pub struct ConfigLoader {
    sources: Vec<Box<dyn ConfigurationSource>>,
}

impl ConfigLoader {
    pub fn builder() -> ConfigLoaderBuilder {
        ConfigLoaderBuilder::new()
    }

    pub fn load(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut merged_config = HashMap::new();

        let mut sources_to_sort = self.sources.iter().collect::<Vec<_>>();
        sources_to_sort.sort_by_key(|source| source.priority());

        for source in sources_to_sort {
            match source.load() {
                Ok(source_config) => {
                    if let Value::Object(map) = source_config {
                        for (key, value) in map {
                            merged_config.insert(key, value);
                        }
                    }
                }
                Err(e) => {
                    if !source.is_optional() {
                        return Err(ConfigError::SourceError(format!(
                            "Failed to load from source {}: {}",
                            source.priority(),
                            e
                        )));
                    }
                }
            }
        }

        Ok(merged_config)
    }

    pub fn load_and_validate<T>(&self) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Validation,
    {
        let config_map = self.load()?;

        let config_json = serde_json::to_string(&config_map)
            .map_err(|e| ConfigError::SerializationError(format!("Failed to serialize config: {}", e)))?;

        let config: T = serde_json::from_str(&config_json)
            .map_err(|e| ConfigError::SerializationError(format!("Failed to deserialize config: {}", e)))?;

        config.validate().map_err(ConfigError::Validation)?;

        Ok(config)
    }
}

impl Default for ConfigLoaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
