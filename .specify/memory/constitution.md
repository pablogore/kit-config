<!--
  Sync Impact Report — Constitution v1.0.0

  Version change: (initial) → 1.0.0
  Modified principles: all placeholders replaced with concrete values for kit-config
  Added sections: none
  Removed sections: none
  Templates requiring updates:
    - .specify/templates/plan-template.md — ✅ reviewed (Constitution Check section is generic — no change needed)
    - .specify/templates/spec-template.md — ✅ reviewed (no change needed)
    - .specify/templates/tasks-template.md — ✅ reviewed (no change needed)
    - .specify/templates/commands/* — ✅ reviewed (no outdated agent references found)
  Follow-up TODOs: none
-->

# kit-config Constitution

A configuration management toolkit built with Rust — principles for building,
validating, and composing configurations programmatically.

## Core Principles

### I. Rust-First Development

All core functionality MUST be implemented in Rust, leveraging its memory safety
guarantees for configuration parsing, validation, and merging. Dependencies MUST
be minimized and justified — no dependency should duplicate core Rust standard
library capabilities. Third-party crates MUST be evaluated for maintenance
quality, safety record, and semver stability before adoption.

### II. Config-as-Code

Configurations MUST be declarative, version-controlled, and composed
programmatically. Users MUST be able to define configurations as code (Rust
structs, TOML, YAML, or domain-specific DSLs) and compose them via overrides,
environment variable interpolation, and conditional logic. The system MUST NOT
require manual merging of configuration files.

### III. Test-First (NON-NEGOTIABLE)

TDD is mandatory. Tests MUST be written and verified to fail before
implementation begins. The Red-Green-Refactor cycle MUST be strictly enforced.
Every module, parser, validator, and CLI command MUST have unit tests.
Integration tests MUST cover contract boundaries and cross-module interactions.

### IV. Schema-Driven Validation

Every configuration format MUST have an associated schema (e.g., JSON Schema,
CUE, or Rust types serving as schema) that defines allowed values, types,
defaults, and constraints. Validation MUST occur at parse time — invalid
configurations MUST be rejected with clear, actionable error messages before any
system state is modified.

### V. Modularity & Composability

The system MUST be composed of small, focused modules with single
responsibilities. Modules MUST expose clear, minimal public APIs. Cross-module
dependencies MUST be explicit and acyclic. No module may exceed a cognitive
complexity that justifies extraction — use YAGNI to decide when to extract.

## Security & Secrets Management

Configuration files MUST NOT contain secrets (passwords, API keys, certificates)
in plain text. The system MUST support environment variable interpolation as the
primary mechanism for exposing secrets to configs. If secret references via
external backends (e.g., vaults) are supported, they MUST require explicit
opt-in. All file read operations MUST validate permissions and path safety —
directory traversal MUST be rejected.

## Development Workflow & Quality Gates

All feature work MUST follow the /spec workflow: specify → clarify → checklist →
plan → tasks → implement → analyze. Feature branches MUST use the
`###-feature-name` convention. No PR may merge without passing the Constitution
Check gate — all principles MUST be respected. CI MUST run the full test suite,
clippy linting, and a dependency audit (`cargo audit` or equivalent).
Complexity MUST be justified in the plan's Complexity Tracking section;
unjustified complexity is a merge blocker.

## Governance

This constitution supersedes all other process guidance. Amendments require:
(1) documented rationale, (2) approval from the project maintainer, (3) a
migration plan for any existing work that conflicts with the change.

The version follows semver:

- **MAJOR**: incompatible principle removals or redefinitions
- **MINOR**: new principles or materially expanded guidance
- **PATCH**: clarifications, wording, typo fixes, non-semantic refinements

All PRs and reviews MUST verify constitution compliance.

**Version**: 1.0.0 | **Ratified**: 2026-06-09 | **Last Amended**: 2026-06-09
