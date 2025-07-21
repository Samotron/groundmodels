use pyo3::prelude::*;
use pyo3::types::PyDict;
use groundmodels_core::{SoilParams, GroundModel};
use serde_json;

#[pyclass]
#[derive(Clone)]
pub struct PySoilParams {
    inner: SoilParams,
}

#[pymethods]
impl PySoilParams {
    #[new]
    fn new() -> Self {
        Self {
            inner: SoilParams::default(),
        }
    }

    #[getter]
    fn unit_weight(&self) -> f64 {
        self.inner.unit_weight
    }

    #[setter]
    fn set_unit_weight(&mut self, value: f64) {
        self.inner.unit_weight = value;
    }

    #[getter]
    fn youngs_modulus(&self) -> f64 {
        self.inner.youngs_modulus
    }

    #[setter]
    fn set_youngs_modulus(&mut self, value: f64) {
        self.inner.youngs_modulus = value;
    }

    #[getter]
    fn phi_prime(&self) -> Option<f64> {
        self.inner.phi_prime
    }

    #[setter]
    fn set_phi_prime(&mut self, value: Option<f64>) {
        self.inner.phi_prime = value;
    }

    #[getter]
    fn c_prime(&self) -> Option<f64> {
        self.inner.c_prime
    }

    #[setter]
    fn set_c_prime(&mut self, value: Option<f64>) {
        self.inner.c_prime = value;
    }

    #[getter]
    fn cu(&self) -> Option<f64> {
        self.inner.cu
    }

    #[setter]
    fn set_cu(&mut self, value: Option<f64>) {
        self.inner.cu = value;
    }

    #[getter]
    fn behaviour(&self) -> String {
        format!("{:?}", self.inner.behaviour)
    }

    fn __repr__(&self) -> String {
        format!(
            "SoilParams(unit_weight={}, youngs_modulus={}, behaviour={})",
            self.inner.unit_weight, self.inner.youngs_modulus, self.behaviour()
        )
    }

    #[staticmethod]
    fn from_agsi_json(agsi_json: &str) -> PyResult<Self> {
        let agsi_data: serde_json::Value = serde_json::from_str(agsi_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        
        // Extract first soil params from ground model
        let ground_model = GroundModel::from_agsi_file(&agsi_data);
        let soil_params = if let Some(first_params) = ground_model.soil_params.first() {
            first_params.clone()
        } else {
            SoilParams::default()
        };
        Ok(Self { inner: soil_params })
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("unit_weight", self.inner.unit_weight)?;
            dict.set_item("youngs_modulus", self.inner.youngs_modulus)?;
            dict.set_item("phi_prime", self.inner.phi_prime)?;
            dict.set_item("c_prime", self.inner.c_prime)?;
            dict.set_item("cu", self.inner.cu)?;
            dict.set_item("behaviour", format!("{:?}", self.inner.behaviour))?;
            Ok(dict.to_object(py))
        })
    }
}

#[pyclass]
pub struct PyGroundModel {
    inner: GroundModel,
}

#[pymethods]
impl PyGroundModel {
    #[new]
    fn new() -> Self {
        Self {
            inner: GroundModel::default(),
        }
    }

    #[getter]
    fn soil_params(&self) -> Vec<PySoilParams> {
        self.inner.soil_params.iter()
            .map(|params| PySoilParams { inner: params.clone() })
            .collect()
    }

    fn __len__(&self) -> usize {
        self.inner.soil_params.len()
    }

    fn __repr__(&self) -> String {
        format!("GroundModel(soil_params_count={})", self.inner.soil_params.len())
    }

    #[staticmethod]
    fn from_agsi_json(agsi_json: &str) -> PyResult<Self> {
        let agsi_data: serde_json::Value = serde_json::from_str(agsi_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        
        let ground_model = GroundModel::from_agsi_file(&agsi_data);
        Ok(Self { inner: ground_model })
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            let soil_params_list: PyResult<Vec<PyObject>> = self.inner.soil_params.iter()
                .map(|params| {
                    let py_params = PySoilParams { inner: params.clone() };
                    py_params.to_dict()
                })
                .collect();
            dict.set_item("soil_params", soil_params_list?)?;
            Ok(dict.to_object(py))
        })
    }
}

#[pyfunction]
fn convert_agsi_to_json(agsi_json: &str, convert_type: &str) -> PyResult<String> {
    let agsi_data: serde_json::Value = serde_json::from_str(agsi_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    let result = match convert_type.to_lowercase().as_str() {
        "soil_params" | "soilparams" => {
            let ground_model = GroundModel::from_agsi_file(&agsi_data);
            let soil_params = if let Some(first_params) = ground_model.soil_params.first() {
                first_params.clone()
            } else {
                SoilParams::default()
            };
            serde_json::to_string_pretty(&soil_params)
        }
        "ground_model" | "groundmodel" => {
            let ground_model = GroundModel::from_agsi_file(&agsi_data);
            serde_json::to_string_pretty(&ground_model)
        }
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Invalid convert_type. Use 'soil_params' or 'ground_model'"
        )),
    };

    result.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pymodule]
fn groundmodels_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySoilParams>()?;
    m.add_class::<PyGroundModel>()?;
    m.add_function(wrap_pyfunction!(convert_agsi_to_json, m)?)?;
    Ok(())
}