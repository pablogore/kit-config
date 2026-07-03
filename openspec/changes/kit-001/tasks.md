# Tasks: KIT-001 — kit-config Application Configuration Model

## Review Workload Forecast

| Field | Value |
|-------|-------|
| Estimated changed lines | ~65 / ~35 / ~180 / ~180 (Slices 1-4) |
| 400-line budget risk | Low (each slice) |
| Chained PRs recommended | Yes |
| Suggested split | PR 1 → PR 2 → PR 3 → PR 4 (stacked to main) |
| Delivery strategy | auto-chain |
| Chain strategy | stacked-to-main |

Decision needed before apply: No
Chained PRs recommended: Yes
Chain strategy: stacked-to-main
400-line budget risk: Low

### Suggested Work Units

| Unit | Goal | Likely PR | Notes |
|------|------|-----------|-------|
| 1 | Remove dead ports (Extension, ConfigurationProfile) | PR 1 | base=main; pure deletion |
| 2 | ConfigModule NAME + ConfigError::Validation | PR 2 | base=main after PR1 |
| 3 | Promote 6 infra modules + auto-validate in load_and_validate | PR 3 | base=main after PR2 |
| 4 | Cargo feature graph + facade re-exports | PR 4 | base=main after PR3 |

## Slice 1: Remove dead ports (config-core)

- [x] 1.1 Delete `config-core/src/extension.rs`; remove `pub mod extension;` and `pub use extension::Extension;` from `config-core/src/lib.rs`.
- [x] 1.2 Delete `config-core/src/profile.rs`; remove `pub mod profile;` and `pub use profile::ConfigurationProfile;` from `config-core/src/lib.rs`.
- [x] 1.3 Remove `Extension`/`ConfigurationProfile`/`profile`/`extension` re-exports from `kit-config/src/lib.rs` and `pub use config_core::extension::Extension;` from `kit-config/src/modules.rs`.
- [x] 1.4 Update `docs/README.md` feature table: drop `ConfigurationProfile` from the `config-core` row.
- [x] 1.5 Verify: `cargo test --workspace`; `cargo check -p kit-config -p config-core`.

## Slice 2: ConfigModule NAME + ConfigError::Validation (config-core)

- [ ] 2.1 RED: in `config-models/src/logging.rs` tests, assert `LoggingConfig::NAME == "logging"`.
- [ ] 2.2 GREEN: add `const NAME: &'static str;` to `ConfigModule` (`config-core/src/config_module.rs`).
- [ ] 2.3 GREEN: add `const NAME: &'static str = "logging";` to `impl ConfigModule for LoggingConfig`.
- [ ] 2.4 RED: in `config-core/src/errors.rs` tests, assert a `ValidationReport` wraps into `ConfigError::Validation(report)`.
- [ ] 2.5 GREEN: add `Validation(ValidationReport)` variant to `ConfigError`, importing `crate::validation::ValidationReport`.
- [ ] 2.6 Verify: `cargo test --workspace`; `cargo check -p config-core -p config-models`.

## Slice 3: Promote infra modules + auto-validate

- [ ] 3.1 RED: add `NAME` assertions in `postgres.rs`, `redis.rs`, `http.rs` (Http+Https), `grpc.rs` (Grpc+GrpcClient): `"postgres"`, `"redis"`, `"http"`, `"https"`, `"grpc"`, `"grpc-client"`.
- [ ] 3.2 GREEN: promote each inherent `defaults()` into `impl ConfigModule for <Module>` with its `NAME`; remove the inherent `impl <Module> { fn defaults() }` blocks.
- [ ] 3.3 Add `use config_core::ConfigModule;` to `kit-config/tests/grpc_test.rs` so `GrpcModule::defaults()`/`GrpcClientModule::defaults()` keep resolving.
- [ ] 3.4 RED: add `kit-config/tests/validation_test.rs` asserting `load_and_validate::<LoggingConfig>()` returns `Err(ConfigError::Validation(_))` on an invalid sampling config.
- [ ] 3.5 RED: assert `load_and_validate::<LoggingConfig>()` returns `Err(ConfigError::SerializationError(_))` on a shape mismatch (validate() never reached).
- [ ] 3.6 GREEN: change `load_and_validate<T>()` bound to `T: DeserializeOwned + Validation`; call `T::validate()` after successful deserialize, mapping `Err(report)` to `ConfigError::Validation(report)`.
- [ ] 3.7 Verify: `cargo test --workspace`.

## Slice 4: Cargo feature graph + facade re-exports

- [ ] 4.1 Add `logging`, `postgres`, `redis`, `http`, `grpc`, `database=["postgres","redis"]` to `config-models/Cargo.toml`; gate each `pub mod` in `lib.rs`/`infra/mod.rs` with `#[cfg(feature = "...")]`.
- [ ] 4.2 Add `aws`, `gcp`, `digitalocean`, `cloud=["aws","gcp","digitalocean"]` to `config-loaders/Cargo.toml`; gate provider structs and `add_*` builder methods in `cloud.rs`/`loader.rs`.
- [ ] 4.3 Rework `kit-config/Cargo.toml`: forward each leaf feature 1:1; keep `config-models` as an umbrella alias (`=["logging","postgres","redis","http","grpc"]`) for back-compat; set `default = ["config-loaders", "logging"]`.
- [ ] 4.4 Gate per-domain re-exports in `kit-config/src/lib.rs`/`modules.rs` behind their features; confirm `ConfigLoader`/builder stay re-exported directly (no new entrypoint function).
- [ ] 4.5 Update `docs/README.md` feature table and examples for the new per-domain flags; note the `default` behavior change (breaking, pre-1.0).
- [ ] 4.6 Verify: `cargo test --workspace`; `cargo check -p kit-config --no-default-features --features logging|database|aws|config-core` (one check per feature).
