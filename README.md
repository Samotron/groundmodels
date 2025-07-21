# GroundModels

A comprehensive Rust library and toolset for geotechnical ground modeling and soil parameter analysis, with support for AGSi (Association of Geotechnical & Geoenvironmental Specialists interpreted) data format.

## Workspace Structure

This workspace contains three main components:

- **`groundmodels-core/`** - Core Rust library with geotechnical modeling functionality
- **`groundmodels-cli/`** - Command-line interface for the library
- **`groundmodels-py/`** - Python bindings for the core library

## Quick Start

### Building the Workspace

```bash
# Build all components
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

### Using the CLI

```bash
# Build and install the CLI
cargo install --path groundmodels-cli

# Convert AGSi JSON to soil parameters
groundmodels convert -i data.json -o output.json --convert-type soil-params

# Analyze AGSi data
groundmodels analyze -i data.json

# Generate example data
groundmodels example -o example.json
```

### Using the Python Bindings

```bash
# Build and install Python bindings
cd groundmodels-py
pip install maturin
maturin develop

# Use in Python
python -c "import groundmodels_py; print('Success!')"
```

## Core Library Features

- **AGSi Data Support**: Parse and process AGSi JSON format files
- **Soil Parameter Analysis**: Extract and analyze geotechnical soil parameters
- **Ground Model Creation**: Build comprehensive ground models from data
- **Multiple Soil Types**: Support for granular, cohesive, and rock materials
- **Advanced Parameters**: Handle complex geotechnical properties
- **Hoek-Brown Criteria**: Rock mass characterization support

## CLI Features

- **Data Conversion**: Convert AGSi data to different formats
- **Analysis Tools**: Analyze soil parameters and ground models
- **Example Generation**: Create sample AGSi data files
- **JSON Output**: Structured output for integration with other tools

## Python Bindings Features

- **Native Python API**: Pythonic interface to core functionality
- **AGSi Processing**: Direct JSON string processing
- **Data Export**: Convert results to Python dictionaries
- **Type Safety**: Leverages Rust's type system for reliability

## Development

### Prerequisites

- Rust 1.70+
- Python 3.8+ (for Python bindings)
- Maturin (for Python bindings development)

### Building Individual Components

```bash
# Core library
cd groundmodels-core
cargo build

# CLI
cd groundmodels-cli  
cargo build

# Python bindings
cd groundmodels-py
maturin develop
```

### Running Tests

```bash
# All tests
cargo test

# Specific component
cargo test -p groundmodels-core
cargo test -p groundmodels-cli
```

## Examples

See the `examples/` directory in each component for usage examples.

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request