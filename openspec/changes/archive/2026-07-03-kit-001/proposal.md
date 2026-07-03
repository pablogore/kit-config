# Proposal: KIT-001 — kit-config Application Configuration Model

## Context

kit-config has evolved from a collection of configuration loaders into the canonical configuration framework for the ecosystem.

Before ego.rs can adopt it as its standard configuration system, the architecture of kit-config itself must be frozen.

The goal of this proposal is **not** to redesign loaders, parsers or providers.

The goal is to define the architectural model of configuration.

---

# Problem Statement

There is currently ambiguity around the responsibilities of:

- Host
- Application
- kit-config
- Libraries

The audit also found several architectural seams that are only partially realized:

- ConfigModule
- Extension
- ConfigurationProfile

Before expanding kit-config further, these responsibilities must be frozen.

---

# Vision

kit-config is **an application configuration framework**.

It does **not** configure libraries.

Libraries expose typed configuration domains.

Applications compose those domains into a root configuration model.

kit-config materializes that root model from one or more configuration sources.

---

# Core Architecture

```
                Host
        (CLI / HTTP / Worker / Tests)
                    │
                    ▼
             Configuration Sources

        TOML
        YAML
        JSON
        Environment
        CLI
        Secret Providers
        Cloud Providers

                    │
                    ▼

               kit-config

                    │
                    ▼

      Deserialize<RootConfiguration>()

                    │
                    ▼

           Root Configuration
        (AppConfig or equivalent)

                    │
         ┌──────────┼───────────┐
         │          │           │
         ▼          ▼           ▼

     JwtConfig  DatabaseConfig  LoggingConfig

         │
         ▼

     Application-specific
     configuration trees
```

---

# Architectural Principles

## 1.

Libraries own configuration **domains**.

Examples:

- JwtConfig
- DatabaseConfig
- LoggingConfig
- GrpcServerConfig

A library never owns application configuration.

---

## 2.

Applications own the **root configuration model**.

The root type may be:

- AppConfig
- GatewayConfig
- BillingConfiguration
- AtlasConfiguration

The name is irrelevant.

The application chooses the root.

---

## 3.

kit-config knows nothing about individual domains.

Its only responsibility is:

Given a root configuration type,

materialize it from configured sources.

---

## 4.

The Host owns configuration loading.

The Host decides:

- sources
- precedence
- profiles
- secret providers
- cloud providers

The Host invokes kit-config exactly once.

---

## 5.

Libraries never know:

- TOML
- YAML
- JSON
- Environment Variables
- CLI
- Vault
- AWS
- Azure
- GCP

Libraries expose only typed configuration.

---

## 6.

Secrets are infrastructure.

Libraries receive resolved values.

Never providers.

Never references.

---

# Resolved Architectural Decisions

## ConfigModule

**Decision: ConfigModule is an optional capability contract for reusable configuration domains that participate in ecosystem tooling (documentation, schema generation, introspection, identity, defaults).**

Being a configuration domain and having introspection/tooling capability are distinct concepts. A publicly exposed configuration type intended to be composed into an application's root configuration model is considered a configuration domain — that requires nothing beyond `serde::Deserialize`. An internal DTO with `#[derive(Deserialize)]` that is never composed into a root model is not a domain. A library implements ConfigModule ONLY when it wants ecosystem tooling capabilities for its domain.

When implemented, the trait contract (not implementation) guarantees:

1. **Identity** — the domain has a stable, machine-readable name (e.g. `logging`, `database`, `jwt`), usable by docs generation, schema generation, tooling, introspection, editor support, and UI generation.
2. **Defaults** — the domain can produce its canonical default value with no external source (`defaults() -> Self`, already present today).

ConfigModule does NOT cover validation. Domain invariants belong exclusively to the separate `Validation` contract (see the Validation decision below) — a module implements `ConfigModule`, `Validation`, or both, independently, depending on whether it needs tooling capabilities, has semantic invariants to check, or both. This keeps "is this a domain with tooling capability" and "does this have invariants to validate" as orthogonal questions.

What ConfigModule explicitly is NOT:

