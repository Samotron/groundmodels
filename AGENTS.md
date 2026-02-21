# AGENTS.md - Groundmodels Codebase Guide

This file is for agentic coding assistants working in this repo.
Keep instructions concise, explicit, and aligned with the actual codebase.

## Workspace Overview
- Rust workspace with 3 crates: core library, CLI, Python bindings
- Entry points: `groundmodels-core/src/lib.rs`, `groundmodels-cli/src/main.rs`
- Python bindings via PyO3/maturin in `groundmodels-py`
- AGSi schema types are generated in `groundmodels-core/src/agsi.rs`

## Build / Test / Lint Commands
General workspace commands (run from repo root):
- Build all crates: `cargo build`
- Test all crates: `cargo test`
- Lint (clippy): `cargo clippy`
- Format: `cargo fmt`

Targeted Rust commands:
- Build core: `cargo build -p groundmodels-core`
- Test core: `cargo test -p groundmodels-core`
- Build CLI: `cargo build -p groundmodels-cli`
- Run CLI: `cargo run -p groundmodels-cli -- <command>`

Single-test execution (preferred patterns):
- Workspace test by name: `cargo test test_ground_model_creation -- --exact`
- Crate + exact test: `cargo test -p groundmodels-core test_ground_model_creation -- --exact`

Python bindings:
- Build/dev install: `cd groundmodels-py && maturin develop`

LSP server (CLI subcommand):
- `cargo run -p groundmodels-cli -- lsp --port 3000`

## Code Style Guidelines (Rust)
Formatting:
- Use `cargo fmt` for all Rust code changes
- Keep line lengths reasonable; prefer clarity over dense formatting

Imports and module layout:
- Order imports: standard library, external crates, local modules
- Use `mod` declarations at the top of files
- Prefer explicit imports over globbing

Naming:
- Types/enums: `PascalCase`
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

Types and data modeling:
- Prefer explicit types in public function signatures
- Use `Option<T>` for nullable values
- Use `Result<T, E>` for fallible operations
- Use `#[derive(Serialize, Deserialize)]` for data structs when serialized
- Serde field names should be `snake_case` unless external schema requires otherwise

Error handling:
- Prefer `?` for error propagation
- Use descriptive error messages; avoid `unwrap()` in library code
- Return structured errors when possible; `&'static str` is acceptable for small helpers

Documentation:
- Public APIs should have `///` doc comments
- Include examples for complex functions or data conversions

Testing:
- Tests live in `#[cfg(test)]` modules near the code they verify
- Use `-- --exact` for single test runs to avoid partial matches

Generated code:
- `groundmodels-core/src/agsi.rs` appears generated from schema
- Avoid manual edits unless regeneration is not possible
- Preserve clippy allow attributes in generated files

## CLI Conventions
- `clap` is used for argument parsing
- Subcommands live in `groundmodels-cli/src/main.rs`
- Keep new subcommands consistent with existing help text style

## Data / Schema Conventions
- AGSi JSON schema is embedded in `groundmodels-core/src/AGSi_JSONSchema_v1-0-1_2020-12.json`
- Prefer `serde_json::Value` for loose parsing in CLI tools
- When adding schema-driven structs, place them in core crate

## Project Structure
- `groundmodels-core`: soil parameters, ground models, AGSi conversion logic
- `groundmodels-cli`: CLI, LSP server, validation, interactive generation
- `groundmodels-py`: Python bindings using PyO3/maturin
- Root `Cargo.toml` defines workspace dependencies and metadata

## Editor Rules
- No Cursor rules found in `.cursor/rules/` or `.cursorrules`
- No Copilot rules found in `.github/copilot-instructions.md`

## Practical Tips for Agents
- Prefer minimal, targeted changes over broad refactors
- Keep public API changes explicit and documented
- Update tests alongside behavior changes
- Avoid introducing new dependencies unless necessary
