# Default Configuration Values

Kit Config provides a robust way to set default configuration values that can be overridden by other configuration sources. This ensures your application has sensible defaults while still allowing for environment-specific overrides.

## Setting Default Values

Default values are provided through the `DefaultSource` which loads configuration from a predefined set of default values:

```rust
use kit_config::{ConfigLoader, sources::DefaultSource};

let config = ConfigLoader::builder()
    .add_source(DefaultSource::new())
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

## Defining Custom Defaults

You can create custom default values by implementing the `DefaultSource` with your own configuration:

```rust
use kit_config::{ConfigLoader, sources::DefaultSource};
use std::collections::HashMap;
use serde_json::Value;

// Create custom defaults
let mut defaults = HashMap::new();
defaults.insert("database_url".to_string(), Value::String("sqlite://default.db".to_string()));
defaults.insert("port".to_string(), Value::Number(3000.into()));
defaults.insert("debug".to_string(), Value::Bool(true));

let config = ConfigLoader::builder()
    .add_source(DefaultSource::with_defaults(defaults))
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

## Default Values with Key-Value Maps

You can also use key-value maps for default configurations:

```rust
use kit_config::{ConfigLoader, sources::DefaultSource};
use serde_json::Value;

// Set default values through environment variable
// export DEFAULT_CONFIG_MAP='{"database_url": "sqlite://default.db", "port": 3000}'

let config = ConfigLoader::builder()
    .add_source(DefaultSource::new().with_key_value_map("DEFAULT_CONFIG_MAP"))
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

## Default Values with Prefixes

Default sources also support variable prefix handling:

```rust
use kit_config::{ConfigLoader, sources::DefaultSource};

let config = ConfigLoader::builder()
    .add_source(DefaultSource::new().with_prefix("MYAPP_"))
    .add_source(kit_config::sources::EnvironmentSource::new())
    .build();
```

With the prefix "MYAPP_", the default source will look for variables like `MYAPP_DATABASE_URL` instead of just `DATABASE_URL`.

## Complete Example

Here's a complete example showing how to set up defaults with various configuration sources:

```rust
use kit_config::{ConfigLoader, sources::{DefaultSource, EnvironmentSource, DotenvSource}};
use std::collections::HashMap;
use serde_json::Value;

// Define default configuration
let mut defaults = HashMap::new();
defaults.insert("app_name".to_string(), Value::String("MyApp".to_string()));
defaults.insert("port".to_string(), Value::Number(3000.into()));
defaults.insert("debug".to_string(), Value::Bool(false));

let config = ConfigLoader::builder()
    .add_source(DefaultSource::with_defaults(defaults))
    .add_source(DotenvSource::new())
    .add_source(EnvironmentSource::new())
    .build();

// The configuration will use defaults unless overridden by .env or environment variables
let app_name = config.get("app_name").unwrap_or_else(|_| Value::String("DefaultApp".to_string()));
```

## Best Practices for Defaults

1. **Provide Sensible Defaults**: Set defaults that work in development and testing environments
2. **Use Environment-Specific Defaults**: Consider different defaults for different environments
3. **Document Your Defaults**: Make it clear what the default values are in your documentation
4. **Separate Concerns**: Keep default values separate from runtime configuration
5. **Validate Defaults**: Ensure default values are valid for your application's requirements

## Default Source Precedence

The DefaultSource should typically be added first in the configuration loader to ensure that:
- Default values are loaded first
- Environment variables and other sources can override defaults
- Your application has fallback values when no configuration is provided

```rust
let config = ConfigLoader::builder()
    .add_source(DefaultSource::new())  // Load defaults first
    .add_source(DotenvSource::new())   // Override with .env values
    .add_source(EnvironmentSource::new()) // Override with environment variables
    .build();
```

This approach ensures that your application always has a working configuration while still allowing for easy customization in different environments.