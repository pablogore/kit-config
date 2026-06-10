# Feature Specification: Configuration Framework

**Feature Branch**: `001-config-001-configuration`
**Created**: 2026-06-09
**Status**: Draft
**Input**: User description: "CONFIG-001 Configuration Framework"

## Overview

Create a reusable configuration framework for the entire Kit ecosystem. The
framework becomes the standard configuration solution for:

- kit-observability
- ego-runtime
- workflow-sdk
- persistent-entity-sdk
- service-sdk
- future ecosystem projects

The framework must provide a consistent, typed, validated, extensible, and
production-ready configuration model. It must eliminate duplicated configuration
structures across projects. Applications should compose existing configuration
models and define only domain-specific configuration.

### Goals

- Typed configuration loading
- Multi-source configuration
- Deterministic precedence
- Environment variable support
- Dotenv support
- TOML support
- Configuration validation
- Environment profiles
- Reusable infrastructure configuration modules (model + defaults + validation)
- Reusable observability configuration modules (model + defaults + validation)
- Reusable runtime configuration modules (model + defaults + validation)
- Framework-level validation
- Application-level validation
- Extensible validation pipeline

Applications should only define domain-specific configuration. Common
infrastructure configuration must be reusable.

### Non-Goals

This feature does **not** implement:

- Secret management
- Vault integration (AWS Secrets Manager, Azure Key Vault, GCP Secret Manager)
- Dynamic hot reload
- Remote configuration servers
- Distributed configuration synchronization

These belong to future specifications.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Configuration Loading (Priority: P1)

As an application developer, I want configuration loaded from multiple sources
so that applications can run consistently across local development, testing,
staging, and production environments.

**Why this priority**: Every other capability depends on loading configuration
first. Without multi-source loading there is no configuration to type, validate,
or compose.

**Independent Test**: A test can verify loading from defaults, TOML, dotenv, and
environment variables independently and together, asserting the merged result
matches deterministic precedence rules.

**Acceptance Scenarios**:

1. **Given** a TOML file with connection settings, **When** the loader reads it,
   **Then** the settings are available as a merged configuration map
2. **Given** a `.env` file and an environment variable `DATABASE_URL`,
   **When** both are loaded, **Then** the environment variable overrides the
   dotenv value
3. **Given** no config file at an optional path, **When** the loader runs,
   **Then** it does not error — it continues with other sources

---

### User Story 2 - Typed Configuration (Priority: P2)

As an application developer, I want configuration mapped into typed structures
so that invalid configuration is detected before startup.

**Why this priority**: Raw string maps are error-prone. Typed deserialization
catches type mismatches (wrong URL format, invalid duration) at load time rather
than at runtime.

**Independent Test**: Define a typed struct with nested fields, collections,
and optional values. Load valid and deliberately invalid input, asserting the
typed result or error on mismatch.

**Acceptance Scenarios**:

1. **Given** a config with `server.host` and `server.port`, **When** loaded into
   a typed `ServerConfig` struct, **Then** the values are accessible as native
   Rust types (String, u16)
2. **Given** a duration value `"30s"`, **When** deserialized into
   `DurationConfig`, **Then** the value is available as a `std::time::Duration`
3. **Given** a URL value `"not-a-url"`, **When** deserialized,
   **Then** a typed error is returned at load time

---

### User Story 3 - Validation (Priority: P3)

As an application developer, I want configuration validation so that startup
fails before runtime errors occur.

**Why this priority**: Catching misconfiguration at startup prevents production
incidents and reduces debugging time. Validation is the safety net for the typed
loading in US2.

**Independent Test**: Provide config with an invalid port range (port 0) and a
missing required field. Assert that validation produces descriptive errors and
startup is prevented.

**Acceptance Scenarios**:

1. **Given** a `port: 0` in config, **When** validated, **Then** the error
   states "port must be between 1 and 65535"
2. **Given** a missing required field `database_url`, **When** validated,
   **Then** the error lists the missing field by name
3. **Given** an enum value `"invalid_mode"`, **When** validated against
   allowed variants, **Then** a descriptive "allowed values are..." error is
   returned

---

### User Story 4 - Environment Profiles (Priority: P4)

As a platform engineer, I want profile-based configuration so that deployments
differ only by configuration.

**Why this priority**: Profiles encode environment differences (local vs
production) in configuration, not code. This is a prerequisite for the reusable
models in later stories.

