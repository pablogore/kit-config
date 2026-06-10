# Macros and Builder Pattern

Kit Config uses a fluent builder pattern to construct configuration loaders, which provides a clean and intuitive API for combining multiple configuration sources. While the library doesn't currently use Rust macros for its core functionality, the builder pattern itself provides macro-like benefits in terms of API ergonomics.

## Builder Pattern Usage

The primary way to construct configuration loaders is through the `ConfigLoaderBuilder` which provides a fluent API:

```rust
use kit_config::loader::ConfigLoader;

let config = ConfigLoader::builder()
    .add_defaults()
    .add_dotenv(".env")
    .add_environment()
    .build()
    .unwrap();
```

## Available Builder Methods

### Core Configuration Sources

- `add_defaults()` - Adds default configuration values
- `add_toml(path)` - Adds TOML file configuration source
- `add_dotenv(path)` - Adds dotenv file configuration source
- `add_environment()` - Adds environment variable configuration source

### Cloud Provider Sources

- `add_aws(optional)` - Adds AWS configuration source
- `add_digitalocean(optional)` - Adds DigitalOcean configuration source
- `add_gcp(optional)` - Adds GCP configuration source

### Key-Value Map Sources

- `add_key_value_map(prefix, map_str, optional)` - Adds key-value map configuration source

### Custom Sources

- `add_source(source)` - Adds a custom configuration source

## Builder Method Chaining

All builder methods return `Self`, enabling method chaining:

```rust
let config = ConfigLoader::builder()
    .add_defaults()
    .add_toml("config.toml")
    .add_dotenv(".env")
    .add_environment()
    .add_aws(true)
    .add_gcp(true)
    .build()
    .unwrap();
```

## Priority System

The builder methods follow a priority system where sources are loaded in order of increasing priority numbers:
1. Defaults (priority 0) - Highest priority
2. Environment variables (priority 50)
3. Dotenv files (priority 100)
4. TOML files (priority 200)
5. Cloud providers (priority 300+)

Higher priority sources override values from lower priority sources.

## Optional Sources

All sources can be marked as optional, meaning they won't cause loading to fail if they're not available:

```rust
let config = ConfigLoader::builder()
    .add_defaults()
    .add_toml("config.toml")  // This will fail if file doesn't exist
    .add_dotenv(".env")       // This will fail if file doesn't exist
    .add_environment()        // This will always succeed
    .build()
    .unwrap();
```

## Custom Source Integration

You can add custom sources to the builder:

```rust
use kit_config::loader::ConfigLoader;
use kit_config::sources::ConfigurationSource;

// Assuming you have a custom source implementation
// let custom_source = MyCustomSource::new();
// let config = ConfigLoader::builder()
//     .add_source(Box::new(custom_source))
//     .build()
//     .unwrap();
```

## Benefits of the Builder Pattern

1. **Type Safety**: Compile-time checking of method calls
2. **Flexibility**: Easy to add or remove configuration sources
3. **Readability**: Clear, declarative configuration construction
4. **Extensibility**: Easy to add new source types without changing existing code
5. **Error Handling**: Clear error propagation through the build process

## Future Macro Considerations

While the current implementation doesn't use macros, the builder pattern could potentially be enhanced with macros in the future for:
- Reducing boilerplate when defining common configuration patterns
- Creating shortcuts for frequently used source combinations
- Generating source-specific methods automatically

However, the current fluent API provides a good balance of expressiveness and simplicity without the complexity that macros might introduce.