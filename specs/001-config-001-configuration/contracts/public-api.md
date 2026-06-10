# Public API Contracts: Configuration Framework

## Core Public Interfaces

### ConfigLoader Builder Pattern
**Contract**: Users must be able to configure the loader using a fluent builder pattern that allows adding sources in a specific order.

```rust
let loader = ConfigLoader::builder()
    .add_defaults()
    .add_toml("config.toml")
    .add_profile_toml()
    .add_dotenv(".env") 
    .add_environment()
    .profile("local")
    .prefix("KIT_")
    .strict_mode(true)
    .build()?;
```

**Guarantees**:
- Sources are added in the exact order specified
- Builder methods are chainable and immutable (each returns new builder)
- `build()` returns `Result<ConfigLoader, Error>` with clear error types
- Default precedence is maintained when no custom order is specified

### Configuration Loading
**Contract**: The `load_and_validate()` method must work with any serde-deserializable type and return either the loaded configuration or a comprehensive validation report.

```rust
let config: MyConfigType = loader.load_and_validate()?;
// OR
let result: Result<MyConfigType, Box<dyn Error>> = loader.load_and_validate();
```

**Guarantees**:
- Type parameter must implement `serde::Deserialize` 
- All validation stages (framework, application, domain) are executed
- All validation errors are collected and reported together
- Invalid configurations never produce partially valid structs
- Error types are downcastable to `ValidationReport` for detailed inspection

### ValidationReport Interface
**Contract**: Validation reports must provide structured access to all validation errors with clear categorization.

```rust
let report = validation_error.downcast_ref::<ValidationReport>().unwrap();
for error in report.framework_errors() { /* handle framework errors */ }
for error in report.application_errors() { /* handle application errors */ }
for error in report.domain_errors() { /* handle domain errors */ }
```

**Guarantees**:
- Errors are categorized by validation source (framework/application/domain)
- Each error contains field path, human-readable message, and machine-readable code
- `is_valid()` method returns `false` when any errors are present  
- `all_errors()` returns flattened list of all errors across all categories

## Reusable Module Contracts

### ConfigModule Trait
**Contract**: All reusable configuration modules must implement the `ConfigModule` trait to provide defaults and validation.

```rust
pub trait ConfigModule: Validation + serde::Deserialize {
    fn defaults() -> Self;
}
```

**Guarantees**:
- `defaults()` returns sensible, production-ready default values
- Default values pass all validation rules
- Modules are composable (can be nested in user-defined structs)
- Modules provide helper methods for common operations

### Validation Trait
**Contract**: All validation-capable types must implement the `Validation` trait.

```rust
pub trait Validation {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
```

**Guarantees**:
- `validate()` returns `Ok(())` for valid configurations
- `validate()` returns `Err(errors)` with all validation failures for invalid configurations  
- Validation is side-effect free (pure function of current state)
- Error messages are actionable and user-friendly

### ApplicationValidator Trait  
**Contract**: Applications can implement custom validation by implementing `ApplicationValidator`.

```rust
pub trait ApplicationValidator: serde::Deserialize {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
```

**Guarantees**:
- Application validation runs after framework validation
- Application validation errors are categorized separately
- Multiple application validators can be composed

## Error Handling Contracts

### ValidationError Structure
**Contract**: All validation errors follow a consistent structure.

```rust
pub struct ValidationError {
    pub field: String,      // Field path (e.g., "http.port", "database.connection.pool_size")
    pub message: String,    // Human-readable error message  
    pub code: Option<String>, // Machine-readable error code (optional)
}
```

**Guarantees**:
- Field paths use dot notation for nested structures
- Messages are grammatically correct and actionable
- Codes follow consistent naming convention when present

### Error Type Hierarchy
**Contract**: Error types follow Rust best practices with `thiserror` and `anyhow`.

```rust
// Public error types implement std::error::Error
pub enum ConfigError {
    Io(std::io::Error),
    Parse(ParseError),  
    Validation(ValidationReport),
    // ...
}

// Internal errors use anyhow for ergonomics
fn internal_function() -> anyhow::Result<()> {
    // ...
}
```

**Guarantees**:
- Public APIs return structured error types
- Internal code can use `?` operator with `anyhow::Result`
- Errors can be downcast to specific types for detailed handling
- Error messages include context and suggestions

## Extension Contracts

### Custom Source Interface
**Contract**: Users can implement custom configuration sources.

```rust
pub trait ConfigurationSource {
    fn load(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    fn priority(&self) -> u8;
    fn is_optional(&self) -> bool;
}
```

**Guarantees**:
- Custom sources integrate seamlessly with built-in sources
- Priority determines merge order (lower = higher precedence)  
- Optional sources don't cause errors when unavailable
- Sources return serde-compatible values

### Custom Validator Registration
**Contract**: Applications can register custom validators for domain-specific logic.

```rust
let loader = ConfigLoader::builder()
    .register_domain_validator(MyCustomValidator)
    // ...
    .build()?;
```

**Guarantees**:
- Domain validators run after application validators
- Multiple domain validators can be registered
- Validators receive the fully merged configuration
- Validator errors are categorized as domain errors

## Testing Utility Contracts

### TestConfigLoader
**Contract**: Test utilities provide easy configuration for unit tests.

```rust
let loader = TestConfigLoader::new()
    .with_toml("config = 'test'")
    .with_env("TEST_VAR", "value")
    .build();
```

**Guarantees**:
- Test utilities don't require actual files
- Environment variables are mocked safely
- Test loaders support all the same features as production loaders
- Test utilities are only available in test builds (`#[cfg(test)]`)

## Versioning and Compatibility

### Semantic Versioning
**Contract**: Public APIs follow semantic versioning.

- **MAJOR**: Breaking changes to public APIs, traits, or contracts
- **MINOR**: New features, modules, or non-breaking API additions  
- **PATCH**: Bug fixes and internal improvements

**Guarantees**:
- Minor versions maintain backward compatibility
- Breaking changes are clearly documented in release notes
- Deprecation warnings are provided before removal
- Migration guides are provided for major versions

### Rust Version Support
**Contract**: The library supports Rust versions according to ecosystem standards.

- Minimum Rust version: 1.75 (as specified in Technical Context)
- Support for latest 3 stable Rust versions
- CI testing against minimum and latest stable versions

**Guarantees**:
- Library compiles and passes tests on supported Rust versions
- Dependencies are compatible with minimum Rust version
- No use of unstable Rust features in stable releases