**Independent Test**: Load config with `profile = "staging"` and verify that
staging-specific overrides apply on top of defaults without affecting other
profiles.

**Acceptance Scenarios**:

1. **Given** a `local` profile with debug logging and a `production` profile
   with error-only logging, **When** the active profile is `production`,
   **Then** logger level is `error`
2. **Given** no explicit profile, **When** the loader runs, **Then** the
   default profile (`local`) is used
3. **Given** a profile override via `KIT_PROFILE=staging`, **When** loaded,
   **Then** the staging profile values take precedence over base config

---

### User Story 5 - Prefix-Based Environment Variables (Priority: P5)

As a platform engineer, I want configurable environment variable prefixes so
that multiple applications can coexist on the same host.

**Why this priority**: Without prefix support, every app in the ecosystem needs
unique variable names, making deployment coordination impractical.

**Independent Test**: Set `EGO_LOGGER_LEVEL=debug` with prefix `EGO_` and
assert the loaded config maps to `logger.level = "debug"`.

**Acceptance Scenarios**:

1. **Given** prefix `KIT_` and env var `KIT_SERVER_PORT=8080`, **When** loaded,
   **Then** `server.port` is `8080`
2. **Given** prefix `EGO_` and env var `EGO_LOGGER_LEVEL=debug`, **When**
   loaded, **Then** `logger.level` is `"debug"`
3. **Given** nested keys `EGO_DATABASE_HOST=db.example.com`, **When** loaded
   under prefix `EGO_`, **Then** the key maps to `database.host`

---

### User Story 6 - Extension Model (Priority: P6)

As an application developer, I want to extend configuration with domain-specific
structures so that custom application needs can be supported.

**Why this priority**: The framework must not be a closed system. Without
extensibility, every new use case requires modifying kit-config itself.

**Independent Test**: Define a custom `PaymentConfig` struct outside kit-config,
register it with the framework, load a config that includes payment settings,
and assert the values are accessible.

**Acceptance Scenarios**:

1. **Given** a custom `PaymentConfig` struct, **When** registered via a trait
   or extension mechanism, **Then** the framework loads and validates it
   alongside built-in models
2. **Given** a custom infrastructure config, **When** composed with built-in
   `HttpConfig`, **Then** both are loaded in a single pass
3. **Given** an extension module with custom validation, **When** validation
   runs, **Then** the custom validator executes after framework validation

---

### User Story 7 - Multi-Level Validation (Priority: P7)

As an application developer, I want framework-level and application-level
validation so that infrastructure configuration and business-specific
configuration can be validated independently.

**Why this priority**: Without layered validation, a bug in application config
could mask a framework config error, or vice versa. Separation clarifies
responsibility.

**Independent Test**: Load a config with an invalid framework model (bad port)
AND an invalid application field (negative retry count). Assert both errors
are reported, with framework errors first.

**Acceptance Scenarios**:

1. **Given** an invalid `HttpConfig` port and a valid app config, **When**
   validated, **Then** the framework validation error is reported and startup
   is prevented
2. **Given** a valid framework config and an invalid app field, **When**
   validated, **Then** the application validation error is reported
3. **Given** both framework and application validation errors, **When**
   validated, **Then** all errors are reported with clear source
   identification (framework vs application)

---

### User Story 8 - Infrastructure Configuration Modules (Priority: P8)

As a framework maintainer, I want reusable infrastructure configuration
modules so that all ecosystem projects share identical configuration
definitions.

**Why this priority**: Shared modules eliminate duplication and divergence
across projects. Every project that needs an HTTP server currently redefines
port, host, TLS settings — this stops that pattern.

**Independent Test**: Use `HttpModule` with host, port, and TLS settings.
Load it from TOML and assert the deserialized struct matches expected values.
Then use it directly in a consuming project without redefining any types.

**Acceptance Scenarios**:

1. **Given** a TOML block `[http]\nhost = "0.0.0.0"\nport = 8080`,
   **When** loaded via `HttpModule`, **Then** both fields are correctly parsed
2. **Given** a `PostgresModule`, **When** loaded, **Then** connection string,
   pool size, and SSL mode are available as typed fields
3. **Given** a consumer project that depends on kit-config, **When** the
   consumer uses `kit_config::infra::PostgresModule`, **Then** no redefinition
   is needed — the config struct, defaults, validation, and helpers are ready

