# Extending Kit Config

Kit Config is designed to be easily extensible, allowing you to add custom configuration sources tailored to your specific needs.

## Creating Custom Configuration Sources

To create a custom configuration source, implement the `ConfigurationSource` trait:

```rust
use kit_config::{ConfigurationSource, Result};
use std::collections::HashMap;
use serde_json::Value;

struct MyCustomSource {
    // Add any necessary fields
    config_file_path: String,
}

impl MyCustomSource {
    fn new(config_file_path: &str) -> Self {
        Self {
            config_file_path: config_file_path.to_string(),
        }
    }
}

impl ConfigurationSource for MyCustomSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        // Load configuration from your custom source
        // Return a HashMap<String, Value> with your configuration
        let mut config = HashMap::new();
        
        // Example: Load from a custom file format
        // let content = std::fs::read_to_string(&self.config_file_path)?;
        // Parse content and populate config HashMap
        
        Ok(config)
    }
}
```

## Adding Custom Sources to ConfigLoader

Once you've implemented your custom source, you can use it with the ConfigLoader:

```rust
use kit_config::{ConfigLoader, ConfigurationSource};

let config = ConfigLoader::builder()
    .add_source(MyCustomSource::new("/path/to/config"))
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

## Advanced Extension Patterns

### 1. Source with Configuration Options

```rust
struct ConfigurableSource {
    source_type: String,
    options: HashMap<String, String>,
}

impl ConfigurableSource {
    fn new(source_type: &str) -> Self {
        Self {
            source_type: source_type.to_string(),
            options: HashMap::new(),
        }
    }
    
    fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }
}

impl ConfigurationSource for ConfigurableSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        // Use self.source_type and self.options to determine how to load config
        Ok(HashMap::new())
    }
}
```

### 2. Source with Key-Value Map Support

```rust
struct KeyValueSource {
    env_var: Option<String>,
}

impl KeyValueSource {
    fn new() -> Self {
        Self { env_var: None }
    }
    
    fn with_key_value_map(mut self, env_var: &str) -> Self {
        self.env_var = Some(env_var.to_string());
        self
    }
}

impl ConfigurationSource for KeyValueSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        let mut config = HashMap::new();
        
        if let Some(env_var) = &self.env_var {
            if let Ok(value) = std::env::var(env_var) {
                // Parse JSON and add to config
                if let Ok(parsed) = serde_json::from_str::<HashMap<String, Value>>(&value) {
                    config.extend(parsed);
                }
            }
        }
        
        Ok(config)
    }
}
```

### 3. Source with Variable Prefix Support

```rust
struct PrefixedSource {
    prefix: Option<String>,
}

impl PrefixedSource {
    fn new() -> Self {
        Self { prefix: None }
    }
    
    fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }
}

impl ConfigurationSource for PrefixedSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        let mut config = HashMap::new();
        
        // When loading environment variables, use the prefix if provided
        if let Some(prefix) = &self.prefix {
            // Load variables with prefix
        } else {
            // Load variables without prefix
        }
        
        Ok(config)
    }
}
```

## Best Practices for Extensions

1. **Keep Sources Focused**: Each source should have a single, well-defined responsibility
2. **Handle Errors Gracefully**: Return appropriate errors for missing files, invalid formats, etc.
3. **Follow Existing Patterns**: Use the same patterns and conventions as existing sources
4. **Document Your Sources**: Provide clear documentation for how to use your custom sources
5. **Test Thoroughly**: Ensure your custom sources work correctly with the rest of the system

## Example: Custom File Source

Here's a complete example of a custom JSON file source:

```rust
use kit_config::{ConfigurationSource, Result};
use std::collections::HashMap;
use serde_json::Value;
use std::fs;

struct JsonFileSource {
    file_path: String,
}

impl JsonFileSource {
    fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl ConfigurationSource for JsonFileSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        let parsed: HashMap<String, Value> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON config: {}", e))?;
        
        Ok(parsed)
    }
}

// Usage:
// let config = ConfigLoader::builder()
//     .add_source(JsonFileSource::new("/path/to/config.json"))
//     .build();
```

## Integration with Existing Sources

Custom sources integrate seamlessly with existing sources. The order in which you add sources determines their precedence:

```rust
let config = ConfigLoader::builder()
    .add_source(kit_config::sources::DefaultSource::new())
    .add_source(MyCustomSource::new())
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

In this example, default values are loaded first, then custom source values, then environment variables, with each source potentially overriding values from previous sources.

## Variable Prefix Handling in Extensions

When creating custom sources, you can also support variable prefix handling to maintain consistency with the existing API:

```rust
struct MyPrefixedSource {
    prefix: Option<String>,
}

impl MyPrefixedSource {
    fn new() -> Self {
        Self { prefix: None }
    }
    
    fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }
}

impl ConfigurationSource for MyPrefixedSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        let mut config = HashMap::new();
        
        // When using prefixes, you can modify variable names accordingly
        if let Some(prefix) = &self.prefix {
            // Apply prefix to your variable lookups
        }
        
        Ok(config)
    }
}
```