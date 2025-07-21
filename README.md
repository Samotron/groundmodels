# GroundModels

A comprehensive Rust library and CLI toolkit for geotechnical ground modeling and soil parameter analysis, with Python bindings and AGSi (Applied Ground Science Information) file support.

## 🌍 Overview

GroundModels provides tools for:
- Converting AGSi JSON files to soil parameters and ground models
- Geotechnical calculations (earth pressure coefficients, Hoek-Brown conversions)
- Interactive AGSi file generation and validation
- Language Server Protocol (LSP) support for AGSi files
- Exporting data to formatted tables and Word documents

## 📦 Project Structure

- **`groundmodels-core`** - Core Rust library with geotechnical modeling functionality
- **`groundmodels-cli`** - Command-line interface with LSP server and interactive tools
- **`groundmodels-py`** - Python bindings using PyO3 for the core library

## 🚀 Quick Start

### CLI Installation

```bash
# Build from source
git clone <repository-url>
cd groundmodels
cargo build --release

# Run CLI
cargo run -p groundmodels-cli -- --help
```

### Python Installation

```bash
cd groundmodels-py
pip install maturin
maturin develop
```

## 🔧 CLI Usage

### Convert AGSi Files

```bash
# Convert AGSi to soil parameters
groundmodels convert -i input.json -o output.json -c soil-params

# Convert AGSi to ground model
groundmodels convert -i input.json -o output.json -c ground-model
```

### Interactive AGSi Generation

```bash
# Generate AGSi file interactively
groundmodels generate -o new_model.json
```

### Validate AGSi Files

```bash
# Validate against AGSi schema
groundmodels validate -i model.json
```

### Export to Tables

```bash
# Display as formatted tables
groundmodels table input.json

# Export to Word document
groundmodels table input.json --word output.docx
```

### Language Server

```bash
# Start LSP server for IDE integration
groundmodels lsp --port 3000
```

## 📚 Library Usage

### Rust

```rust
use groundmodels_core::{GroundModel, SoilParams, ConvertType, convert_agsi_file};

// Convert AGSi file
let result = convert_agsi_file(
    "input.json", 
    ConvertType::GroundModel, 
    Some("output.json")
)?;

// Create ground model from AGSi
let agsi_data: serde_json::Value = serde_json::from_str(&json_content)?;
let ground_model = GroundModel::from_agsi_file(&agsi_data);

// Access soil parameters
for params in &ground_model.soil_params {
    println!("Unit weight: {} kN/m³", params.unit_weight);
    if let Some(phi) = params.phi_prime {
        println!("Friction angle: {:.1}°", phi.to_degrees());
    }
}
```

### Python

```python
import groundmodels_py as gm

# Create ground model from AGSi JSON
ground_model = gm.GroundModel.from_agsi_json(agsi_json_string)

# Access soil parameters
for params in ground_model.soil_params:
    print(f"Unit weight: {params.unit_weight} kN/m³")
    print(f"Young's modulus: {params.youngs_modulus} kPa")

# Convert AGSi to JSON
result = gm.convert_agsi_to_json(agsi_json, "soil_params")
```

## 🧮 Geotechnical Features

### Soil Parameter Analysis
- Unit weight, friction angle, cohesion
- Young's modulus, Poisson's ratio
- Undrained shear strength
- Volume compressibility

### Rock Parameter Analysis
- Hoek-Brown parameters (mi, GSI, UCS)
- Equivalent Mohr-Coulomb conversion
- Rock mass modulus calculations

### Ground Model Operations
- Layer-based soil profiles
- Groundwater level handling
- Stress calculations (total, effective, pore water pressure)
- Excavation modeling

### Earth Pressure Calculations
- Active/passive earth pressure coefficients
- At-rest earth pressure (K₀)
- Coulomb's theory implementation

## 📋 AGSi Parameter Support

| AGSi Code ID | Field | Description |
|--------------|-------|-------------|
| `UnitWeight` | unit_weight | Unit weight (kN/m³) |
| `AngleFriction` | phi_prime | Effective friction angle |
| `Cohesion` | c_prime | Effective cohesion (kPa) |
| `UndrainedShearStrength` | cu | Undrained shear strength (kPa) |
| `YoungsModulus` | youngs_modulus | Young's modulus (kPa) |
| `GeologicalStrengthIndex` | gsi | Geological Strength Index |
| `UnconfinedCompressiveStrength` | ucs | UCS (kPa) |
| `HoekBrownParamMi` | mi | Hoek-Brown parameter mi |

## 🔨 Development

### Build Commands

```bash
# Build workspace
cargo build

# Run tests
cargo test

# Run specific test
cargo test test_ground_model_creation -- --exact

# Format code
cargo fmt

# Lint code
cargo clippy

# Build Python bindings
cd groundmodels-py && maturin develop
```

### Testing

```bash
# Test core library
cargo test -p groundmodels-core

# Test CLI
cargo test -p groundmodels-cli

# Test specific functionality
cargo test hoek_brown -- --exact
```

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## 📖 Documentation

- [Core Library README](groundmodels-core/README.md)
- [Python Bindings README](groundmodels-py/README.md)
- [AGENTS.md](AGENTS.md) - Development guide for AI coding assistants