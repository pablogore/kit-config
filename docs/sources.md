# Configuration Sources

Kit Config supports multiple configuration sources with a defined precedence order. Sources are loaded in the order they are added to the loader, with later sources taking precedence over earlier ones.

## Available Sources

### 1. Default Source
Provides default configuration values that can be overridden by other sources.

### 2. Environment Source
Loads configuration from environment variables.

### 3. Dotenv Source
Loads configuration from .env files using the dotenv format.

### 4. Key-Value Map Source
Supports structured configuration data in key-value map format.

### 5. Cloud Provider Sources
- AWS Configuration
- GCP Configuration  
- DigitalOcean Configuration

## Source Precedence

The configuration sources are loaded in the order they are added to the ConfigLoader. Later sources will override values from earlier sources.

## Variable Prefix Handling

All sources support variable prefix handling through the `with_prefix` method. This allows you to namespace configuration variables to avoid conflicts:

```rust
use kit_config::{ConfigLoader, sources::EnvironmentSource};

let config = ConfigLoader::builder()
    .add_source(EnvironmentSource::new().with_prefix("MYAPP_"))
    .build();
```

With the prefix "MYAPP_", the source will look for environment variables like `MYAPP_DATABASE_URL` instead of just `DATABASE_URL`.

## Key-Value Map Support

All sources support key-value map configuration through the `with_key_value_map` method, allowing structured configuration data to be loaded from various sources.

## Source Implementation

Each source implements the `ConfigurationSource` trait, which requires a `load()` method that returns a `HashMap<String, serde_json::Value>`.

## Custom Sources

To create a custom source, implement the `ConfigurationSource` trait:

```rust
use kit_config::{ConfigurationSource, Result};
use std::collections::HashMap;
use serde_json::Value;

struct MyCustomSource;

impl ConfigurationSource for MyCustomSource {
    fn load(&self) -> Result<HashMap<String, Value>> {
        // Your implementation here
        Ok(HashMap::new())
    }
}
```