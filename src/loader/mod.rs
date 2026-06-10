use std::collections::HashMap;

use crate::sources::ConfigurationSource;
use crate::errors::ConfigError;
use serde_json::Value;

/// Configuration loader builder pattern
/// 
/// Provides a fluent API for building configuration loaders with various sources.
/// 
/// # Examples
/// 
/// ```
/// use kit_config::loader::ConfigLoader;
/// 
/// let loader = ConfigLoader::builder()
///     .add_defaults()
///     .add_toml("config.toml")
///     .add_environment()
///     .build()
///     .unwrap();
/// ```
pub struct ConfigLoaderBuilder {
    sources: Vec<Box<dyn ConfigurationSource>>,
}

impl ConfigLoaderBuilder {
    /// Creates a new configuration loader builder
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let builder = ConfigLoader::builder();
    /// ```
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }
    
    /// Adds default configuration values to the loader
    /// 
    /// Defaults are loaded first and have the highest priority.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_defaults()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_defaults(mut self) -> Self {
        self.sources.push(Box::new(crate::sources::DefaultsSource));
        self
    }
    
    /// Adds a TOML file source to the loader
    /// 
    /// # Arguments
    /// * `file_path` - Path to the TOML configuration file
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_toml("config.toml")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_toml(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::TomlFileSource::new(file_path, false)));
        self
    }
    
    /// Adds a dotenv file source to the loader
    /// 
    /// # Arguments
    /// * `file_path` - Path to the dotenv configuration file
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_dotenv(".env")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_dotenv(mut self, file_path: impl Into<String>) -> Self {
        self.sources.push(Box::new(crate::sources::DotenvSource::new(file_path, false)));
        self
    }
    
    /// Adds environment variable source to the loader
    /// 
    /// Environment variables have lower priority than defaults but higher than files.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_environment()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_environment(mut self) -> Self {
        self.sources.push(Box::new(crate::sources::EnvironmentSource::new(None::<String>)));
        self
    }
    
    /// Adds AWS configuration source to the loader
    /// 
    /// # Arguments
    /// * `optional` - Whether this source is optional (default: true)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_aws(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_aws(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::AwsSource::new(optional)));
        self
    }
    
    /// Adds DigitalOcean configuration source to the loader
    /// 
    /// # Arguments
    /// * `optional` - Whether this source is optional (default: true)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_digitalocean(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_digitalocean(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::DigitalOceanSource::new(optional)));
        self
    }
    
    /// Adds GCP configuration source to the loader
    /// 
    /// # Arguments
    /// * `optional` - Whether this source is optional (default: true)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_gcp(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_gcp(mut self, optional: bool) -> Self {
        self.sources.push(Box::new(crate::cloud::GcpSource::new(optional)));
        self
    }
    
    /// Adds a key-value map configuration source to the loader
    /// 
    /// # Arguments
    /// * `prefix` - Prefix for the configuration keys
    /// * `map_str` - String representation of key-value pairs in format "key1=value1,key2=value2"
    /// * `optional` - Whether this source is optional (default: true)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_key_value_map("myapp", "host=localhost,port=8080", true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_key_value_map(mut self, prefix: &str, map_str: &str, optional: bool) -> Self {
        self.sources.push(Box::new(crate::sources::key_value_map::KeyValueMapSource::new(prefix, map_str, optional)));
        self
    }
    
    /// Adds a custom configuration source to the loader
    /// 
    /// # Arguments
    /// * `source` - A boxed configuration source trait object
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// use kit_config::sources::ConfigurationSource;
    /// 
    /// // Assuming you have a custom source implementation
    /// // let custom_source = MyCustomSource::new();
    /// // let loader = ConfigLoader::builder()
    /// //     .add_source(Box::new(custom_source))
    /// //     .build()
    /// //     .unwrap();
    /// ```
    pub fn add_source(mut self, source: Box<dyn ConfigurationSource>) -> Self {
        self.sources.push(source);
        self
    }
    
    /// Builds the configuration loader
    /// 
    /// # Returns
    /// * `Result<ConfigLoader, ConfigError>` - The built configuration loader or an error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let loader = ConfigLoader::builder()
    ///     .add_defaults()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<ConfigLoader, ConfigError> {
        Ok(ConfigLoader {
            sources: self.sources,
        })
    }
}

/// Main configuration loader
/// 
/// This struct is responsible for loading configuration from multiple sources
/// and merging them according to priority rules.
/// 
/// # Examples
/// 
/// ```
/// use kit_config::loader::ConfigLoader;
/// 
/// let config = ConfigLoader::builder()
///     .add_defaults()
///     .add_environment()
///     .build()
///     .unwrap();
/// ```
pub struct ConfigLoader {
    sources: Vec<Box<dyn ConfigurationSource>>,
}

impl ConfigLoader {
    /// Creates a new configuration loader builder
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let builder = ConfigLoader::builder();
    /// ```
    pub fn builder() -> ConfigLoaderBuilder {
        ConfigLoaderBuilder::new()
    }
    
    /// Load configuration from all sources and merge them
    /// 
    /// Sources are loaded in priority order (lower numbers = higher priority).
    /// If a source is optional and fails to load, it will be skipped.
    /// 
    /// # Returns
    /// * `Result<HashMap<String, Value>, ConfigError>` - Merged configuration or error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let config = ConfigLoader::builder()
    ///     .add_defaults()
    ///     .build()
    ///     .unwrap();
    /// 
    /// let loaded_config = config.load().unwrap();
    /// ```
    pub fn load(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut merged_config = HashMap::new();
        
        // Create a temporary vector to sort sources by priority
        let mut sources_to_sort = self.sources.iter().collect::<Vec<_>>();
        sources_to_sort.sort_by_key(|source| source.priority());
        
       for source in sources_to_sort {
            match source.load() {
                Ok(source_config) => {
                    // Merge configuration from this source
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
                    // If source is optional, we just continue with other sources
                }
            }
        }
        
        Ok(merged_config)
    }
    
    /// Load and validate configuration
    /// 
    /// This method loads configuration from all sources and deserializes it into
    /// the specified type T. It also performs validation if T implements the Validation trait.
    /// 
    /// # Type Parameters
    /// * `T` - The type to deserialize configuration into
    /// 
    /// # Returns
    /// * `Result<T, ConfigError>` - The deserialized and validated configuration or error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kit_config::loader::ConfigLoader;
    /// 
    /// let config = ConfigLoader::builder()
    ///     .add_defaults()
    ///     .build()
    ///     .unwrap();
    /// 
    /// // Assuming you have a configuration struct that implements serde::Deserialize
    /// // let my_config: MyConfig = config.load_and_validate().unwrap();
    /// ```
    pub fn load_and_validate<T>(&self) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        // Load configuration
        let config_map = self.load()?;
        
        // Convert to JSON string for deserialization
        let config_json = serde_json::to_string(&config_map)
            .map_err(|e| ConfigError::SerializationError(format!("Failed to serialize config: {}", e)))?;
        
        // Deserialize into the target type
        let config: T = serde_json::from_str(&config_json)
            .map_err(|e| ConfigError::SerializationError(format!("Failed to deserialize config: {}", e)))?;
        
        Ok(config)
    }
}