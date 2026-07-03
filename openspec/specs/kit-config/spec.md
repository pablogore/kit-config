# Delta for kit-config

## REMOVED Requirements

### Requirement: Extension plugin registration

(Reason: Zero implementations workspace-wide. Its `register_source`/`register_validator` contract creates a second composition authority competing with the Host, violating Principle 4. Proposal decision: removed entirely.)
(Migration: None. Third-party crates implement `ConfigurationSource` or `Validation` directly; the Host composes them via `ConfigLoaderBuilder`.)

### Requirement: ConfigurationProfile lifecycle

(Reason: Dead port — mutable `active` state read by no code, never participates in materialization. Principle 3/4: kit-config must stay source/profile-ignorant; profile selection is a Host concern already expressed by choosing which sources to compose.)
(Migration: None. Hosts select profiles by composing `config.{profile}.toml` / env-overlay sources explicitly in their own `ConfigLoaderBuilder` chain.)

## MODIFIED Requirements

### Requirement: ConfigModule capability contract

`ConfigModule` MUST expose `const NAME: &'static str` (stable, machine-readable domain identity) in addition to the existing `fn defaults() -> Self`. `ConfigModule` MUST NOT require `Validation` as a supertrait — the two capabilities are independently adoptable.
(Previously: `ConfigModule` had only `fn defaults() -> Self`, no identity.)

#### Scenario: Domain implements ConfigModule for tooling identity
- GIVEN a domain type `LoggingConfig`
- WHEN it implements `ConfigModule`
- THEN `LoggingConfig::NAME` resolves to a stable string (`"logging"`) usable by docs/schema tooling

#### Scenario: Trivial domain skips ConfigModule
- GIVEN a domain type with no invariants and no tooling need (e.g. `RetryConfig`)
- WHEN it implements only `serde::Deserialize`
- THEN it remains a valid, composable configuration domain without implementing `ConfigModule` or `Validation`

## ADDED Requirements

### Requirement: ConfigError exposes structured validation failures

`ConfigError` MUST gain a `Validation(ValidationReport)` variant. `ValidationReport` MUST remain defined in `config-core::validation` (not a new crate/module) since it is the return type of `Validation::validate` and both live in the base crate all others depend on; splitting it out adds an import boundary with no reuse benefit.

#### Scenario: Validation failure is structured, not stringified
- GIVEN a domain's `validate()` returns `Err(ValidationReport)`
- WHEN the error is propagated as `ConfigError`
- THEN it is `ConfigError::Validation(report)`, preserving `framework_errors`/`application_errors`/`domain_errors` instead of a flattened string

### Requirement: `load_and_validate::<T>()` deserializes then automatically validates

`ConfigLoader::load_and_validate<T>()` MUST require `T: DeserializeOwned + Validation` and MUST call `T::validate()` immediately after successful deserialization, before returning. The Host MUST NOT need a second call to obtain a validated value — this satisfies Principle 4 ("invokes kit-config exactly once").

#### Scenario: Success — structurally valid and passes validation
- GIVEN configured sources produce data matching `AppConfig`'s shape and its invariants hold
- WHEN the Host calls `loader.load_and_validate::<AppConfig>()`
- THEN it returns `Ok(AppConfig)`

#### Scenario: Structural failure — shape mismatch
- GIVEN configured sources produce data that does not deserialize into `AppConfig`
- WHEN the Host calls `loader.load_and_validate::<AppConfig>()`
- THEN it returns `Err(ConfigError::SerializationError(..))` and `AppConfig::validate()` is never invoked

### Scenario: Validation failure — deserializes but invariant fails
- GIVEN data deserializes into `AppConfig` but `AppConfig::validate()` returns `Err(report)`
- WHEN the Host calls `loader.load_and_validate::<AppConfig>()`
- THEN it returns `Err(ConfigError::Validation(report))`

### Requirement: Three-layer validation flow

Validation MUST be layered: (1) Host/structural — serde deserialization producing `ConfigError::SerializationError`; (2) Library/domain — each domain's own `Validation::validate()` filling `domain_errors`; (3) Application/cross-domain — the root model's `Validation::validate()` filling `application_errors`, which MUST delegate to each field's `.validate()` since only the root sees across domains.

#### Scenario: Root validate aggregates per-domain and cross-domain errors
- GIVEN `AppConfig { logging: LoggingConfig, postgres: PostgresModule }` with an invalid `logging.retention.days = 0` and a cross-domain rule violation
- WHEN `AppConfig::validate()` runs
- THEN the returned `ValidationReport` contains the logging error under `domain_errors` and the cross-domain error under `application_errors`

### Requirement: Official kit-config modules adopt ConfigModule/Validation per domain invariants

| Module | ConfigModule (NAME) | Validation | Reason |
|--------|---------------------|------------|--------|
| LoggingConfig | Yes (existing) | Yes (existing) | Reference domain; already tooling-enabled with real invariants |
| PostgresModule | Yes (add NAME, promote inherent `defaults()` to trait) | Yes (existing) | Official domain; real invariants (empty conn string, pool size, ssl mode) |
| RedisModule | Yes (add NAME, promote `defaults()`) | Yes (existing) | Official domain; real invariants (conn string, pool size) |
| HttpModule / HttpsModule | Yes (add NAME, promote `defaults()`) | Yes (existing) | Official domain; real invariants (port, TLS cert/key pairing) |
| GrpcModule / GrpcClientModule | Yes (add NAME, promote `defaults()`) | Yes (existing) | Official domain; real invariants (port, keepalive, TLS pairing) |

#### Scenario: Promoted module gains identity without behavior change
- GIVEN `PostgresModule` currently has an inherent `fn defaults()` but no `ConfigModule` impl
- WHEN it implements `ConfigModule` with `NAME = "postgres"` and its `defaults()` becomes the trait method
- THEN existing callers of `PostgresModule::defaults()` and `.validate()` continue to compile and behave identically

### Requirement: Cargo feature graph enables minimal builds

Features MUST allow: logging-only, database-only (postgres+redis), independent cloud providers, and a minimal core-only build.

| Crate | Features | Default |
|-------|----------|---------|
| config-core | none (always compiled) | n/a |
| config-models | `logging`, `postgres`, `redis`, `http`, `grpc`, `database=["postgres","redis"]` | none |
| config-loaders | `aws`, `gcp`, `digitalocean`, `cloud=["aws","gcp","digitalocean"]` | none |
| kit-config | forwards each leaf feature 1:1; `config-models`=umbrella of all domains | `["config-loaders","logging"]` |

#### Scenario: Logging-only minimal build
- GIVEN `kit-config = { features = ["logging"], default-features = false }`
- WHEN the workspace builds
- THEN only `config-core` + `config-models/logging` compile; postgres/redis/http/grpc/cloud code is absent

#### Scenario: Database-only build excludes http/grpc/cloud
- GIVEN `kit-config = { features = ["database"], default-features = false }`
- WHEN the workspace builds
- THEN `config-models/postgres` and `config-models/redis` compile; `http`, `grpc`, and all `config-loaders` cloud providers are absent