---

### User Story 9 - Observability Configuration Modules (Priority: P9)

As a framework maintainer, I want reusable observability configuration
modules so that all ecosystem projects share identical observability
configuration.

**Why this priority**: Observability (logging, metrics, tracing) is
cross-cutting. Every project needs it, and every project benefits from a
consistent, validated configuration module with built-in defaults.

**Independent Test**: Use `LoggerModule` with level, format, and output.
Load from TOML, assert correct parsing. Then verify the same module works in
kit-observability without redefinition.

**Acceptance Scenarios**:

1. **Given** a `[logger]\nlevel = "info"\nformat = "json"` block, **When**
   loaded via `LoggerModule`, **Then** all fields parse correctly
2. **Given** a `MetricsModule` with an OTLP endpoint, **When** validated,
   **Then** the endpoint URL is verified as well-formed
3. **Given** a `TracingModule` with sampling rate, **When** loaded,
   **Then** the rate is parsed as a valid ratio (0.0–1.0)

---

### User Story 10 - Runtime Configuration Modules (Priority: P10)

As a framework maintainer, I want reusable runtime configuration modules so
that operational behavior remains consistent across projects.

**Why this priority**: Runtime patterns (retry, backoff, circuit breaker) are
repeated across every service. Shared modules ensure they behave identically
everywhere.

**Independent Test**: Use `RetryModule` with max_retries, base_delay, and
max_delay. Load from config and assert the computed backoff durations match
the configured strategy.

**Acceptance Scenarios**:

1. **Given** a `[retry]\nmax_retries = 3\nbase_delay = "1s"` block, **When**
   loaded via `RetryModule`, **Then** fields are accessible as typed values
2. **Given** a `CircuitBreakerModule` with threshold and reset timeout,
   **When** validated, **Then** threshold > 0 is enforced
3. **Given** a `WorkerPoolModule` with min and max workers, **When**
   validated, **Then** min <= max is enforced

### Edge Cases

