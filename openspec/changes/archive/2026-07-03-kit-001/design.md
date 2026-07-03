# Design: KIT-001 — kit-config Application Configuration Model

## Technical Approach

Freeze the four resolved decisions and close the two deferred questions at the trait/module/feature boundary only. Loaders, parsers, and merge stay untouched (Non Goals). Changes are: extend `ConfigModule` with a `NAME` identity constant (kept orthogonal to `Validation`, no supertrait), formalize the three-layer validation flow over the existing `Validation` trait, remove `Extension` and `ConfigurationProfile` from the kernel, have the facade re-export the existing `ConfigLoader` (no new entrypoint function), and introduce per-domain Cargo features across all four crates.

## Architecture Decisions

### Decision: ConfigurationProfile is removed from config-core
**Choice**: Delete `ConfigurationProfile` (and `profile` module) from config-core; no replacement type in the framework.
**Alternatives**: (a) keep as Host source-selection type; (b) move to application root model.
**Rationale**: Principle 4 gives the Host ownership of profiles, and it already expresses them by choosing which sources to compose (`config.{profile}.toml`, env overlays). Principle 3 makes kit-config domain- and source-ignorant, so the kernel must not model profiles. The current struct carries mutable `active` state no code reads and never participates in materialization — same dead-port profile as `Extension`. A first-class profile abstraction, if ever needed, belongs to the Host layer as its own proposal.

### Decision: ConfigModule and Validation are orthogonal, independently-adoptable capabilities
**Choice**: Extend `ConfigModule` with only a `NAME` identity constant. It does NOT gain a `Validation` supertrait; `Validation` stays a separate, independent trait (see Contracts).
**Alternatives**: (a) make `ConfigModule: Validation` (recouples tooling with invariants); (b) put a fresh `validate()` on ConfigModule (duplicates `Validation`).
**Rationale**: The Proposal's Resolved Decision keeps these concepts distinct. A trivial domain (e.g. `RetryConfig { retries: u32 }`) may want `ConfigModule` for tooling/introspection without any invariants to check; a domain with real invariants implements `Validation`; a domain needing both implements both. Interface segregation (proposal). `ConfigModule` today is bare `defaults()` (config-core/src/config_module.rs) — only `NAME` is added.

