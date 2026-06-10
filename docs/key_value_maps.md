# Key-Value Maps

Kit Config provides comprehensive support for key-value map configurations, allowing structured configuration data to be loaded from various sources.

## What are Key-Value Maps?

Key-value maps are structured configuration data where each key maps to a value. These values can be simple types (strings, numbers, booleans) or complex nested structures.

## Usage

Key-value map support is available in all configuration sources through the `with_key_value_map` method:

```rust
use kit_config::{ConfigLoader, sources::EnvironmentSource};

let config = ConfigLoader::builder()
    .add_source(EnvironmentSource::new().with_key_value_map("MY_CONFIG_MAP"))
    .build();
```

## Configuration Format

Key-value maps should be valid JSON strings that represent configuration data. For example:

```json
{
  "database": {
    "host": "localhost",
    "port": 5432,
    "credentials": {
      "username": "admin",
      "password": "secret"
    }
  },
  "api": {
    "timeout": 30,
    "retries": 3
  }
}
```

## Environment Variable Usage

To use key-value maps with environment variables:

1. Set the environment variable with a JSON string:
   ```bash
   export MY_CONFIG_MAP='{"key1": "value1", "key2": 42}'
   ```

2. Load it in your code:
   ```rust
   use kit_config::{ConfigLoader, sources::EnvironmentSource};

   let config = ConfigLoader::builder()
       .add_source(EnvironmentSource::new().with_key_value_map("MY_CONFIG_MAP"))
       .build();
   ```

## Variable Prefix Handling

All sources with key-value map support also support variable prefix handling:

```rust
use kit_config::{ConfigLoader, sources::EnvironmentSource};

let config = ConfigLoader::builder()
    .add_source(EnvironmentSource::new()
        .with_key_value_map("MY_CONFIG_MAP")
        .with_prefix("MYAPP_"))
    .build();
```

## Source Integration

All configuration sources support key-value map integration:

- DotenvSource
- EnvironmentSource  
- DefaultSource
- Cloud provider sources (AWS, GCP, DO)

## Benefits

- **Structured Configuration**: Organize complex configuration data in a hierarchical structure
- **Type Safety**: Leverage serde_json::Value for flexible type handling
- **Environment Integration**: Easily load structured configuration from environment variables
- **Flexible**: Works with any JSON-compatible configuration format

## Best Practices

1. Use descriptive environment variable names for key-value maps
2. Validate configuration structure at application startup
3. Provide sensible defaults for optional configuration values
4. Document the expected JSON structure for your configuration maps