- **Missing config file**: Optional paths must not produce errors
- **Missing optional file**: Silently skipped, not required
- **Invalid TOML**: Parsing error with file/line information
- **Invalid dotenv**: Line-level error with line number
- **Invalid type conversion**: Typed error (e.g., "expected integer, got
  string for `server.port`")
- **Invalid duration**: Error stating the accepted format (e.g., "30s, 5m,
  1h")
- **Invalid size**: Error stating bytes format (e.g., "10MB, 1.5GiB")
- **Invalid URL**: Parse error with URL validation details
- **Invalid socket address**: Error with expected format ("host:port")
- **Invalid enum**: Error listing allowed variants
- **Empty configuration**: Must not panic; returns defaults or empty config
- **Conflicting overrides**: Last source in precedence wins deterministically
- **Prefix collisions**: When two prefixes overlap, the more specific prefix
  wins or an error is produced
- **Unknown fields**: Default behavior is strict — unknown fields cause a
  startup failure. A per-loader or per-module permissive mode can be opted
  into for forward compatibility during schema migrations
- **Multiple profile layers**: Profile-specific config merges on top of base
  config layer by layer
- **Framework validation failure**: Clear diagnostic identifying the failing
  model and field
- **Application validation failure**: Clear diagnostic identifying the
  application-specific rule and field
- **Multiple validation failures**: All failures are collected and reported
  together, not just the first

Applications must fail deterministically when configuration is invalid.

## Requirements _(mandatory)_

### Functional Requirements

#### Configuration Sources

- **FR-001**: System MUST support configuration defaults
- **FR-002**: System MUST support TOML files
- **FR-003**: System MUST support dotenv files
- **FR-004**: System MUST support environment variables
- **FR-005**: System MUST support deterministic source precedence

Default precedence (from lowest to highest):

1. Defaults
2. `config.toml`
3. Profile files
4. `.env`
5. Environment variables

Applications MAY customize precedence.

#### Typed Configuration

- **FR-006**: System MUST support typed configuration loading
- **FR-007**: System MUST support nested configuration structures
- **FR-008**: System MUST support collections (lists, maps)
- **FR-009**: System MUST support optional values
- **FR-010**: System MUST support duration values (e.g., `"30s"`, `"5m"`,
  `"1h"`)
- **FR-011**: System MUST support size values (e.g., `"10MB"`, `"1.5GiB"`)
- **FR-012**: System MUST support URL values
- **FR-013**: System MUST support socket address values (`"host:port"`)

#### Validation

- **FR-014**: System MUST support framework-level validation of reusable
  configuration modules (e.g., `PostgresModule`, `RedisModule`, `KafkaModule`,
  `HttpModule`, `GrpcModule`)
- **FR-015**: Validation failures MUST prevent startup
- **FR-016**: System MUST support field-level validation (required values,
  numeric ranges, string length, allowed values)
- **FR-017**: System MUST support structure-level validation (PostgreSQL
  config, Redis config, Kafka config)
- **FR-018**: System MUST support cross-field validation (TLS enabled
  requires certificates; auth enabled requires credentials; retry enabled
  requires retry configuration)
- **FR-019**: Validation failures MUST provide machine-readable diagnostics
- **FR-020**: Validation failures MUST provide human-readable diagnostics
- **FR-021**: Reusable infrastructure configuration modules MUST provide
  built-in validation
- **FR-022**: Reusable observability configuration modules MUST provide
  built-in validation
- **FR-023**: Reusable runtime configuration modules MUST provide built-in
  validation
- **FR-024**: System MUST support application-level validation (executes
  after framework validation, before domain validation)
- **FR-025**: System MUST support domain-level validation for custom
  configuration extensions (executes last in the pipeline)
- **FR-026**: Applications MUST be able to register custom validators
- **FR-027**: Custom validators MUST execute in the correct pipeline stage
  (application validators after framework, domain validators after application)
- **FR-028**: Custom validators MAY be implemented through traits, functions,
  closures, or equivalent extension mechanisms

#### Environment Variables

- **FR-028**: System MUST support configurable environment variable prefixes
  (e.g., `KIT_`, `EGO_`, `EVENT_ENGINE_`)
- **FR-029**: System MUST support nested environment variable mapping (e.g.,
  `EGO_LOGGER_LEVEL=debug` maps to `logger.level`)

#### Unknown Fields

- **FR-030**: System MUST reject unknown configuration fields by default
  (strict mode), causing startup failure
- **FR-031**: System MUST support a permissive mode, per loader or per module,
  that logs a warning and ignores unknown fields instead of failing

#### Reusable Models

- **FR-032**: System MUST provide reusable infrastructure configuration
  modules
- **FR-033**: System MUST provide reusable observability configuration
  modules
- **FR-034**: System MUST provide reusable runtime configuration modules

#### Extensibility

- **FR-035**: System MUST support custom application configuration extensions
- **FR-036**: System MUST support future configuration source extensions
  (secret providers, remote config) without API redesign

### Non-Functional Requirements

- **NFR-001**: The framework must remain reusable across the entire ecosystem
- **NFR-002**: Configuration loading must be deterministic
- **NFR-003**: Configuration loading must be reproducible
- **NFR-004**: The framework must support future secret-provider integrations
  without API redesign
- **NFR-005**: The framework must support future remote configuration
  providers without API redesign
- **NFR-006**: The framework must remain compatible with containerized and
  cloud-native deployments
- **NFR-007**: Validation must be extensible without modifying framework code

### Testing Requirements

- **TR-001**: Minimum line coverage must be 85%
- **TR-002**: Minimum branch coverage must be 85%
- **TR-003**: All public APIs must have unit tests
- **TR-004**: All validation paths must have unit tests
- **TR-005**: All precedence resolution rules must have unit tests
- **TR-006**: All profile loading scenarios must have unit tests
- **TR-007**: Framework validation must be tested
- **TR-008**: Application validation must be tested
- **TR-009**: Configuration source merging must be tested
- **TR-010**: Failure scenarios must be tested (invalid TOML, invalid dotenv,
  invalid URL, invalid duration, invalid configuration, validation failures)
- **TR-011**: No ignored tests are permitted
- **TR-012**: Tests must be deterministic and runnable in CI

### Key Entities

- **ConfigLoader**: Responsible for loading configuration from multiple sources
  and producing a merged configuration value
- **ConfigurationSource**: Represents a configuration provider (defaults, TOML
  files, dotenv, environment variables). Each source has a priority level and
  a read method
- **ConfigurationProfile**: Represents environment-specific configuration.
  Supported profiles: `local`, `development`, `test`, `staging`, `production`
- **Infrastructure Configuration Modules**: Reusable modules per infrastructure
  domain, each bundling a config struct, built-in defaults, validation logic, and
  helper functions:
  - `HttpModule`, `HttpsModule`, `GrpcModule`
  - `PostgresModule`, `MySqlModule`, `RedisModule`
  - `KafkaModule`, `RedpandaModule`, `NatsModule`, `S3Module`
- **Observability Configuration Modules**: Reusable modules per observability
  domain, each bundling a config struct, defaults, validation, and helpers:
  - `LoggerModule`, `MetricsModule`, `TracingModule`, `OpenTelemetryModule`
- **Runtime Configuration Modules**: Reusable modules per operational pattern,
  each bundling a config struct, defaults, validation, and helpers:
  - `RetryModule`, `BackoffModule`, `CircuitBreakerModule`, `WorkerPoolModule`
- **FrameworkValidator**: Responsible for validating reusable kit-config
  configuration modules. Runs first in the pipeline.
- **ApplicationValidator**: Responsible for validating application-specific
  configuration rules. Runs second, after framework validation completes.
- **DomainValidator**: Responsible for validating domain-specific
  configuration extensions. Runs third, after application validation completes.
- **ValidationReport**: A single data structure returned after all pipeline
  stages execute, containing all collected errors from every stage with source
  identification (framework vs application vs domain).
- **ValidationPipeline**: Ordered pipeline coordinating Framework → Application
  → Domain validation. All stages always execute; all errors are collected into
  a single ValidationReport.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Applications can load configuration from defaults, TOML, dotenv,
  and environment variables
- **SC-002**: Configuration precedence behaves deterministically — re-running
  with the same sources produces identical results
- **SC-003**: Applications can reuse infrastructure configuration modules
  (struct + defaults + validation + helpers) without redefining them
- **SC-004**: Applications can reuse observability configuration modules
  (struct + defaults + validation + helpers) without redefining them
- **SC-005**: Applications can reuse runtime configuration modules
  (struct + defaults + validation + helpers) without redefining them
- **SC-006**: Validation failures prevent startup — a binary with invalid
  config exits with a non-zero code and descriptive error message
- **SC-007**: Applications can extend configuration with domain-specific
  structures
- **SC-008**: The framework can be reused by kit-observability, ego-runtime,
  workflow-sdk, persistent-entity-sdk, and service-sdk without modification
- **SC-009**: Framework validation and application validation execute
  independently and in order
- **SC-010**: Applications can register custom validators without modifying
  kit-config
- **SC-011**: The project maintains at least 85% test coverage (line and
  branch)

## Clarifications

### Session 2026-06-09

- Q: Configuration Module Model → A: Option B — Configuration modules. Each reusable
  infrastructure domain (Postgres, Redis, Kafka, HTTP, etc.) is delivered as a module
  containing the config struct, built-in defaults, validation logic, and helper
  functions (e.g., `PostgresModule` with `PostgresConfig`, defaults, `validate()`).
- Q: Unknown Configuration Fields → A: Option C — Configurable strict/permissive mode,
  strict by default. Unknown fields cause startup failure unless permissive mode is
  explicitly enabled per loader or per module.
- Q: Validation Error Strategy → A: Option B — Collect and report all validation
  errors. All validators run, all failures are accumulated and returned as a single
  diagnostic batch. Both framework-level and application-level errors are reported
  together.
- Q: Validation Pipeline Model → A: Custom — Ordered validation pipeline:
  Framework → Application → Domain. All three stages always execute. All errors are
  collected within each stage. A single `ValidationReport` is returned containing
  all errors from every stage.
- Q: Profile Resolution Model → A: Option A — config.toml, config.local.toml, config.dev.toml, config.prod.toml. Environment profiles are organized as individual files in the root configuration directory with profile-specific suffixes.
- Q: Default Configuration Strategy → A: Required but overridable. Reusable configuration models must provide built-in defaults that are required to be present, but can be overridden by application-specific configuration.

## Assumptions

- The framework becomes the standard configuration solution for the entire Kit
  ecosystem
- Applications should primarily compose existing configuration structures and
  only define domain-specific extensions when necessary
- The implementation may leverage `config-rs`, `serde`, `dotenvy`, `validator`,
  or equivalent ecosystem libraries
- Secret management and remote configuration belong to future specifications
