# kit-config Development Guidelines

Auto-generated from constitution and feature plans. Last updated: 2026-06-09

## Constitution

**Version**: 1.0.0 | **Ratified**: 2026-06-09

The project follows the constitution at `.specify/memory/constitution.md`. Key
principles that guide implementation:

1. **Rust-First Development** — All core in Rust, minimize dependencies
2. **Config-as-Code** — Declarative, version-controlled, composable configs
3. **Test-First (NON-NEGOTIABLE)** — TDD, Red-Green-Refactor enforced
4. **Schema-Driven Validation** — Parse-time validation with clear errors
5. **Modularity & Composability** — Small modules, explicit acyclic deps

## Active Technologies

- **Language**: Rust (edition 2024)
- **Build**: Cargo
- **Testing**: `cargo test`
- **Linting**: Clippy (`cargo clippy`)
- **Audit**: `cargo audit`

## Project Structure

```text
.
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs              # Entry point
├── tests/                   # Integration tests
├── specs/                   # Feature specifications
│   └── ###-feature-name/    # Per-feature directories
│       ├── spec.md
│       ├── plan.md
│       ├── research.md
│       ├── data-model.md
│       ├── contracts/
│       ├── tasks.md
│       └── quickstart.md
└── .specify/
    ├── memory/
    │   ├── constitution.md  # Project governance
    │   └── pi-agent.md      # This file
    └── templates/           # Workflow templates
```

## Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Build the project |
| `cargo test` | Run all tests |
| `cargo clippy` | Lint check |
| `cargo fmt` | Format code |
| `cargo audit` | Security audit |

## Code Style

- Follow standard Rust formatting via `cargo fmt`
- Use `cargo clippy` — no warnings allowed
- Prefer `Result<T, E>` over panics; panics only in unrecoverable states
- Use `thiserror` or `anyhow` for error handling patterns (decide per module)
- Module structure: one module per responsibility, `pub` API minimal

## Recent Changes

- **2026-06-09**: Initial scaffold — Rust workspace, constitution (v1.0.0)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
