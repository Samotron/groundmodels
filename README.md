# GroundModels - AGSi Conversion Library

A Rust library for converting AGSi (Applied Ground Science Information) files to soil parameters and ground models.

## Overview

This library provides functionality equivalent to the TypeScript `addConvertCommand` function, allowing you to:

1. Convert AGSi files to arrays of `SoilParams`
2. Convert AGSi files to `GroundModel` objects
3. Export results to JSON format

## Core Functions

### `convert_agsi_file`

The main conversion function that processes AGSi JSON files:

```rust
pub fn convert_agsi_file(
    file_path: &str, 
    convert_type: ConvertType, 
    output_path: Option<&str>
) -> Result<String, Box<dyn std::error::Error>>
```

### `SoilParams::from_agsi_data_parameters`

Converts AGSi data parameter values to soil parameters:

```rust
pub fn from_agsi_data_parameters(data: &[AgsiDataParameterValueElement]) -> Self
```

### `GroundModel::from_agsi_file`

Creates a ground model from AGSi JSON data:

```rust
pub fn from_agsi_file(agsi_json: &serde_json::Value) -> Self
```

## Supported AGSi Parameters

The conversion process maps the following AGSi parameter codes:

| AGSi Code ID | SoilParams Field | Description |
|--------------|------------------|-------------|
| `UnitWeight` | `unit_weight` | Unit weight of soil |
| `AngleFriction` | `phi_prime` | Angle of internal friction |
| `UndrainedShearStrength` | `cu` | Undrained shear strength (sets behavior to Cohesive) |
| `YoungsModulus` | `youngs_modulus` | Young's modulus |
| `Cohesion` | `c_prime` | Effective cohesion |
| `ModulusOfVolumeCompressibility` | `mv` | Coefficient of volume compressibility |
| `GeologicalStrengthIndex` | `gsi` | Geological Strength Index |
| `UnconfinedCompressiveStrength` | `ucs` | UCS (sets behavior to Rock) |
| `HoekBrownParamMi` | `mi` | Hoek-Brown parameter mi |
| `Disturbance` | `disturbance` | Disturbance factor |

Any unrecognized parameters are stored in the `advanced_parameters` field.

## Usage Examples

### Basic Conversion

```rust
use groundmodels::{convert_agsi_file, ConvertType};

// Convert to SoilParams array
let result = convert_agsi_file(
    "input.json", 
    ConvertType::SoilParams, 
    Some("output.json")
)?;

// Convert to GroundModel
let result = convert_agsi_file(
    "input.json", 
    ConvertType::GroundModel, 
    None // Print to stdout
)?;
```

### Direct Parameter Conversion

```rust
use groundmodels::{SoilParams, AgsiDataParameterValueElement};

let data = vec![
    AgsiDataParameterValueElement {
        code_id: "UnitWeight".to_string(),
        value_numeric: Some(18.0),
    },
    AgsiDataParameterValueElement {
        code_id: "AngleFriction".to_string(),
        value_numeric: Some(30.0),
    },
];

let soil_params = SoilParams::from_agsi_data_parameters(&data);
```

## Soil Behavior Classification

The library automatically determines soil behavior based on parameters:

- **Cohesive**: When `UndrainedShearStrength` is provided with a positive value
- **Rock**: When `UnconfinedCompressiveStrength` is provided
- **Granular**: Default behavior

## Data Structures

### `AgsiDataParameterValueElement`

```rust
pub struct AgsiDataParameterValueElement {
    pub code_id: String,
    pub value_numeric: Option<f64>,
}
```

### `ConvertType`

```rust
pub enum ConvertType {
    SoilParams,
    GroundModel,
}
```

### `AdvancedParameter`

```rust
pub struct AdvancedParameter {
    pub name: String,
    pub value: f64,
}
```

## Error Handling

The conversion functions return `Result` types for proper error handling:

```rust
match convert_agsi_file("input.json", ConvertType::SoilParams, None) {
    Ok(json_output) => println!("Success: {}", json_output),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Input Format

The expected AGSi JSON structure:

```json
{
  "agsiModel": [
    {
      "agsiModelElement": [
        {
          "elementName": "Layer Name",
          "agsiDataParameterValue": [
            {
              "codeID": "UnitWeight",
              "valueNumeric": 18.0
            }
          ]
        }
      ]
    }
  ]
}
```

## Output Format

### SoilParams Array
Returns a JSON array of soil parameter objects with all the geotechnical properties.

### GroundModel
Returns a JSON object containing:
- `soil_layers`: Array of soil layers (empty in basic conversion)
- `soil_params`: Array of soil parameters
- `rigid_boundary`: Optional rigid boundary depth
- `groundwater`: Groundwater level
- `reference`: Reference identifier