use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
pub mod agsi;
pub mod soil_description;
pub mod strip_log;

#[cfg(test)]
mod soil_description_tests;
use crate::agsi::AgsiDataParameterValue;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConvertType {
    SoilParams,
    GroundModel,
}

impl ConvertType {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "soilparams" => Ok(ConvertType::SoilParams),
            "groundmodel" => Ok(ConvertType::GroundModel),
            _ => Err("Invalid convert type. Use 'soilparams' or 'groundmodel'"),
        }
    }
}

pub fn convert_agsi_file(
    file_path: &str,
    convert_type: ConvertType,
    output_path: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(file_path)?;
    let agsi: serde_json::Value = serde_json::from_str(&text)?;

    let output_json = match convert_type {
        ConvertType::GroundModel => {
            let ground_model = GroundModel::from_agsi_file(&agsi);
            serde_json::to_string_pretty(&ground_model)?
        }
        ConvertType::SoilParams => {
            let mut soil_params = Vec::new();

            if let Some(agsi_models) = agsi["agsiModel"].as_array() {
                if let Some(first_model) = agsi_models.get(0) {
                    if let Some(elements) = first_model["agsiModelElement"].as_array() {
                        for element in elements {
                            if let Some(param_values) = element["agsiDataParameterValue"].as_array()
                            {
                                let params_data: Vec<AgsiDataParameterValue> = param_values
                                    .iter()
                                    .filter_map(|p| {
                                        Some(AgsiDataParameterValue {
                                            code_id: p["codeID"].as_str()?.parse().ok()?,
                                            case_id: None,
                                            data_id: None,
                                            remarks: None,
                                            value_numeric: p["valueNumeric"].as_f64(),
                                            value_profile: None,
                                            value_profile_ind_var_code_id: None,
                                            value_text: None,
                                        })
                                    })
                                    .collect();

                                if !params_data.is_empty() {
                                    soil_params
                                        .push(SoilParams::from_agsi_data_parameters(&params_data));
                                }
                            }
                        }
                    }
                }
            }

            serde_json::to_string_pretty(&soil_params)?
        }
    };

    if let Some(output_file) = output_path {
        let mut file = fs::File::create(output_file)?;
        file.write_all(output_json.as_bytes())?;
        println!("Output written to {}", output_file);
    } else {
        println!("{}", output_json);
    }

    Ok(output_json)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedParameter {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SoilType {
    Cohesive,
    Granular,
    Rock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialFactors {
    pub gamma_phi: f64,
    pub gamma_c: f64,
    pub gamma_gamma: f64,
    pub gamma_cu: f64,
}

impl PartialFactors {
    pub fn new(gamma_phi: f64, gamma_c: f64, gamma_gamma: f64, gamma_cu: f64) -> Self {
        PartialFactors {
            gamma_phi,
            gamma_c,
            gamma_gamma,
            gamma_cu,
        }
    }
}

impl Default for PartialFactors {
    fn default() -> Self {
        PartialFactors {
            gamma_phi: 1.0,
            gamma_c: 1.0,
            gamma_gamma: 1.0,
            gamma_cu: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilParams {
    pub reference: String,
    pub behaviour: SoilType,
    pub phi_prime: Option<f64>,
    pub c_prime: Option<f64>,
    pub unit_weight: f64,
    pub cu: Option<f64>,
    pub mv: f64,
    pub youngs_modulus: f64,
    pub poissons_ratio: f64,
    pub coefficient_of_consolidation: f64,
    pub gsi: Option<f64>,
    pub ucs: Option<f64>,
    pub mi: Option<f64>,
    pub disturbance: f64,
    pub advanced_parameters: Option<Vec<AdvancedParameter>>,
    pub factored: bool,
    pub factors: Option<PartialFactors>,
}

impl SoilParams {
    pub fn new(
        reference: String,
        mv: f64,
        youngs_modulus: f64,
        poissons_ratio: f64,
        coefficient_of_consolidation: f64,
        behaviour: SoilType,
        unit_weight: f64,
    ) -> Self {
        SoilParams {
            reference,
            behaviour,
            phi_prime: None,
            c_prime: None,
            unit_weight,
            cu: None,
            mv,
            youngs_modulus,
            poissons_ratio,
            coefficient_of_consolidation,
            gsi: None,
            ucs: None,
            mi: None,
            disturbance: 0.0,
            advanced_parameters: None,
            factored: false,
            factors: None,
        }
    }

    pub fn from_agsi_data_parameters(data: &[AgsiDataParameterValue]) -> Self {
        let mut sp = SoilParams::default();

        for item in data {
            match item.code_id.as_str() {
                "UnitWeight" => {
                    sp.unit_weight = item.value_numeric.unwrap_or(0.0);
                }
                "AngleFriction" | "EffectiveFrictionAngle" => {
                    sp.phi_prime = item.value_numeric;
                }
                "UndrainedShearStrength" => {
                    if let Some(value) = item.value_numeric {
                        if value > 0.0 {
                            sp.cu = Some(value);
                            sp.behaviour = SoilType::Cohesive;
                        } else {
                            sp.cu = Some(0.0);
                        }
                    } else {
                        sp.cu = Some(0.0);
                    }
                }
                "YoungsModulus" => {
                    sp.youngs_modulus = item.value_numeric.unwrap_or(0.0);
                }
                "Cohesion" | "EffectiveCohesion" => {
                    sp.c_prime = item.value_numeric;
                }
                "ModulusOfVolumeCompressibility" => {
                    sp.mv = item.value_numeric.unwrap_or(0.0);
                }
                "GeologicalStrengthIndex" => {
                    sp.gsi = item.value_numeric;
                }
                "UnconfinedCompressiveStrength" => {
                    sp.ucs = item.value_numeric;
                    if item.value_numeric.is_some() {
                        sp.behaviour = SoilType::Rock;
                    }
                }
                "HoekBrownParamMi" => {
                    sp.mi = item.value_numeric;
                }
                "Disturbance" => {
                    sp.disturbance = item.value_numeric.unwrap_or(0.0);
                }
                _ => {
                    if sp.advanced_parameters.is_none() {
                        sp.advanced_parameters = Some(Vec::new());
                    }
                    sp.advanced_parameters
                        .as_mut()
                        .unwrap()
                        .push(AdvancedParameter {
                            name: item.code_id.to_string(),
                            value: item.value_numeric.unwrap_or(0.0),
                        });
                }
            }
        }

        sp
    }

    pub fn with_all_fields(
        reference: String,
        behaviour: SoilType,
        phi_prime: Option<f64>,
        c_prime: Option<f64>,
        unit_weight: f64,
        cu: Option<f64>,
        mv: f64,
        youngs_modulus: f64,
        poissons_ratio: f64,
        coefficient_of_consolidation: f64,
        gsi: Option<f64>,
        ucs: Option<f64>,
        mi: Option<f64>,
        disturbance: f64,
    ) -> Self {
        SoilParams {
            reference,
            behaviour,
            phi_prime,
            c_prime,
            unit_weight,
            cu,
            mv,
            youngs_modulus,
            poissons_ratio,
            coefficient_of_consolidation,
            gsi,
            ucs,
            mi,
            disturbance,
            advanced_parameters: None,
            factored: false,
            factors: None,
        }
    }

    pub fn apply_partial_factors(&self, pf: &PartialFactors) -> SoilParams {
        let mut result = self.clone();

        if let Some(phi) = self.phi_prime {
            result.phi_prime = Some((phi.tan() / pf.gamma_phi).atan());
        }

        if let Some(c) = self.c_prime {
            result.c_prime = Some(c / pf.gamma_c);
        }

        result.unit_weight = self.unit_weight / pf.gamma_gamma;

        if let Some(cu_val) = self.cu {
            result.cu = Some(cu_val / pf.gamma_cu);
        }

        result.factored = true;
        result.factors = Some(pf.clone());
        result
    }

    pub fn remove_partial_factors(&self) -> Result<SoilParams, &'static str> {
        let pf = self
            .factors
            .as_ref()
            .ok_or("No factors stored on this instance")?;
        let mut result = self.clone();

        if let Some(phi) = self.phi_prime {
            result.phi_prime = Some((phi.tan() * pf.gamma_phi).atan());
        }

        if let Some(c) = self.c_prime {
            result.c_prime = Some(c * pf.gamma_c);
        }

        result.unit_weight = self.unit_weight * pf.gamma_gamma;

        if let Some(cu_val) = self.cu {
            result.cu = Some(cu_val * pf.gamma_cu);
        }

        result.factored = false;
        result.factors = None;
        Ok(result)
    }

    fn coulomb_ka(&self, phi: f64, beta: f64) -> f64 {
        let cos_beta = beta.cos();
        let cos_phi = phi.cos();
        let discriminant = cos_beta.powi(2) - cos_phi.powi(2);
        let sqrt_disc = discriminant.sqrt();

        (cos_beta - sqrt_disc) / (cos_beta + sqrt_disc)
    }

    pub fn get_k_active(&self, slope: Option<f64>) -> Result<f64, &'static str> {
        let phi = self.phi_prime.ok_or("Phi must be a value")?;

        match slope {
            None => Ok((1.0 - phi.sin()) / (1.0 + phi.sin())),
            Some(beta) => Ok(self.coulomb_ka(phi, beta)),
        }
    }

    pub fn get_k_passive(&self, slope: Option<f64>) -> Result<f64, &'static str> {
        match slope {
            None => Ok(1.0 / self.get_k_active(None)?),
            Some(_) => self.get_k_passive(None), // TODO: implement Coulomb method for passive
        }
    }

    pub fn k0(&self) -> Result<f64, &'static str> {
        let phi = self.phi_prime.ok_or("Phi must be a value")?;
        Ok(1.0 - phi.sin())
    }

    pub fn mb(&self) -> Result<f64, &'static str> {
        let mi = self
            .mi
            .ok_or("mi (rock mass factor) must be set for mb calculation")?;
        let gsi = self
            .gsi
            .ok_or("gsi (geological strength index) must be set for mb calculation")?;

        let exponent = (gsi - 100.0) / (28.0 - (14.0 * self.disturbance));
        Ok(mi * exponent.exp())
    }

    pub fn s(&self) -> Result<f64, &'static str> {
        let gsi = self
            .gsi
            .ok_or("gsi (geological strength index) must be set for s calculation")?;

        let exponent = (gsi - 100.0) / (9.0 - (3.0 * self.disturbance));
        Ok(exponent.exp())
    }

    pub fn a(&self) -> Result<f64, &'static str> {
        let gsi = self
            .gsi
            .ok_or("gsi (geological strength index) must be set for a calculation")?;

        Ok(0.5 + ((-gsi / 15.0).exp() - (-20.0_f64 / 3.0).exp()))
    }

    fn hb_to_mc_conv(&self, sig3: f64) -> Result<f64, &'static str> {
        let ucs = self.ucs.ok_or("ucs (unconfined compressive strength) must be set and nonzero for Hoek-Brown conversion")?;
        if ucs == 0.0 {
            return Err(
                "ucs (unconfined compressive strength) must be nonzero for Hoek-Brown conversion",
            );
        }

        let sig3n = sig3 / ucs;
        let first_bit = 6.0 * self.a()? * self.mb()?;
        let second_bit = (self.s()? + (self.mb()? * sig3n)).powf(self.a()? - 1.0);

        Ok(first_bit * second_bit)
    }

    pub fn hb_equiv_phi_ang(&self, sig3: f64) -> Result<f64, &'static str> {
        let top = self.hb_to_mc_conv(sig3)?;
        let bottom = (2.0 * (1.0 + self.a()?) * (2.0 + self.a()?)) + top;

        if bottom == 0.0 {
            return Err("Denominator for equivalent phi angle calculation is zero");
        }

        Ok(top / bottom)
    }

    pub fn hb_equiv_c_prime(&self, sig3: f64) -> Result<f64, &'static str> {
        let ucs = self.ucs.ok_or("ucs (unconfined compressive strength) must be set and nonzero for equivalent cohesion calculation")?;
        if ucs == 0.0 {
            return Err("ucs must be nonzero for equivalent cohesion calculation");
        }

        let sig3n = sig3 / ucs;
        let a_val = self.a()?;
        let s_val = self.s()?;
        let mb_val = self.mb()?;

        let first_brack = ((1.0 + (2.0 * a_val)) * s_val) + ((1.0 - a_val) * mb_val * sig3n);
        let top = ucs * first_brack * ((s_val * mb_val * sig3n).powf(a_val - 1.0));
        let denom = (1.0 + a_val) * (2.0 + a_val);

        if denom == 0.0 {
            return Err("Denominator for equivalent cohesion calculation is zero");
        }

        let sqrt_bit = 1.0 + (self.hb_to_mc_conv(sig3)? / denom);
        if sqrt_bit < 0.0 {
            return Err("sqrtBit for equivalent cohesion calculation is negative");
        }

        let bottom = denom * sqrt_bit.sqrt();
        if bottom == 0.0 {
            return Err("Denominator for equivalent cohesion calculation is zero");
        }

        Ok(top / bottom)
    }

    pub fn rock_e_val(&self) -> Result<f64, &'static str> {
        let ucs = self.ucs.ok_or(
            "ucs (unconfined compressive strength) must be set for Young's Modulus calculation",
        )?;
        let gsi = self
            .gsi
            .ok_or("gsi (geological strength index) must be set for Young's Modulus calculation")?;

        let rock_val = if ucs < 100.0 { 1.0 } else { ucs / 100.0 };

        Ok((1.0 - (self.disturbance / 2.0))
            * rock_val.sqrt()
            * (10.0_f64.powf((gsi - 10.0) / 40.0)))
    }

    pub fn convert_equivalent_rock(&self, sig3: f64) -> Result<SoilParams, &'static str> {
        Ok(SoilParams::with_all_fields(
            self.reference.clone(),
            self.behaviour,
            Some(self.hb_equiv_phi_ang(sig3)?),
            Some(self.hb_equiv_c_prime(sig3)?),
            self.unit_weight,
            self.cu,
            self.mv,
            self.rock_e_val()?,
            self.poissons_ratio,
            self.coefficient_of_consolidation,
            self.gsi,
            self.ucs,
            self.mi,
            self.disturbance,
        ))
    }
}