### Decision: three-layer validation over existing Validation trait
**Choice**: Structural = serde deserialize (Host); domain invariants = `Validation::validate` on the domain (Library, fills `domain_errors`); cross-domain = `Validation::validate` on the root model (Application, fills `application_errors`, and delegates to each field's `.validate()` since only it sees across domains).
**Rationale**: kit-config cannot enumerate domains (Principle 3), so per-domain validation is invoked by the Application inside its root `validate`, not by the framework.

### Decision: Host owns loading; facade re-exports ConfigLoader (no new entrypoint function)
**Choice**: The `kit-config` facade re-exports the existing `ConfigLoader` (and its builder) so the Host depends on a single crate. The Host's one call is `loader.load_and_validate::<AppConfig>()`. No new public function is introduced.
**Alternatives**: add `kit_config::materialize` / `materialize_validated` free functions (rejected — `ConfigLoader::load_and_validate` at config-loaders/src/loader.rs:109 already materializes into `T`; a parallel facade API is unjustified duplication).
**Rationale**: Principle 4 ("invokes kit-config exactly once") is satisfied by the direct `load_and_validate` call — that IS the one call. Source/precedence/profile composition is done by the Host assembling the `ConfigLoader` via its existing builder, not by a wrapping function (Non Goals).

### Decision: load_and_validate() owns semantic validation
**Choice**: `load_and_validate::<T>()` requires `T: Validation` and, after a successful deserialize, calls `T::validate()` itself before returning. It does not return a partially-validated `T` for the Host to validate separately.
**Alternatives**: deserialize only and let the Host call `T::validate()` as a second, separate step (rejected).
**Rationale**: the Host performs exactly one call into kit-config (Principle 4). Returning a partially-validated configuration would force the Host into a second mandatory call to get a trustworthy value, silently reintroducing the two-call shape Principle 4 rules out. Structural failures short-circuit before `validate()` runs (`ConfigError::SerializationError`); validation failures surface as `ConfigError::Validation(report)`.

### Decision: per-domain + per-provider Cargo features
See Feature Model below.

## Contracts

```rust
// config-core/src/config_module.rs — orthogonal to Validation (NO supertrait)
pub trait ConfigModule {
    const NAME: &'static str;                 // identity (docs/schema/introspection)
    fn defaults() -> Self where Self: Sized;  // unchanged
}

// config-core/src/validation.rs — separate, independently-adoptable trait
pub trait Validation {
    fn validate(&self) -> Result<(), ValidationReport>;
}
// Library -> report.add_domain_error(..)   Application -> add_application_error(..)
// A module implements ConfigModule for tooling, Validation for invariants, or both.

// kit-config facade re-exports the existing ConfigLoader; the Host calls once:
//   loader.load_and_validate::<AppConfig>()   // deserializes AND validates
```

Add `ConfigError::Validation(ValidationReport)` so reports are not stringified. Open question for Spec: whether `ValidationReport` belongs in config-core or a dedicated validation-domain location deserves a small ADR — deferred to Spec, not decided here.

## Data Flow

    Host: ConfigLoader::builder().add_*()...build()   (sources/precedence/profile-as-source)
             │  (one call)
             ▼
    loader.load_and_validate::<AppConfig>()
             │
     serde deserialize  ──► structural errors (ConfigError::SerializationError)
             │ ok
     AppConfig::validate() ─► cross-domain (application_errors)   [invoked automatically by load_and_validate]
             │                └► field.validate() per domain (domain_errors)
             ▼
        Result<AppConfig, ConfigError>   (ConfigError::Validation(report) on validation failure)

## Feature Model

| Crate | Features | Gates |
|-------|----------|-------|
| config-core | none (base) | traits + errors, always compiled |
| config-models | `logging`, `postgres`, `redis`, `http`, `grpc`, `database=[postgres,redis]` | each `pub mod` in lib.rs / infra/mod.rs |
| config-loaders | `aws`, `gcp`, `digitalocean`, `cloud=[all three]` | provider structs + `add_*` builder methods in cloud.rs/loader.rs |
| kit-config (facade) | `config-core`, `config-loaders`, `logging`, `postgres`, `redis`, `http`, `grpc`, `database`, `cloud`, `aws`, `gcp`, `digitalocean`; `config-models`=umbrella of all domains; `default=["config-loaders","logging"]` | forwards to leaf `crate/feature`; per-domain re-exports replace today's bundled `logging` block |

logging-only: `default-features=false, features=["logging"]`. database-only: `["database"]`. cloud independent: `["aws"]`. minimal: `["config-core"]`.

## File Changes

| File | Action | Description |
|------|--------|-------------|
| config-core/src/config_module.rs | Modify | Add `NAME` constant (no supertrait; stays orthogonal to `Validation`) |
| config-core/src/profile.rs | Delete | Profile removed (Principle 3/4) |
| config-core/src/extension.rs | Delete | Extension removed |
| config-core/src/lib.rs | Modify | Drop `profile`/`extension` exports |
| config-core/src/errors.rs | Modify | Add `Validation(ValidationReport)` variant |
| config-models/{lib.rs,infra/mod.rs} | Modify | `#[cfg(feature=..)]` per domain; add `NAME` to modules opting into ConfigModule |
| config-models/Cargo.toml | Modify | Add domain features |
| config-loaders/{cloud.rs,loader.rs} | Modify | `#[cfg(feature=..)]` per provider + builder method |
| config-loaders/Cargo.toml | Modify | Add provider features |
| kit-config/src/{lib.rs,modules.rs} | Modify | Re-export `ConfigLoader`/builder; per-domain gated re-exports; drop profile/extension |
| kit-config/Cargo.toml | Modify | Per-domain/provider feature graph |

## Testing Strategy

| Layer | What | Approach |
|-------|------|----------|
| Unit | `ConfigModule::NAME`, domain `validate` | per-module tests (extend existing) |
| Integration | `loader.load_and_validate` structural+cross-domain | root model in kit-config/tests |
| Build matrix | each feature builds in isolation | `cargo check --no-default-features --features X` per feature |

## Migration / Rollout

Pre-1.0 (0.1.0): breaking facade feature rename is acceptable. Removing `Extension`/`ConfigurationProfile` has zero internal consumers (audit). No data migration.

## Open Questions

- [ ] Which official modules implement `ConfigModule` (add `NAME`) vs. `Validation`-only — deferred to Spec.
- [ ] Keep back-compat aliases for old `config-models` bundled feature, or hard-break.
- [ ] `ConfigError::Validation(ValidationReport)` is publicly constructible with an `is_valid: true` report, a domain-invalid state. Future improvement: encapsulate via a smart constructor (e.g. `ConfigError::validation(report)` with a `debug_assert!(!report.is_valid)`) or a report type that only exists when errors are present. Not a regression (the enum was already public) and out of scope for KIT-001's slices — `load_and_validate()` never constructs this state in practice.