- **Not a loading mechanism.** It never touches sources, formats, environment variables, secret providers, or cloud providers (Principles 4–6).
- **Not required for materialization.** kit-config deserializes the root model via serde and remains ignorant of individual domains (Principle 3). ConfigModule is orthogonal to loading — it represents the domain, it does not load it.
- **Not a requirement for being a domain.** Forcing a trivial type like `pub struct RetryConfig { pub max_retries: u32 }` to implement ConfigModule would turn an opt-in capability into a universal ecosystem-wide obligation — a violation of interface segregation.
- **Not a validation mechanism.** A module needing no semantic validation (e.g. `RetryConfig`) can implement ConfigModule alone, with no obligation to also implement Validation.

Scope: which official kit-config-provided modules (Logging, Database, Http, Postgres, Redis, gRPC, ...) choose to implement ConfigModule is deferred to Spec. Likely all of them will, since they are the reference implementations — but that is a Spec-level choice, not a universal mandate. Where implemented, the trait contract is identity (`NAME`) + `defaults()`, per the Design.

---

## Extension

**Decision: Extension is removed entirely.**

Rationale:

- It has zero implementations anywhere in the workspace — a dead public hexagonal port that carries API-stability cost with no consumer.
- Its contract (`register_source`, `register_validator`) contradicts the frozen principles. The Host owns configuration loading, sources, and precedence (Principle 4). A plugin that injects sources creates a second composition authority competing with the Host, and blurs precedence and secret-handling boundaries (Principles 5–6).
- Extensibility is already served without it: third-party crates implement `ConfigurationSource` or `Validation` directly, and the Host composes them explicitly. That is the plugin story — with the Host in charge, no registry indirection.
- If auto-registration of providers ever becomes a real need, it must arrive as its own proposal driven by real consumers, not remain as a speculative port.

---

## Publishing Configuration Domains

**Decision: the minimum bar for a published, composable configuration domain is `serde::Deserialize` alone. ConfigModule is an enhancement, never a requirement.**

A library publishes a configuration domain by shipping:

1. a public, documented struct implementing `serde::Deserialize` — this is what lets applications embed it in their root model and lets kit-config materialize it. This alone makes the type a published, composable configuration domain.
2. an implementation of `ConfigModule` when it needs ecosystem tooling capabilities (identity, defaults; docs, schema, introspection) — optional, never mandatory. Invariants are a separate concern, covered exclusively by `Validation`.

This is mutually consistent with the ConfigModule decision: a domain's identity comes from `Deserialize` plus composition into the root model; ConfigModule is the opt-in capability layer on top. Libraries still never learn about loaders, sources, or secrets — both contracts are loader-agnostic by construction.

---

## Validation

**Decision: validation ownership is split across the three layers.**

- **Host** — structural validation: deserialization correctness, the shape of the data — did it parse into the expected types.
- **Library** — local invariants within a single configuration domain (e.g. a JwtConfig validating that its own expiry duration is positive).
- **Application** — cross-module / cross-domain rules: rules that span more than one configuration domain, since only the application composes the root model and can see across domains.

---

# Remaining Questions (deferred to Spec / Design)

## ConfigurationProfile

What role does ConfigurationProfile play?

Examples:

- Development
- Test
- Production

Should it remain?

Should it move?

Should it disappear?

---

## Feature Model

Define the intended feature strategy.

Granularity should allow:

- logging only
- database only
- cloud providers independently
- minimal builds

---

# Non Goals

This proposal does NOT redesign:

- ConfigLoader
- Builder
- Parser
- TOML parsing
- YAML parsing
- JSON parsing
- Merge algorithms

unless required by one of the architectural decisions above.

---

# Success Criteria

After this proposal:

- the responsibilities of Host, kit-config, Application and Libraries are explicit
- ConfigModule has a clear architectural role as an optional capability contract for ecosystem tooling
- validation ownership is resolved: Host (structural), libraries (domain invariants), application (cross-domain rules)
- Extension is removed from the public API
- ConfigurationProfile has a clear architectural role (or is removed)
- library crates publish configuration domains with `Deserialize` alone; ConfigModule is an optional enhancement
- applications compose the root configuration model
- kit-config materializes the root model
- secret providers remain infrastructure concerns
- the feature model is coherent from facade down to leaf crates

---

**Status: Frozen**