impl Default for SoilParams {
    fn default() -> Self {
        SoilParams {
            reference: String::new(),
            behaviour: SoilType::Granular,
            phi_prime: None, // Set to None so error handling tests work
            c_prime: None,
            unit_weight: 0.0,
            cu: None,
            mv: 0.0,
            youngs_modulus: 0.0,
            poissons_ratio: 0.0,
            coefficient_of_consolidation: 0.0,
            gsi: None,
            ucs: None,
            mi: None,
            disturbance: 0.0,
            advanced_parameters: None,
            factored: false,
            factors: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilLayer {
    pub unit_reference: String,
    pub top_level: f64,
    pub base_level: Option<f64>,
    pub base_unit_reference: Option<String>,
    pub typical_description: String,
    pub geol_code: String,
    pub reference: String, // keeping for backward compatibility
}

impl SoilLayer {
    pub fn new(top_level: f64, base_level: f64, reference: String) -> Self {
        SoilLayer {
            unit_reference: String::new(),
            top_level,
            base_level: Some(base_level),
            base_unit_reference: None,
            typical_description: String::new(),
            geol_code: String::new(),
            reference,
        }
    }

    pub fn with_unit_reference(
        top_level: f64,
        base_level: f64,
        reference: String,
        unit_reference: String,
    ) -> Self {
        SoilLayer {
            unit_reference,
            top_level,
            base_level: Some(base_level),
            base_unit_reference: None,
            typical_description: String::new(),
            geol_code: String::new(),
            reference,
        }
    }

    pub fn with_all_fields(
        unit_reference: String,
        top_level: f64,
        base_level: Option<f64>,
        base_unit_reference: Option<String>,
        typical_description: String,
        geol_code: String,
    ) -> Self {
        SoilLayer {
            unit_reference,
            top_level,
            base_level,
            base_unit_reference,
            typical_description,
            geol_code,
            reference: String::new(),
        }
    }

    pub fn excavate_layer(&self, level: f64) -> (SoilLayer, bool) {
        let base_level = self.base_level.unwrap_or(0.0);

        if level < self.top_level && level > base_level {
            // Excavation cuts through the layer
            (
                SoilLayer {
                    unit_reference: self.unit_reference.clone(),
                    top_level: level,
                    base_level: self.base_level,
                    base_unit_reference: self.base_unit_reference.clone(),
                    typical_description: self.typical_description.clone(),
                    geol_code: self.geol_code.clone(),
                    reference: self.reference.clone(),
                },
                false,
            )
        } else if level > self.top_level {
            // Excavation is above the layer - keep the layer
            (self.clone(), false)
        } else {
            // Excavation removes the entire layer
            (
                SoilLayer {
                    unit_reference: self.unit_reference.clone(),
                    top_level: self.top_level,
                    base_level: self.base_level,
                    base_unit_reference: self.base_unit_reference.clone(),
                    typical_description: self.typical_description.clone(),
                    geol_code: "DELETE".to_string(),
                    reference: self.reference.clone(),
                },
                true,
            )
        }
    }

    pub fn process_layer(&self, sig3: f64, soil_type: SoilType) -> SoilLayer {
        match soil_type {
            SoilType::Rock => {
                // In a real implementation, you would apply rock conversion logic here
                // For now, just return a clone
                let mut processed = self.clone();
                processed.typical_description = format!(
                    "{} (processed with sig3: {})",
                    processed.typical_description, sig3
                );
                processed
            }
            _ => self.clone(),
        }
    }
}

impl Default for SoilLayer {
    fn default() -> Self {
        SoilLayer {
            unit_reference: String::new(),
            top_level: 100.0,
            base_level: Some(0.0),
            base_unit_reference: None,
            typical_description: String::new(),
            geol_code: String::new(),
            reference: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroundModel {
    pub soil_layers: Vec<SoilLayer>,
    pub soil_params: Vec<SoilParams>,
    pub rigid_boundary: Option<f64>,
    pub groundwater: f64,
    pub reference: String,
}

impl GroundModel {
    pub fn new(soil_layers: Vec<SoilLayer>, soil_params: Vec<SoilParams>) -> Self {
        GroundModel {
            soil_layers,
            soil_params,
            rigid_boundary: None,
            groundwater: 0.0,
            reference: String::new(),
        }
    }

    pub fn from_agsi_file(agsi_json: &serde_json::Value) -> Self {
        let mut soil_params = Vec::new();

        if let Some(agsi_models) = agsi_json["agsiModel"].as_array() {
            if let Some(first_model) = agsi_models.get(0) {
                if let Some(elements) = first_model["agsiModelElement"].as_array() {
                    for element in elements {
                        if let Some(param_values) = element["agsiDataParameterValue"].as_array() {
                            let params_data: Vec<AgsiDataParameterValue> = param_values
                                .iter()
                                .filter_map(|p| {
                                    Some(AgsiDataParameterValue {
                                        code_id: p["codeID"].as_str()?.parse().ok()?,
                                        case_id: None,
                                        data_id: None,
                                        remarks: None,
                                        value_numeric: p["valueNumeric"].as_f64(),
                                        value_profile: None,
                                        value_profile_ind_var_code_id: None,
                                        value_text: None,
                                    })
                                })
                                .collect();

                            if !params_data.is_empty() {
                                let mut soil_param =
                                    SoilParams::from_agsi_data_parameters(&params_data);
                                soil_param.reference = element["elementName"]
                                    .as_str()
                                    .unwrap_or("unknown")
                                    .to_string();
                                soil_params.push(soil_param);
                            }
                        }
                    }
                }
            }
        }

        GroundModel::new(Vec::new(), soil_params)
    }

    pub fn with_all_fields(
        soil_layers: Vec<SoilLayer>,
        soil_params: Vec<SoilParams>,
        rigid_boundary: Option<f64>,
        groundwater: f64,
        reference: String,
    ) -> Self {
        GroundModel {
            soil_layers,
            soil_params,
            rigid_boundary,
            groundwater,
            reference,
        }
    }

    pub fn get_base_level(&self) -> f64 {
        let mut base = 10000.0;
        for layer in &self.soil_layers {
            let layer_base = layer.base_level.unwrap_or(0.0);
            if base > layer_base {
                base = layer_base;
            }
        }
        base
    }

    pub fn get_top_level(&self) -> f64 {
        let mut top = -10000.0;
        for layer in &self.soil_layers {
            if top < layer.top_level {
                top = layer.top_level;
            }
        }
        top
    }

    pub fn get_soil_params(&self, reference: &str) -> Option<&SoilParams> {
        self.soil_params
            .iter()
            .find(|param| param.reference == reference)
    }

    pub fn get_params_at_level(&self, level: f64) -> Result<&SoilParams, &'static str> {
        for layer in &self.soil_layers {
            let base_level = layer.base_level.unwrap_or(0.0);
            if layer.top_level >= level && level >= base_level {
                let unit_ref = &layer.unit_reference;
                for param in &self.soil_params {
                    if param.reference == *unit_ref {
                        return Ok(param);
                    }
                }
            }
        }
        Err("Layer not present")
    }

    pub fn get_layer_at_level(&self, level: f64) -> Result<&SoilLayer, &'static str> {
        for layer in &self.soil_layers {
            let base_level = layer.base_level.unwrap_or(0.0);
            if layer.top_level >= level && level >= base_level {
                return Ok(layer);
            }
        }
        Err("Layer not present")
    }

    pub fn get_soil_params_at_level(&self, level: f64) -> Option<&SoilParams> {
        for (i, layer) in self.soil_layers.iter().enumerate() {
            let base_level = layer.base_level.unwrap_or(0.0);
            if level <= layer.top_level && level >= base_level {
                return self.soil_params.get(i);
            }
        }
        None
    }

    fn get_unit_weight_at_level(&self, level: f64) -> Result<f64, &'static str> {
        Ok(self.get_params_at_level(level)?.unit_weight)
    }

    pub fn get_pwp_at_level(&self, level: f64) -> f64 {
        if level > self.groundwater {
            0.0
        } else {
            10.0 * (self.groundwater - level)
        }
    }

    pub fn get_total_stress_at_level(&self, level: f64) -> f64 {
        let top = self.get_top_level();
        let spacing = 0.1;
        let mut result = 0.0;
        let mut current_level = level;

        while current_level <= top {
            if let Ok(unit_weight) = self.get_unit_weight_at_level(current_level) {
                result += unit_weight;
            }
            current_level += spacing;
        }

        result * spacing
    }

    pub fn get_effective_stress_at_level(&self, level: f64) -> f64 {
        self.get_total_stress_at_level(level) - self.get_pwp_at_level(level)
    }

    pub fn quick_init(soil_params: SoilParams, top_level: f64, groundwater_level: f64) -> Self {
        let reference = if soil_params.reference.is_empty() {
            "gm_soil".to_string()
        } else {
            soil_params.reference.clone()
        };

        let mut params = soil_params;
        params.reference = reference.clone();

        let layer = SoilLayer::with_all_fields(
            reference.clone(),
            top_level,
            Some(top_level - 1000.0),
            None,
            String::new(),
            String::new(),
        );

        GroundModel {
            soil_layers: vec![layer],
            soil_params: vec![params],
            rigid_boundary: None,
            groundwater: groundwater_level,
            reference,
        }
    }

    pub fn get_depth_at_level(&self, level: f64) -> Option<f64> {
        if self.soil_layers.is_empty() {
            return None;
        }
        let ground_surface = self.soil_layers[0].top_level;
        if level > ground_surface {
            return None;
        }
        Some(ground_surface - level)
    }
}

impl Default for GroundModel {
    fn default() -> Self {
        GroundModel {
            soil_layers: vec![SoilLayer::default()],
            soil_params: vec![SoilParams::default()],
            rigid_boundary: None,
            groundwater: 0.0,
            reference: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_model_creation() {
        let layers = vec![
            SoilLayer::new(10.0, 5.0, "Layer 1".to_string()),
            SoilLayer::new(5.0, 0.0, "Layer 2".to_string()),
        ];
        let params = vec![
            SoilParams::new(
                "Cohesive soil".to_string(),
                0.1,
                10000.0,
                0.3,
                1e-8,
                SoilType::Cohesive,
                18.0,
            ),
            SoilParams::new(
                "Granular soil".to_string(),
                0.0,
                50000.0,
                0.25,
                0.0,
                SoilType::Granular,
                20.0,
            ),
        ];

        let ground_model = GroundModel::new(layers, params);
        assert_eq!(ground_model.soil_layers.len(), 2);
        assert_eq!(ground_model.soil_params.len(), 2);
    }

    #[test]
    fn test_get_soil_params_at_level() {
        let layers = vec![
            SoilLayer::new(10.0, 5.0, "Layer 1".to_string()),
            SoilLayer::new(5.0, 0.0, "Layer 2".to_string()),
        ];
        let params = vec![
            SoilParams::new(
                "Cohesive soil".to_string(),
                0.1,
                10000.0,
                0.3,
                1e-8,
                SoilType::Cohesive,
                18.0,
            ),
            SoilParams::new(
                "Granular soil".to_string(),
                0.0,
                50000.0,
                0.25,
                0.0,
                SoilType::Granular,
                20.0,
            ),
        ];

        let ground_model = GroundModel::new(layers, params);

        let soil_at_7 = ground_model.get_soil_params_at_level(7.0);
        assert!(soil_at_7.is_some());
        assert_eq!(soil_at_7.unwrap().behaviour, SoilType::Cohesive);

        let soil_at_3 = ground_model.get_soil_params_at_level(3.0);
        assert!(soil_at_3.is_some());
        assert_eq!(soil_at_3.unwrap().behaviour, SoilType::Granular);

        let soil_at_15 = ground_model.get_soil_params_at_level(15.0);
        assert!(soil_at_15.is_none());
    }

    #[test]
    fn test_get_depth_at_level() {
        let layers = vec![SoilLayer::new(10.0, 5.0, "Layer 1".to_string())];
        let params = vec![SoilParams::default()];

        let ground_model = GroundModel::new(layers, params);

        assert_eq!(ground_model.get_depth_at_level(8.0), Some(2.0));
        assert_eq!(ground_model.get_depth_at_level(10.0), Some(0.0));
        assert_eq!(ground_model.get_depth_at_level(12.0), None);
    }

    #[test]
    fn test_get_base_and_top_level() {
        let layers = vec![
            SoilLayer::new(15.0, 8.0, "Layer 1".to_string()),
            SoilLayer::new(8.0, 2.0, "Layer 2".to_string()),
            SoilLayer::new(2.0, -5.0, "Layer 3".to_string()),
        ];
        let params = vec![SoilParams::default(); 3];

        let ground_model = GroundModel::new(layers, params);

        assert_eq!(ground_model.get_top_level(), 15.0);
        assert_eq!(ground_model.get_base_level(), -5.0);
    }

    #[test]
    fn test_get_soil_params_by_reference() {
        let params = vec![
            SoilParams::new(
                "clay".to_string(),
                0.1,
                10000.0,
                0.3,
                1e-8,
                SoilType::Cohesive,
                18.0,
            ),
            SoilParams::new(
                "sand".to_string(),
                0.0,
                50000.0,
                0.25,
                0.0,
                SoilType::Granular,
                20.0,
            ),
        ];

        let ground_model = GroundModel::new(vec![], params);

        let clay_params = ground_model.get_soil_params("clay");
        assert!(clay_params.is_some());
        assert_eq!(clay_params.unwrap().behaviour, SoilType::Cohesive);

        let sand_params = ground_model.get_soil_params("sand");
        assert!(sand_params.is_some());
        assert_eq!(sand_params.unwrap().behaviour, SoilType::Granular);

        let unknown_params = ground_model.get_soil_params("rock");
        assert!(unknown_params.is_none());
    }

    #[test]
    fn test_get_params_at_level_with_unit_reference() {
        let layers = vec![
            SoilLayer::with_unit_reference(10.0, 5.0, "Layer 1".to_string(), "clay".to_string()),
            SoilLayer::with_unit_reference(5.0, 0.0, "Layer 2".to_string(), "sand".to_string()),
        ];
        let params = vec![
            SoilParams::new(
                "clay".to_string(),
                0.1,
                10000.0,
                0.3,
                1e-8,
                SoilType::Cohesive,
                18.0,
            ),
            SoilParams::new(
                "sand".to_string(),
                0.0,
                50000.0,
                0.25,
                0.0,
                SoilType::Granular,
                20.0,
            ),
        ];

        let ground_model = GroundModel::new(layers, params);

        let params_at_7 = ground_model.get_params_at_level(7.0);
        assert!(params_at_7.is_ok());
        assert_eq!(params_at_7.unwrap().behaviour, SoilType::Cohesive);

        let params_at_3 = ground_model.get_params_at_level(3.0);
        assert!(params_at_3.is_ok());
        assert_eq!(params_at_3.unwrap().behaviour, SoilType::Granular);
    }

    #[test]
    fn test_pwp_calculation() {
        let ground_model = GroundModel::with_all_fields(
            vec![],
            vec![],
            None,
            5.0, // groundwater at 5.0 mAOD
            "test".to_string(),
        );

        // Above groundwater
        assert_eq!(ground_model.get_pwp_at_level(6.0), 0.0);
        assert_eq!(ground_model.get_pwp_at_level(5.0), 0.0);

        // Below groundwater
        assert_eq!(ground_model.get_pwp_at_level(4.0), 10.0);
        assert_eq!(ground_model.get_pwp_at_level(3.0), 20.0);
    }

    #[test]
    fn test_quick_init() {
        let soil_params = SoilParams::new(
            "test_soil".to_string(),
            0.1,
            10000.0,
            0.3,
            1e-8,
            SoilType::Cohesive,
            18.0,
        );

        let ground_model = GroundModel::quick_init(soil_params, 10.0, 5.0);

        assert_eq!(ground_model.soil_layers.len(), 1);
        assert_eq!(ground_model.soil_params.len(), 1);
        assert_eq!(ground_model.groundwater, 5.0);
        assert_eq!(ground_model.soil_layers[0].top_level, 10.0);
        assert_eq!(ground_model.soil_layers[0].base_level, Some(-990.0));
        assert_eq!(ground_model.soil_params[0].reference, "test_soil");
    }

    #[test]
    fn test_quick_init_with_empty_reference() {
        let soil_params = SoilParams::default(); // Empty reference

        let ground_model = GroundModel::quick_init(soil_params, 10.0, 5.0);

        assert_eq!(ground_model.soil_params[0].reference, "gm_soil");
        assert_eq!(ground_model.reference, "gm_soil");
    }

    #[test]
    fn test_layer_at_level() {
        let layers = vec![
            SoilLayer::new(10.0, 5.0, "Upper layer".to_string()),
            SoilLayer::new(5.0, 0.0, "Lower layer".to_string()),
        ];

        let ground_model = GroundModel::new(layers, vec![]);

        let layer_at_7 = ground_model.get_layer_at_level(7.0);
        assert!(layer_at_7.is_ok());
        assert_eq!(layer_at_7.unwrap().reference, "Upper layer");

        let layer_at_3 = ground_model.get_layer_at_level(3.0);
        assert!(layer_at_3.is_ok());
        assert_eq!(layer_at_3.unwrap().reference, "Lower layer");

        let layer_at_15 = ground_model.get_layer_at_level(15.0);
        assert!(layer_at_15.is_err());
    }

    #[test]
    fn test_soil_layer_new_structure() {
        let layer = SoilLayer::with_all_fields(
            "CLAY".to_string(),
            10.0,
            Some(5.0),
            Some("SAND".to_string()),
            "Firm brown clay".to_string(),
            "CL".to_string(),
        );

        assert_eq!(layer.unit_reference, "CLAY");
        assert_eq!(layer.top_level, 10.0);
        assert_eq!(layer.base_level, Some(5.0));
        assert_eq!(layer.base_unit_reference, Some("SAND".to_string()));
        assert_eq!(layer.typical_description, "Firm brown clay");
        assert_eq!(layer.geol_code, "CL");
    }

    #[test]
    fn test_excavate_layer() {
        let layer = SoilLayer::with_all_fields(
            "CLAY".to_string(),
            10.0,
            Some(5.0),
            None,
            "Firm brown clay".to_string(),
            "CL".to_string(),
        );

        // Excavation cuts through the layer
        let (excavated, should_delete) = layer.excavate_layer(7.0);
        assert!(!should_delete);
        assert_eq!(excavated.top_level, 7.0);
        assert_eq!(excavated.base_level, Some(5.0));

        // Excavation above the layer
        let (excavated, should_delete) = layer.excavate_layer(12.0);
        assert!(!should_delete);
        assert_eq!(excavated.top_level, 10.0);

        // Excavation removes the entire layer
        let (excavated, should_delete) = layer.excavate_layer(3.0);
        assert!(should_delete);
        assert_eq!(excavated.geol_code, "DELETE");
    }

    #[test]
    fn test_process_layer() {
        let layer = SoilLayer::with_all_fields(
            "ROCK".to_string(),
            10.0,
            Some(5.0),
            None,
            "Weathered limestone".to_string(),
            "LS".to_string(),
        );

        let processed = layer.process_layer(100.0, SoilType::Rock);
        assert!(processed
            .typical_description
            .contains("processed with sig3: 100"));

        let processed_soil = layer.process_layer(0.0, SoilType::Cohesive);
        assert_eq!(processed_soil.typical_description, "Weathered limestone");
    }

    #[test]
    fn test_updated_ground_model_with_optional_base_level() {
        let layers = vec![
            SoilLayer::with_all_fields(
                "clay".to_string(),
                10.0,
                Some(5.0),
                None,
                "Firm brown clay".to_string(),
                "CL".to_string(),
            ),
            SoilLayer::with_all_fields(
                "sand".to_string(),
                5.0,
                None, // No base level specified
                None,
                "Dense sand".to_string(),
                "SP".to_string(),
            ),
        ];
        let params = vec![
            SoilParams::new(
                "clay".to_string(),
                0.1,
                10000.0,
                0.3,
                1e-8,
                SoilType::Cohesive,
                18.0,
            ),
            SoilParams::new(
                "sand".to_string(),
                0.0,
                50000.0,
                0.25,
                0.0,
                SoilType::Granular,
                20.0,
            ),
        ];

        let ground_model = GroundModel::new(layers, params);

        // Test with layer that has base_level
        let params_at_7 = ground_model.get_params_at_level(7.0);
        assert!(params_at_7.is_ok());
        assert_eq!(params_at_7.unwrap().behaviour, SoilType::Cohesive);

        // Test with layer that has no base_level (should default to 0.0)
        let params_at_3 = ground_model.get_params_at_level(3.0);
        assert!(params_at_3.is_ok());
        assert_eq!(params_at_3.unwrap().behaviour, SoilType::Granular);
    }

    #[test]
    fn test_soil_layer_default() {
        let layer = SoilLayer::default();
        assert_eq!(layer.unit_reference, "");
        assert_eq!(layer.top_level, 100.0);
        assert_eq!(layer.base_level, Some(0.0));
        assert_eq!(layer.base_unit_reference, None);
        assert_eq!(layer.typical_description, "");
        assert_eq!(layer.geol_code, "");
    }

    #[test]
    fn test_soil_params_new_structure() {
        let params = SoilParams::with_all_fields(
            "test_soil".to_string(),
            SoilType::Cohesive,
            Some(0.5),  // phi_prime in radians
            Some(10.0), // c_prime in kPa
            18.0,       // unit_weight
            Some(50.0), // cu
            0.1,        // mv
            10000.0,    // youngs_modulus
            0.3,        // poissons_ratio
            1e-8,       // coefficient_of_consolidation
            Some(65.0), // gsi
            Some(25.0), // ucs
            Some(10.0), // mi
            0.0,        // disturbance
        );

        assert_eq!(params.reference, "test_soil");
        assert_eq!(params.behaviour, SoilType::Cohesive);
        assert_eq!(params.phi_prime, Some(0.5));
        assert_eq!(params.c_prime, Some(10.0));
        assert_eq!(params.unit_weight, 18.0);
        assert_eq!(params.cu, Some(50.0));
        assert_eq!(params.gsi, Some(65.0));
        assert_eq!(params.ucs, Some(25.0));
        assert_eq!(params.mi, Some(10.0));
        assert!(!params.factored);
    }

    #[test]
    fn test_partial_factors() {
        let pf = PartialFactors::new(1.25, 1.25, 1.1, 1.4); // Changed gamma_gamma to 1.1
        let params = SoilParams::with_all_fields(
            "test".to_string(),
            SoilType::Cohesive,
            Some(0.5), // ~28.6 degrees
            Some(20.0),
            18.0,
            Some(100.0),
            0.1,
            10000.0,
            0.3,
            1e-8,
            None,
            None,
            None,
            0.0,
        );

        let factored = params.apply_partial_factors(&pf);
        assert!(factored.factored);
        assert!(factored.factors.is_some());

        // Check that values are reduced
        assert!(factored.phi_prime.unwrap() < params.phi_prime.unwrap());
        assert!(factored.c_prime.unwrap() < params.c_prime.unwrap());
        assert!(factored.unit_weight < params.unit_weight); // Now this should work
        assert!(factored.cu.unwrap() < params.cu.unwrap());

        // Test removing factors
        let unfactored = factored.remove_partial_factors().unwrap();
        assert!(!unfactored.factored);
        assert!(unfactored.factors.is_none());

        // Values should be approximately back to original (within floating point precision)
        let phi_diff = (unfactored.phi_prime.unwrap() - params.phi_prime.unwrap()).abs();
        assert!(phi_diff < 1e-10);
    }

    #[test]
    fn test_earth_pressure_coefficients() {
        let params = SoilParams::with_all_fields(
            "test".to_string(),
            SoilType::Granular,
            Some(30.0_f64.to_radians()), // 30 degrees
            Some(0.0),
            20.0,
            None,
            0.0,
            50000.0,
            0.25,
            0.0,
            None,
            None,
            None,
            0.0,
        );

        // Test K_active for 30 degree friction angle
        let k_active = params.get_k_active(None).unwrap();
        let expected_ka = (1.0 - 30.0_f64.to_radians().sin()) / (1.0 + 30.0_f64.to_radians().sin());
        assert!((k_active - expected_ka).abs() < 1e-10);

        // Test K_passive
        let k_passive = params.get_k_passive(None).unwrap();
        let expected_kp = 1.0 / expected_ka;
        assert!((k_passive - expected_kp).abs() < 1e-10);

        // Test K0
        let k0 = params.k0().unwrap();
        let expected_k0 = 1.0 - 30.0_f64.to_radians().sin();
        assert!((k0 - expected_k0).abs() < 1e-10);
    }

    #[test]
    fn test_hoek_brown_parameters() {
        let params = SoilParams::with_all_fields(
            "rock".to_string(),
            SoilType::Rock,
            None,
            None,
            25.0,
            None,
            0.0,
            0.0,
            0.25,
            0.0,
            Some(65.0), // gsi
            Some(25.0), // ucs in MPa
            Some(10.0), // mi
            0.0,        // disturbance
        );

        // Test mb calculation
        let mb = params.mb().unwrap();
        assert!(mb > 0.0);

        // Test s calculation
        let s = params.s().unwrap();
        assert!(s > 0.0);

        // Test a calculation
        let a = params.a().unwrap();
        assert!(a > 0.0 && a < 1.0);

        // Test rock E value
        let e_val = params.rock_e_val().unwrap();
        assert!(e_val > 0.0);
    }

    #[test]
    fn test_hoek_brown_conversion() {
        let params = SoilParams::with_all_fields(
            "rock".to_string(),
            SoilType::Rock,
            None,
            None,
            25.0,
            None,
            0.0,
            0.0,
            0.25,
            0.0,
            Some(65.0), // gsi
            Some(25.0), // ucs in MPa
            Some(10.0), // mi
            0.0,        // disturbance
        );

        let sig3 = 100.0; // confining stress in kPa

        // Test equivalent phi angle
        let phi_equiv = params.hb_equiv_phi_ang(sig3).unwrap();
        assert!(phi_equiv >= 0.0 && phi_equiv <= 1.0); // Should be reasonable phi value in radians

        // Test equivalent cohesion
        let c_equiv = params.hb_equiv_c_prime(sig3).unwrap();
        assert!(c_equiv > 0.0);

        // Test full conversion
        let converted = params.convert_equivalent_rock(sig3).unwrap();
        assert!(converted.phi_prime.is_some());
        assert!(converted.c_prime.is_some());
        assert!(converted.phi_prime.unwrap() > 0.0);
        assert!(converted.c_prime.unwrap() > 0.0);
        assert_eq!(converted.unit_weight, params.unit_weight);
    }

    #[test]
    fn test_error_handling() {
        let params = SoilParams::default();

        // Test that methods fail appropriately when required values are missing
        assert!(params.get_k_active(None).is_err());
        assert!(params.k0().is_err());
        assert!(params.mb().is_err());
        assert!(params.s().is_err());
        assert!(params.a().is_err());
        assert!(params.rock_e_val().is_err());
        assert!(params.hb_equiv_phi_ang(100.0).is_err());
        assert!(params.hb_equiv_c_prime(100.0).is_err());
        assert!(params.convert_equivalent_rock(100.0).is_err());

        // Test removing factors when none are present
        assert!(params.remove_partial_factors().is_err());
    }

    #[test]
    fn test_from_agsi_data_parameters() {
        let data = vec![
            AgsiDataParameterValue {
                code_id: "UnitWeight".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(18.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "EffectiveCohesion".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(5.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "EffectiveFrictionAngle".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(30.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "YoungsModulus".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(50000.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
        ];

        let soil_params = SoilParams::from_agsi_data_parameters(&data);

        assert_eq!(soil_params.unit_weight, 18.0);
        assert_eq!(soil_params.phi_prime, Some(30.0));
        assert_eq!(soil_params.c_prime, Some(5.0));
        assert_eq!(soil_params.youngs_modulus, 50000.0);
        assert_eq!(soil_params.behaviour, SoilType::Granular); // Default behavior
    }

    #[test]
    fn test_from_agsi_data_parameters_cohesive() {
        let data = vec![AgsiDataParameterValue {
            code_id: "UndrainedShearStrength".parse().unwrap(),
            case_id: None,
            data_id: None,
            remarks: None,
            value_numeric: Some(100.0),
            value_profile: None,
            value_profile_ind_var_code_id: None,
            value_text: None,
        }];

        let soil_params = SoilParams::from_agsi_data_parameters(&data);

        assert_eq!(soil_params.cu, Some(100.0));
        assert_eq!(soil_params.behaviour, SoilType::Cohesive);
    }

    #[test]
    fn test_from_agsi_data_parameters_rock() {
        let data = vec![
            AgsiDataParameterValue {
                code_id: "UnconfinedCompressiveStrength".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(25.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "GeologicalStrengthIndex".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(65.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "HoekBrownParamMi".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(10.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
        ];

        let soil_params = SoilParams::from_agsi_data_parameters(&data);

        assert_eq!(soil_params.ucs, Some(25.0));
        assert_eq!(soil_params.gsi, Some(65.0));
        assert_eq!(soil_params.mi, Some(10.0));
        assert_eq!(soil_params.behaviour, SoilType::Rock);
    }

    #[test]
    fn test_from_agsi_data_parameters_advanced_params() {
        let data = vec![
            AgsiDataParameterValue {
                code_id: "CustomParameter".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(42.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
            AgsiDataParameterValue {
                code_id: "AnotherCustom".parse().unwrap(),
                case_id: None,
                data_id: None,
                remarks: None,
                value_numeric: Some(123.0),
                value_profile: None,
                value_profile_ind_var_code_id: None,
                value_text: None,
            },
        ];

        let soil_params = SoilParams::from_agsi_data_parameters(&data);

        assert!(soil_params.advanced_parameters.is_some());
        let advanced = soil_params.advanced_parameters.unwrap();
        assert_eq!(advanced.len(), 2);
        assert_eq!(advanced[0].name, "CustomParameter");
        assert_eq!(advanced[0].value, 42.0);
        assert_eq!(advanced[1].name, "AnotherCustom");
        assert_eq!(advanced[1].value, 123.0);
    }
}
