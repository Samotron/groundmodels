# AGENTS.md - Groundmodels Codebase Guide

## Build/Test/Lint Commands
- **Build all**: `cargo build` (workspace builds all crates)
- **Test all**: `cargo test` (runs all tests across workspace)
- **Test specific crate**: `cargo test -p groundmodels-core`
- **Run single test**: `cargo test test_ground_model_creation -- --exact`
- **CLI build**: `cargo build -p groundmodels-cli`
- **CLI run**: `cargo run -p groundmodels-cli -- <command>`
- **Python build**: `cd groundmodels-py && maturin develop`
- **Format**: `cargo fmt`
- **Clippy**: `cargo clippy`

## Code Style Guidelines
- **Imports**: Standard library first, external crates, then local modules (use `mod` statements at top)
- **Naming**: snake_case for functions/variables, PascalCase for types/enums, SCREAMING_SNAKE_CASE for constants
- **Error handling**: Use `Result<T, E>` with descriptive error messages, prefer `?` operator
- **Documentation**: Public APIs must have doc comments with `///`, include examples for complex functions
- **Types**: Prefer explicit types over inference in function signatures, use `Option<T>` for nullable values
- **Serde**: Use `#[derive(Serialize, Deserialize)]` for data structures, prefer snake_case field names

## Project Structure
- **groundmodels-core**: Core geotechnical modeling library (soil parameters, ground models, AGSI conversion)
- **groundmodels-cli**: Command-line interface with LSP server, validation, interactive generation
- **groundmodels-py**: Python bindings using PyO3/maturin for the core library
- Workspace configured in root Cargo.toml with shared dependencies and metadata