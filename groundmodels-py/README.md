# GroundModels Python Bindings

Python bindings for the groundmodels Rust library, providing geotechnical ground modeling and soil parameter analysis capabilities.

## Installation

```bash
pip install groundmodels-py
```

Or build from source:

```bash
cd groundmodels-py
pip install maturin
maturin develop
```

## Usage

```python
import groundmodels_py as gm

# Load AGSi data from JSON string
agsi_json = '''
{
    "agsFile": {"producedBy": "Example", "title": "Test"},
    "agsSchema": {"version": "1.0.1"},
    "agsiModel": [...]
}
'''

# Create soil parameters from AGSi data
soil_params = gm.SoilParams.from_agsi_json(agsi_json)
print(f"Unit weight: {soil_params.unit_weight}")
print(f"Young's modulus: {soil_params.youngs_modulus}")

# Create ground model from AGSi data
ground_model = gm.GroundModel.from_agsi_json(agsi_json)
print(f"Number of soil parameter sets: {len(ground_model)}")

# Convert AGSi to JSON
result = gm.convert_agsi_to_json(agsi_json, "soil_params")
print(result)
```

## Classes

### SoilParams
- `unit_weight`: Soil unit weight (kN/m³)
- `youngs_modulus`: Young's modulus (kPa)
- `phi_prime`: Effective friction angle (degrees)
- `c_prime`: Effective cohesion (kPa)
- `cu`: Undrained shear strength (kPa)
- `behaviour`: Soil behaviour type

### GroundModel
- `soil_params`: List of SoilParams objects
- `from_agsi_json()`: Create from AGSi JSON string
- `to_dict()`: Convert to Python dictionary

## Functions

### convert_agsi_to_json(agsi_json, convert_type)
Convert AGSi data to JSON format.

**Parameters:**
- `agsi_json`: AGSi data as JSON string
- `convert_type`: "soil_params" or "ground_model"

**Returns:**
- JSON string of converted data