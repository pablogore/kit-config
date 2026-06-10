# Research Findings: Configuration Framework

## Decision: Configuration Loading Architecture
**Rationale**: The `config-rs` crate provides a solid foundation for multi-source configuration loading with support for TOML, environment variables, and custom sources. However, it lacks some features needed for our use case (profile management, strict unknown field handling, layered validation). We'll build a wrapper around config-rs that extends it with our required functionality while maintaining compatibility with its core concepts.

**Alternatives considered**: 
- Building from scratch with serde + custom loaders (too much complexity, reinventing proven patterns)
- Using figment crate (good but less mature ecosystem, fewer examples)
- Using envy + custom TOML parsing (fragmented approach, harder to maintain consistency)

## Decision: Validation Strategy
**Rationale**: The `validator` crate provides attribute-based validation that integrates well with serde and Rust types. Combined with custom validation traits, this allows us to implement the layered validation pipeline (framework → application → domain) while maintaining type safety and clear error reporting.

**Alternatives considered**:
- Custom validation functions only (more verbose, harder to maintain)
- JSON Schema validation (overkill for Rust types, runtime overhead)
- Compile-time validation with macros (limited flexibility for complex business rules)

## Decision: Error Handling
**Rationale**: Using `anyhow` for internal error handling and `thiserror` for public error types provides the best balance of ergonomics and user experience. This allows internal code to use `?` operator freely while exposing well-defined, structured error types to consumers.

**Alternatives considered**:
- Only `thiserror` (more verbose internal code)
- Only `anyhow` (loses structured error information for consumers)
- Custom error enum (reinventing proven patterns)

## Decision: Profile Management
**Rationale**: Implementing profile management as file-based overlays (config.local.toml, config.dev.toml, etc.) in the root directory aligns with user preference and provides simple, predictable behavior. Profiles are loaded as additional sources with higher precedence than base config.

**Alternatives considered**:
- Directory-based profiles (profiles/dev.toml) - rejected per user clarification
- Single file with profile sections - harder to manage large configurations
- Environment variable-only profiles - insufficient for complex scenarios

## Decision: Module Architecture
**Rationale**: Each reusable configuration module (HttpModule, PostgresModule, etc.) will be implemented as a Rust struct with associated validation methods and helper functions. This provides type safety, clear APIs, and easy composition while supporting the "required but overridable" defaults strategy.

**Alternatives considered**:
- Trait-based modules (more complex generics, harder to use)
- Macro-generated modules (less transparent, harder to debug)
- Separate crates per module (too fragmented for ecosystem reuse)

## Decision: Testing Strategy
**Rationale**: Following Rust best practices with comprehensive unit tests (85%+ coverage), integration tests for multi-source scenarios, and contract tests for public APIs ensures quality and maintainability. Using cargo's built-in test framework with standard conventions maximizes compatibility with CI/CD tooling.

**Alternatives considered**:
- External testing frameworks (unnecessary complexity for Rust ecosystem)
- Property-based testing only (insufficient for business logic validation)
- Manual testing (violates constitution's test-first principle)