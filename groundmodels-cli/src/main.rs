use clap::{Parser, Subcommand};
use groundmodels_core::{ConvertType, SoilParams, GroundModel, SoilType};
use groundmodels_core::soil_description::{
    parse_soil_description, validate_soil_description, generate_description,
    ValidationOptions as DescValidationOptions, SoilDescription,
    StrengthParameterType as DescStrengthType,
};
use groundmodels_core::strip_log::{BuildStripLogOptions, StripLogRenderOptions};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use jsonschema::Validator;
use dialoguer::{Input, Select, MultiSelect, Confirm};
use console::Style;
use tower_lsp::{LspService, Server};
use tower_lsp::lsp_types::*;
use tokio::net::TcpListener;
use tabled::{Table as TabledTable, Tabled};
use docx_rs::*;

mod lsp;

#[derive(Parser)]
#[command(name = "groundmodels")]
#[command(about = "A CLI tool for geotechnical ground modeling and soil parameter analysis")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start AGSi Language Server Protocol server
    Lsp {
        /// Port to listen on (default: 3000)
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Validate AGSi JSON against schema
    Validate {
        /// Input AGSi JSON file to validate
        #[arg(short, long)]
        input: PathBuf,
        /// AGSi schema file (optional, uses embedded schema if not provided)
        #[arg(short, long)]
        schema: Option<PathBuf>,
    },
    /// Interactive AGSi JSON file generation
    Generate {
        /// Output file path for generated AGSi JSON
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Convert AGSi file to formatted tables
    Table {
        /// Input AGSi JSON file
        input: PathBuf,
        /// Export tables to Microsoft Word (.docx) file
        #[arg(long)]
        word: Option<PathBuf>,
    },
    /// Convert AGSi JSON files to different formats
    Convert {
        /// Input AGSi JSON file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        
        /// Conversion type
        #[arg(short, long, value_enum)]
        convert_type: CliConvertType,
    },
    /// Add predefined materials to AGSi files
    AddMaterials {
        /// Input AGSi JSON file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        
        /// Material type to add
        #[arg(short, long)]
        material: String,
    },
    /// Analyze soil parameters from AGSi data
    Analyze {
        /// Input AGSi JSON file
        #[arg(short, long)]
        input: PathBuf,
    },
    /// Generate example AGSi data
    Example {
        /// Output file path for example data
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Parse soil description text into structured fields
    Describe {
        /// Input text (single description)
        #[arg(short, long)]
        text: Option<String>,
        /// Input file with one description per line
        #[arg(short, long)]
        input: Option<PathBuf>,
        /// Treat warnings as errors
        #[arg(long, default_value_t = false)]
        strict: bool,
        /// Require strength parameters
        #[arg(long, default_value_t = false)]
        require_strength: bool,
        /// Require primary type
        #[arg(long, default_value_t = false)]
        require_primary: bool,
        /// Disable correlation warnings
        #[arg(long, default_value_t = false)]
        no_correlations: bool,
    },
    /// Suggest soil parameters from description
    ParamsFromDescription {
        /// Input text (single description)
        #[arg(short, long)]
        text: Option<String>,
        /// Input file with one description per line
        #[arg(short, long)]
        input: Option<PathBuf>,
    },
    /// Export strip log outputs from a GroundModel JSON
    StripLog {
        /// Input GroundModel JSON file
        #[arg(short, long)]
        input: PathBuf,
        /// Output SVG path
        #[arg(long)]
        svg: Option<PathBuf>,
        /// Output CSV path
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Output AGS GEOL CSV path
        #[arg(long)]
        ags_geol: Option<PathBuf>,
        /// Hole ID for AGS GEOL CSV
        #[arg(long, default_value = "BH1")]
        hole_id: String,
        /// Include stress calculations
        #[arg(long, default_value_t = false)]
        include_stresses: bool,
        /// Stress integration dz (m)
        #[arg(long, default_value_t = 0.05)]
        dz: f64,
        /// Strip log title
        #[arg(long)]
        title: Option<String>,
        /// Axis unit label (right axis)
        #[arg(long, default_value = "m")]
        axis_unit: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum CliConvertType {
    SoilParams,
    GroundModel,
}

impl From<CliConvertType> for ConvertType {
    fn from(cli_type: CliConvertType) -> Self {
        match cli_type {
            CliConvertType::SoilParams => ConvertType::SoilParams,
            CliConvertType::GroundModel => ConvertType::GroundModel,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Lsp { port } => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(start_lsp_server(port))?;
        }

        Commands::Validate { input, schema } => {
            validate_agsi_file(&input, schema.as_ref())?;
        }

        Commands::Generate { output } => {
            generate_interactive_agsi(&output)?;
        }

        Commands::Table { input, word } => {
            handle_table_command(&input, word.as_ref())?;
        }

        Commands::Convert { input, output, convert_type } => {
            let input_content = fs::read_to_string(&input)?;
            let agsi_data: serde_json::Value = serde_json::from_str(&input_content)?;
            
            let result = match convert_type.into() {
                ConvertType::SoilParams => {
                    // Extract parameter data from AGSi and create SoilParams
                    let ground_model = GroundModel::from_agsi_file(&agsi_data);
                    let soil_params = if let Some(first_params) = ground_model.soil_params.first() {
                        first_params.clone()
                    } else {
                        SoilParams::default()
                    };
                    serde_json::to_string_pretty(&soil_params)?
                }
                ConvertType::GroundModel => {
                    let ground_model = GroundModel::from_agsi_file(&agsi_data);
                    serde_json::to_string_pretty(&ground_model)?
                }
            };
            
            fs::write(&output, result)?;
            println!("Conversion complete! Output written to: {}", output.display());
        }

        Commands::AddMaterials { input, output, material } => {
            add_predefined_materials(&input, &output, &material)?;
        }
        
        Commands::Analyze { input } => {
            let input_content = fs::read_to_string(&input)?;
            let agsi_data: serde_json::Value = serde_json::from_str(&input_content)?;
            
            let ground_model = GroundModel::from_agsi_file(&agsi_data);
            
            println!("Ground Model Analysis:");
            println!("=====================");
            println!("Number of soil parameters: {}", ground_model.soil_params.len());
            
            for (i, params) in ground_model.soil_params.iter().enumerate() {
                println!("\nSoil Parameter Set {}:", i + 1);
                println!("  Unit Weight: {:.2} kN/m³", params.unit_weight);
                println!("  Young's Modulus: {:.0} kPa", params.youngs_modulus);
                println!("  Behaviour: {:?}", params.behaviour);
                
                if let Some(phi) = params.phi_prime {
                    println!("  Friction Angle: {:.1}°", phi);
                }
                if let Some(c) = params.c_prime {
                    println!("  Cohesion: {:.1} kPa", c);
                }
                if let Some(cu) = params.cu {
                    println!("  Undrained Shear Strength: {:.1} kPa", cu);
                }
            }
        }
        
        Commands::Example { output } => {
            // Create example AGSi data structure
            let example_data = serde_json::json!({
                "agsFile": {
                    "producedBy": "Example Organization",
                    "title": "Example Ground Model"
                },
                "agsSchema": {
                    "version": "1.0.1"
                },
                "agsiModel": [{
                    "agsiModelElement": [{
                        "agsiDataParameterValue": [
                            {
                                "codeID": "UnitWeight",
                                "valueNumeric": 18.0
                            },
                            {
                                "codeID": "EffectiveFrictionAngle", 
                                "valueNumeric": 30.0
                            },
                            {
                                "codeID": "EffectiveCohesion",
                                "valueNumeric": 5.0
                            },
                            {
                                "codeID": "YoungsModulus",
                                "valueNumeric": 50000.0
                            }
                        ]
                    }]
                }]
            });
            
            let example_json = serde_json::to_string_pretty(&example_data)?;
            fs::write(&output, example_json)?;
            println!("Example AGSi data written to: {}", output.display());
        }
        Commands::Describe {
            text,
            input,
            strict,
            require_strength,
            require_primary,
            no_correlations,
        } => {
            let descriptions = collect_descriptions(text.as_deref(), input.as_ref())?;
            let opts = DescValidationOptions {
                strict,
                require_strength_params: require_strength,
                require_primary_type: require_primary,
                check_correlations: !no_correlations,
            };

            #[derive(serde::Serialize)]
            struct DescriptionOutput {
                input: String,
                description: SoilDescription,
                validation: groundmodels_core::soil_description::ValidationResult,
                generated: String,
            }

            let mut outputs = Vec::new();
            for desc in descriptions {
                let parsed = parse_soil_description(&desc);
                let validation = validate_soil_description(&parsed, opts);
                let generated = generate_description(&parsed);
                outputs.push(DescriptionOutput {
                    input: desc,
                    description: parsed,
                    validation,
                    generated,
                });
            }

            if outputs.len() == 1 {
                let json = serde_json::to_string_pretty(&outputs[0])?;
                println!("{}", json);
            } else {
                let json = serde_json::to_string_pretty(&outputs)?;
                println!("{}", json);
            }
        }
        Commands::ParamsFromDescription { text, input } => {
            let descriptions = collect_descriptions(text.as_deref(), input.as_ref())?;

            #[derive(serde::Serialize)]
            struct InferredRange {
                parameter_type: String,
                lower: f64,
                upper: f64,
                typical: f64,
                units: String,
            }

            #[derive(serde::Serialize)]
            struct ParamsOutput {
                input: String,
                description: SoilDescription,
                suggested_params: SoilParams,
                inferred_ranges: Vec<InferredRange>,
            }

            let mut outputs = Vec::new();
            for desc in descriptions {
                let parsed = parse_soil_description(&desc);
                let mut params = SoilParams::default();
                if let Some(soil) = parsed.primary_soil_type {
                    params.reference = format!("{:?}", soil).to_lowercase();
                } else if let Some(rock) = parsed.primary_rock_type {
                    params.reference = format!("{:?}", rock).to_lowercase();
                } else {
                    params.reference = "inferred".to_string();
                }

                if parsed.material_type == Some(groundmodels_core::soil_description::MaterialType::Rock) {
                    params.behaviour = SoilType::Rock;
                } else if let Some(soil) = parsed.primary_soil_type {
                    params.behaviour = if groundmodels_core::soil_description::is_cohesive(soil) {
                        SoilType::Cohesive
                    } else {
                        SoilType::Granular
                    };
                }

                let mut ranges = Vec::new();
                for sp in &parsed.strength_parameters {
                    let inferred = InferredRange {
                        parameter_type: format!("{:?}", sp.parameter_type),
                        lower: sp.value_range.lower_bound,
                        upper: sp.value_range.upper_bound,
                        typical: sp.value_range.typical_value,
                        units: sp.units.clone(),
                    };
                    ranges.push(inferred);

                    match sp.parameter_type {
                        DescStrengthType::UndrainedShear => {
                            params.cu = Some(sp.value_range.typical_value);
                        }
                        DescStrengthType::Ucs => {
                            params.ucs = Some(sp.value_range.typical_value);
                            params.behaviour = SoilType::Rock;
                        }
                        _ => {}
                    }
                }

                outputs.push(ParamsOutput {
                    input: desc,
                    description: parsed,
                    suggested_params: params,
                    inferred_ranges: ranges,
                });
            }

            if outputs.len() == 1 {
                let json = serde_json::to_string_pretty(&outputs[0])?;
                println!("{}", json);
            } else {
                let json = serde_json::to_string_pretty(&outputs)?;
                println!("{}", json);
            }
        }
        Commands::StripLog {
            input,
            svg,
            csv,
            ags_geol,
            hole_id,
            include_stresses,
            dz,
            title,
            axis_unit,
        } => {
            let input_content = fs::read_to_string(&input)?;
            let ground_model: GroundModel = serde_json::from_str(&input_content)?;

            let rows = ground_model.to_strip_log(BuildStripLogOptions {
                include_stresses,
                stress_dz: dz,
            });

            if let Some(csv_path) = csv {
                let content = ground_model.to_strip_log_csv(Some(rows.clone()));
                fs::write(&csv_path, content)?;
                println!("CSV written to: {}", csv_path.display());
            }

            if let Some(ags_path) = ags_geol {
                let content = ground_model.to_ags_geol_csv(&hole_id, Some(rows.clone()));
                fs::write(&ags_path, content)?;
                println!("AGS GEOL CSV written to: {}", ags_path.display());
            }

            if let Some(svg_path) = svg {
                let svg_content = ground_model.render_strip_log_svg(StripLogRenderOptions {
                    title,
                    axis_unit_label: axis_unit,
                    ..Default::default()
                });
                fs::write(&svg_path, svg_content)?;
                println!("SVG written to: {}", svg_path.display());
            }
        }
    }

    Ok(())
}

fn collect_descriptions(
    text: Option<&str>,
    input: Option<&PathBuf>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut descriptions = Vec::new();

    if let Some(t) = text {
        if !t.trim().is_empty() {
            descriptions.push(t.to_string());
        }
    }

    if let Some(path) = input {
        let content = fs::read_to_string(path)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                descriptions.push(trimmed.to_string());
            }
        }
    }

    if descriptions.is_empty() {
        return Err("Provide --text or --input with at least one description".into());
    }

    Ok(descriptions)
}

async fn start_lsp_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("AGSi Language Server listening on port {}", port);

    loop {
        let (stream, _) = listener.accept().await?;
        let (read, write) = tokio::io::split(stream);
        let (service, socket) = LspService::new(|client| lsp::Backend::new(client));
        let server = Server::new(read, write, socket);
        tokio::spawn(server.serve(service));
    }
}

fn validate_agsi_file(input: &PathBuf, schema_path: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let input_content = fs::read_to_string(input)?;
    let agsi_data: serde_json::Value = serde_json::from_str(&input_content)?;

    let schema_content = if let Some(schema_file) = schema_path {
        fs::read_to_string(schema_file)?
    } else {
        // Use embedded AGSi schema
        include_str!("../../groundmodels-core/src/AGSi_JSONSchema_v1-0-1_2020-12.json").to_string()
    };

    let schema: serde_json::Value = serde_json::from_str(&schema_content)?;
    let validator = Validator::new(&schema)?;

    if let Err(error) = validator.validate(&agsi_data) {
        println!("✗ AGSi file validation failed:");
        println!("  - {}", error);
        return Err("Validation failed".into());
    } else {
        println!("✓ AGSi file is valid according to schema");
    }

    Ok(())
}

fn generate_interactive_agsi(output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let green = Style::new().green();
    let blue = Style::new().blue();
    
    println!("{}", green.apply_to("🌍 Interactive AGSi JSON Generator"));
    println!("Guided tour to create a minimal AGSi JSON file (industry standard) with parameters and/or layers\n");

    // Prompt for required agsSchema fields
    let schema_name: String = Input::new()
        .with_prompt("Schema name (e.g. AGSi)")
        .default("AGSi".to_string())
        .interact_text()?;

    let schema_version: String = Input::new()
        .with_prompt("Schema version (e.g. 1.0.1)")
        .default("1.0.1".to_string())
        .interact_text()?;

    // Prompt for required agsFile fields
    let title: String = Input::new()
        .with_prompt("File title")
        .interact_text()?;

    let produced_by: String = Input::new()
        .with_prompt("Produced by (organisation)")
        .interact_text()?;

    // Ask what to create
    let create_options = vec!["Layers", "Parameters", "Add groundwater level"];
    let create_selections = MultiSelect::new()
        .with_prompt("What do you want to create?")
        .items(&create_options)
        .interact()?;

    if create_selections.is_empty() {
        return Err("Select at least one option.".into());
    }

    let create_layers = create_selections.contains(&0);
    let create_parameters = create_selections.contains(&1);
    let create_groundwater = create_selections.contains(&2);

    // Prepare AGSi structure
    let mut agsi_observation_columns = Vec::new();
    let mut agsi_model_elements = Vec::new();

    // If layers selected, prompt for layers
    if create_layers {
        let mut add_layer = true;
        let mut last_bottom_elevation: Option<f64> = None;
        let mut hole_top_elevation: Option<f64> = None;

        while add_layer {
            println!("\n{}", blue.apply_to("--- Add Layer ---"));

            let top_elevation: f64 = Input::new()
                .with_prompt("Top Elevation")
                .default(last_bottom_elevation.unwrap_or(100.0))
                .interact_text()?;

            let bottom_elevation: f64 = Input::new()
                .with_prompt("Bottom Elevation")
                .interact_text()?;

            let geology_code: String = Input::new()
                .with_prompt("Geology Code (ElementID)")
                .interact_text()?;

            // Set hole top elevation on first entry
            if hole_top_elevation.is_none() {
                hole_top_elevation = Some(top_elevation);
            }

            // Calculate depths
            let hole_top = hole_top_elevation.unwrap();
            let top_depth = hole_top - top_elevation;
            let bottom_depth = hole_top - bottom_elevation;

            agsi_observation_columns.push(serde_json::json!({
                "topDepth": top_depth,
                "bottomDepth": bottom_depth,
                "topElevation": top_elevation,
                "bottomElevation": bottom_elevation,
                "geologyCode": geology_code
            }));

            last_bottom_elevation = Some(bottom_elevation);

            add_layer = Confirm::new()
                .with_prompt("Add another layer?")
                .default(false)
                .interact()?;
        }
    }

    // If parameters selected, prompt for parameters
    if create_parameters {
        let mut add_element = true;

        while add_element {
            println!("\n{}", blue.apply_to("--- Add Model Element ---"));

            let model_id: String = Input::new()
                .with_prompt("Model ID (optional)")
                .default("".to_string())
                .interact_text()?;

            let geol_code: String = Input::new()
                .with_prompt("GeolCode (Element ID)")
                .default("".to_string())
                .interact_text()?;

            let element_name: String = Input::new()
                .with_prompt("Element name/description (optional)")
                .default("".to_string())
                .interact_text()?;

            let material_types = vec!["Soil", "Rock"];
            let material_type_idx = Select::new()
                .with_prompt("Is this element Soil or Rock?")
                .items(&material_types)
                .default(0)
                .interact()?;

            let mut agsi_data_parameter_value = Vec::new();

            if material_type_idx == 0 {
                // Soil parameters
                let gamma: f64 = Input::new()
                    .with_prompt("Unit Weight (kN/m³)")
                    .default(18.0)
                    .interact_text()?;

                let phi_prime_deg: f64 = Input::new()
                    .with_prompt("Angle of Friction (degrees)")
                    .default(30.0)
                    .interact_text()?;

                let c_prime: f64 = Input::new()
                    .with_prompt("Cohesion (kPa)")
                    .default(5.0)
                    .interact_text()?;

                let cu: f64 = Input::new()
                    .with_prompt("Undrained Shear Strength (kPa)")
                    .default(0.0)
                    .interact_text()?;

                let youngs_modulus_mpa: f64 = Input::new()
                    .with_prompt("Young's Modulus (MPa)")
                    .default(50.0)
                    .interact_text()?;

                let mv_per_mpa: f64 = Input::new()
                    .with_prompt("Modulus Of Volume Compressibility (1/MPa)")
                    .default(0.0)
                    .interact_text()?;

                let poissons_ratio: f64 = Input::new()
                    .with_prompt("Poisson's Ratio (0.2–0.5 typical)")
                    .default(0.3)
                    .interact_text()?;

                // Convert degrees to radians for storage
                let phi_prime = phi_prime_deg.to_radians();
                let youngs_modulus = youngs_modulus_mpa * 1000.0; // Convert MPa to kPa
                let mv = mv_per_mpa / 1000.0; // Convert 1/MPa to 1/kPa

                agsi_data_parameter_value = vec![
                    serde_json::json!({"codeID": "UnitWeight", "valueNumeric": gamma}),
                    serde_json::json!({"codeID": "AngleFriction", "valueNumeric": phi_prime}),
                    serde_json::json!({"codeID": "Cohesion", "valueNumeric": c_prime}),
                    serde_json::json!({"codeID": "UndrainedShearStrength", "valueNumeric": cu}),
                    serde_json::json!({"codeID": "YoungsModulus", "valueNumeric": youngs_modulus}),
                    serde_json::json!({"codeID": "ModulusOfVolumeCompressibility", "valueNumeric": mv}),
                    serde_json::json!({"codeID": "PoissonsRatio", "valueNumeric": poissons_ratio}),
                ];
            } else {
                // Rock parameters
                let unit_weight: f64 = Input::new()
                    .with_prompt("Unit Weight (kN/m³)")
                    .default(25.0)
                    .interact_text()?;

                let gsi: f64 = Input::new()
                    .with_prompt("Geological Strength Index (GSI)")
                    .default(50.0)
                    .interact_text()?;

                let ucs: f64 = Input::new()
                    .with_prompt("Unconfined Compressive Strength (UCS, MPa)")
                    .default(50.0)
                    .interact_text()?;

                let mi: f64 = Input::new()
                    .with_prompt("Hoek-Brown parameter mi")
                    .default(10.0)
                    .interact_text()?;

                let d: f64 = Input::new()
                    .with_prompt("Disturbance factor D (0-1)")
                    .default(0.0)
                    .interact_text()?;

                agsi_data_parameter_value = vec![
                    serde_json::json!({"codeID": "UnitWeight", "valueNumeric": unit_weight}),
                    serde_json::json!({"codeID": "GeologicalStrengthIndex", "valueNumeric": gsi}),
                    serde_json::json!({"codeID": "UnconfinedCompressiveStrength", "valueNumeric": ucs}),
                    serde_json::json!({"codeID": "HoekBrownParamMi", "valueNumeric": mi}),
                    serde_json::json!({"codeID": "Disturbance", "valueNumeric": d}),
                ];
            }

            let mut element = serde_json::json!({
                "agsiDataParameterValue": agsi_data_parameter_value
            });

            if !geol_code.is_empty() {
                element["elementID"] = serde_json::Value::String(geol_code);
            }
            if !element_name.is_empty() {
                element["elementName"] = serde_json::Value::String(element_name);
            }

            agsi_model_elements.push(element);

            add_element = Confirm::new()
                .with_prompt("Add another model element?")
                .default(false)
                .interact()?;
        }
    }

    // If groundwater selected, prompt for groundwater plane
    if create_groundwater {
        println!("\n{}", blue.apply_to("--- Add Groundwater Level ---"));

        let elevation: f64 = Input::new()
            .with_prompt("Groundwater elevation (z)")
            .interact_text()?;

        let description: String = Input::new()
            .with_prompt("Description (optional)")
            .default("Groundwater level".to_string())
            .interact_text()?;

        let remarks: String = Input::new()
            .with_prompt("Remarks (optional)")
            .default("".to_string())
            .interact_text()?;

        agsi_model_elements.push(serde_json::json!({
            "elementID": "Groundwater",
            "elementName": description,
            "elementType": "Groundwater",
            "geometryObject": "agsiGeometryPlane",
            "agsiGeometry": {
                "elevation": elevation,
                "description": description,
                "remarks": remarks
            }
        }));
    }

    // Compose AGSi JSON structure
    let mut agsi_data = serde_json::json!({
        "agsSchema": {
            "name": schema_name,
            "version": schema_version
        },
        "agsFile": {
            "title": title,
            "producedBy": produced_by
        },
        "agsiModel": [{
            "agsiModelElement": agsi_model_elements
        }]
    });

    // Add observation structure if layers were created
    if !agsi_observation_columns.is_empty() {
        agsi_data["agsiModel"][0]["agsiObservationSet"] = serde_json::json!([{
            "agsiObservationExpHole": [{
                "holeID": "GeneratedHole1",
                "topCoordinate": [],
                "verticalHoleDepth": serde_json::Value::Null,
                "profileCoordinates": [],
                "holeType": "",
                "agsiObservationColumn": agsi_observation_columns
            }]
        }]);
    }

    let json_output = serde_json::to_string_pretty(&agsi_data)?;
    fs::write(output, json_output)?;

    println!("\n{}", green.apply_to("✓ AGSi JSON file generated successfully!"));
    println!("File saved to: {}", output.display());

    Ok(())
}

fn add_predefined_materials(input: &PathBuf, output: &PathBuf, material: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_content = fs::read_to_string(input)?;
    let mut agsi_data: serde_json::Value = serde_json::from_str(&input_content)?;

    let material_params = get_predefined_material(material)?;

    // Add material parameters to the AGSi structure
    if let Some(agsi_model) = agsi_data["agsiModel"].as_array_mut() {
        if let Some(first_model) = agsi_model.first_mut() {
            if let Some(elements) = first_model["agsiModelElement"].as_array_mut() {
                elements.push(serde_json::json!({
                    "agsiDataParameterValue": material_params
                }));
            }
        }
    }

    let output_json = serde_json::to_string_pretty(&agsi_data)?;
    fs::write(output, output_json)?;

    println!("Added material '{}' to AGSi file. Output written to: {}", material, output.display());
    Ok(())
}

fn get_predefined_material(material: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let materials = HashMap::from([
        ("clay", vec![
            serde_json::json!({"codeID": "UnitWeight", "valueNumeric": 19.0}),
            serde_json::json!({"codeID": "UndrainedShearStrength", "valueNumeric": 40.0}),
            serde_json::json!({"codeID": "YoungsModulus", "valueNumeric": 25000.0}),
        ]),
        ("sand", vec![
            serde_json::json!({"codeID": "UnitWeight", "valueNumeric": 17.0}),
            serde_json::json!({"codeID": "EffectiveFrictionAngle", "valueNumeric": 32.0}),
            serde_json::json!({"codeID": "EffectiveCohesion", "valueNumeric": 0.0}),
            serde_json::json!({"codeID": "YoungsModulus", "valueNumeric": 30000.0}),
        ]),
        ("gravel", vec![
            serde_json::json!({"codeID": "UnitWeight", "valueNumeric": 20.0}),
            serde_json::json!({"codeID": "EffectiveFrictionAngle", "valueNumeric": 38.0}),
            serde_json::json!({"codeID": "EffectiveCohesion", "valueNumeric": 0.0}),
            serde_json::json!({"codeID": "YoungsModulus", "valueNumeric": 80000.0}),
        ]),
        ("rock", vec![
            serde_json::json!({"codeID": "UnitWeight", "valueNumeric": 25.0}),
            serde_json::json!({"codeID": "EffectiveFrictionAngle", "valueNumeric": 45.0}),
            serde_json::json!({"codeID": "EffectiveCohesion", "valueNumeric": 100.0}),
            serde_json::json!({"codeID": "YoungsModulus", "valueNumeric": 500000.0}),
        ]),
    ]);

    materials.get(material)
        .ok_or_else(|| format!("Unknown material: {}", material).into())
        .map(|params| params.clone())
}

#[derive(Tabled)]
struct SoilLayerRow {
    #[tabled(rename = "Top Elevation (mAOD)")]
    top_elevation: String,
    #[tabled(rename = "Bottom Elevation (mAOD)")]
    bottom_elevation: String,
    #[tabled(rename = "Top Depth (mbgl)")]
    top_depth: String,
    #[tabled(rename = "Bottom Depth (mbgl)")]
    bottom_depth: String,
    #[tabled(rename = "Geology Code")]
    geology_code: String,
}

#[derive(Tabled)]
struct SoilParamsRow {
    #[tabled(rename = "Reference")]
    reference: String,
    #[tabled(rename = "γ (kN/m³)")]
    unit_weight: String,
    #[tabled(rename = "φ′ (°)")]
    friction_angle: String,
    #[tabled(rename = "c′ (kPa)")]
    cohesion: String,
    #[tabled(rename = "cu (kPa)")]
    undrained_strength: String,
    #[tabled(rename = "mv (1/kPa)")]
    compressibility: String,
    #[tabled(rename = "E (kPa)")]
    youngs_modulus: String,
    #[tabled(rename = "ν (Poisson)")]
    poissons_ratio: String,
}

#[derive(Tabled)]
struct RockParamsRow {
    #[tabled(rename = "Reference")]
    reference: String,
    #[tabled(rename = "GSI")]
    gsi: String,
    #[tabled(rename = "UCS (kPa)")]
    ucs: String,
    #[tabled(rename = "mi (Hoek-Brown)")]
    mi: String,
    #[tabled(rename = "Disturbance")]
    disturbance: String,
}

#[derive(Tabled)]
struct GroundwaterRow {
    #[tabled(rename = "Reference")]
    reference: String,
    #[tabled(rename = "Elevation (mAOD)")]
    elevation: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Remarks")]
    remarks: String,
}

fn handle_table_command(input: &PathBuf, word_output: Option<&PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let input_content = fs::read_to_string(input)?;
    let agsi_data: serde_json::Value = serde_json::from_str(&input_content)?;

    // Extract elements from AGSi structure
    let mut elements = Vec::new();
    if let Some(agsi_model) = agsi_data["agsiModel"].as_array() {
        for model in agsi_model {
            if let Some(model_elements) = model["agsiModelElement"].as_array() {
                elements.extend(model_elements.iter().cloned());
            }
        }
    }

    if elements.is_empty() {
        println!("Could not find any model elements in the AGSi file.");
        return Ok(());
    }

    // Convert elements to soil parameters
    let mut soil_params_vec = Vec::new();
    for element in &elements {
        if let Some(params_array) = element["agsiDataParameterValue"].as_array() {
            let ground_model = GroundModel::from_agsi_file(&agsi_data);
            soil_params_vec.extend(ground_model.soil_params);
            break; // Use the ground model conversion
        }
    }

    // Separate soils and rocks
    let soils: Vec<_> = soil_params_vec.iter().filter(|sp| sp.behaviour != SoilType::Rock).collect();
    let rocks: Vec<_> = soil_params_vec.iter().filter(|sp| sp.behaviour == SoilType::Rock).collect();

    // Extract soil layers from observation columns if available
    let mut layer_rows = Vec::new();
    if let Some(agsi_model) = agsi_data["agsiModel"].as_array() {
        if let Some(obs_set) = agsi_model.get(0).and_then(|m| m["agsiObservationSet"].as_array()) {
            if let Some(exp_hole) = obs_set.get(0).and_then(|o| o["agsiObservationExpHole"].as_array()) {
                if let Some(columns) = exp_hole.get(0).and_then(|e| e["agsiObservationColumn"].as_array()) {
                    for column in columns {
                        layer_rows.push(SoilLayerRow {
                            top_elevation: column.get("topElevation").and_then(|v| v.as_f64()).map(|f| f.to_string()).unwrap_or_default(),
                            bottom_elevation: column.get("bottomElevation").and_then(|v| v.as_f64()).map(|f| f.to_string()).unwrap_or_default(),
                            top_depth: column.get("topDepth").and_then(|v| v.as_f64()).map(|f| f.to_string()).unwrap_or_default(),
                            bottom_depth: column.get("bottomDepth").and_then(|v| v.as_f64()).map(|f| f.to_string()).unwrap_or_default(),
                            geology_code: column.get("geologyCode").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        });
                    }
                }
            }
        }
    }

    // Display soil layers table
    if !layer_rows.is_empty() {
        println!("\nSOIL LAYERS\n");
        println!("{}", "=".repeat(80));
        println!("{}", TabledTable::new(&layer_rows));
    }

    // Display soil parameters table
    if !soils.is_empty() {
        let soil_rows: Vec<SoilParamsRow> = soils.iter().map(|sp| {
            SoilParamsRow {
                reference: sp.reference.clone(),
                unit_weight: sp.unit_weight.to_string(),
                friction_angle: sp.phi_prime.map(|phi| format!("{:.1}", phi.to_degrees())).unwrap_or_default(),
                cohesion: sp.c_prime.map(|c| c.to_string()).unwrap_or_default(),
                undrained_strength: sp.cu.map(|cu| cu.to_string()).unwrap_or_default(),
                compressibility: if sp.mv > 0.0 { sp.mv.to_string() } else { String::new() },
                youngs_modulus: sp.youngs_modulus.to_string(),
                poissons_ratio: if sp.poissons_ratio > 0.0 { sp.poissons_ratio.to_string() } else { String::new() },
            }
        }).collect();

        println!("\nSOIL PARAMETERS\n");
        println!("{}", "=".repeat(120));
        println!("{}", TabledTable::new(&soil_rows));
    }

    // Display rock parameters table
    if !rocks.is_empty() {
        let rock_rows: Vec<RockParamsRow> = rocks.iter().map(|sp| {
            RockParamsRow {
                reference: sp.reference.clone(),
                gsi: sp.gsi.map(|g| g.to_string()).unwrap_or_default(),
                ucs: sp.ucs.map(|u| u.to_string()).unwrap_or_default(),
                mi: sp.mi.map(|m| m.to_string()).unwrap_or_default(),
                disturbance: sp.disturbance.to_string(),
            }
        }).collect();

        println!("\nROCK PARAMETERS\n");
        println!("{}", "=".repeat(80));
        println!("{}", TabledTable::new(&rock_rows));
    }

    // Extract groundwater elements
    let groundwater_elements: Vec<_> = elements.iter()
        .filter(|el| {
            el.get("elementType").and_then(|v| v.as_str()) == Some("Groundwater") &&
            el.get("geometryObject").and_then(|v| v.as_str()) == Some("agsiGeometryPlane") &&
            el.get("agsiGeometry").and_then(|g| g.get("elevation")).is_some()
        })
        .collect();

    if !groundwater_elements.is_empty() {
        let gw_rows: Vec<GroundwaterRow> = groundwater_elements.iter().map(|gw| {
            GroundwaterRow {
                reference: gw.get("elementID").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                elevation: gw.get("agsiGeometry")
                    .and_then(|g| g.get("elevation"))
                    .and_then(|v| v.as_f64())
                    .map(|f| f.to_string())
                    .unwrap_or_default(),
                description: gw.get("agsiGeometry")
                    .and_then(|g| g.get("description"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                remarks: gw.get("agsiGeometry")
                    .and_then(|g| g.get("remarks"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            }
        }).collect();

        println!("\nGROUNDWATER LEVELS\n");
        println!("{}", "=".repeat(80));
        println!("{}", TabledTable::new(&gw_rows));
    }

    // Export to Word document if requested
    if let Some(word_path) = word_output {
        export_to_word(&layer_rows, &soils, &rocks, &groundwater_elements, word_path)?;
        println!("\nExported tables to Word: {}", word_path.display());
    }

    Ok(())
}

fn export_to_word(
    layer_rows: &[SoilLayerRow],
    soils: &[&SoilParams],
    rocks: &[&SoilParams],
    groundwater_elements: &[&serde_json::Value],
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Docx::new();

    // Add soil layers table
    if !layer_rows.is_empty() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text("SOIL LAYERS").bold()));
        
        let mut table_rows = vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Top Elevation (mAOD)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bottom Elevation (mAOD)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Top Depth (mbgl)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bottom Depth (mbgl)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Geology Code"))),
            ])
        ];

        for row in layer_rows {
            table_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&row.top_elevation))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&row.bottom_elevation))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&row.top_depth))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&row.bottom_depth))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&row.geology_code))),
            ]));
        }

        let table = Table::new(table_rows);
        doc = doc.add_table(table).add_paragraph(Paragraph::new());
    }

    // Add soil parameters table
    if !soils.is_empty() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text("SOIL PARAMETERS").bold()));
        
        let mut table_rows = vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Reference"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("γ (kN/m³)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("φ′ (°)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("c′ (kPa)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("cu (kPa)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("mv (1/kPa)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("E (kPa)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("ν (Poisson)"))),
            ])
        ];

        for sp in soils {
            table_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.reference))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.unit_weight.to_string()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.phi_prime.map(|phi| format!("{:.1}", phi.to_degrees())).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.c_prime.map(|c| c.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.cu.map(|cu| cu.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&if sp.mv > 0.0 { sp.mv.to_string() } else { String::new() }))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.youngs_modulus.to_string()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&if sp.poissons_ratio > 0.0 { sp.poissons_ratio.to_string() } else { String::new() }))),
            ]));
        }

        let table = Table::new(table_rows);
        doc = doc.add_table(table).add_paragraph(Paragraph::new());
    }

    // Add rock parameters table
    if !rocks.is_empty() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text("ROCK PARAMETERS").bold()));
        
        let mut table_rows = vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Reference"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("GSI"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("UCS (kPa)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("mi (Hoek-Brown)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Disturbance"))),
            ])
        ];

        for sp in rocks {
            table_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.reference))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.gsi.map(|g| g.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.ucs.map(|u| u.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.mi.map(|m| m.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&sp.disturbance.to_string()))),
            ]));
        }

        let table = Table::new(table_rows);
        doc = doc.add_table(table).add_paragraph(Paragraph::new());
    }

    // Add groundwater table
    if !groundwater_elements.is_empty() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text("GROUNDWATER LEVELS").bold()));
        
        let mut table_rows = vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Reference"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Elevation (mAOD)"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Description"))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Remarks"))),
            ])
        ];

        for gw in groundwater_elements {
            table_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(gw.get("elementID").and_then(|v| v.as_str()).unwrap_or("")))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(&gw.get("agsiGeometry").and_then(|g| g.get("elevation")).and_then(|v| v.as_f64()).map(|f| f.to_string()).unwrap_or_default()))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(gw.get("agsiGeometry").and_then(|g| g.get("description")).and_then(|v| v.as_str()).unwrap_or("")))),
                TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(gw.get("agsiGeometry").and_then(|g| g.get("remarks")).and_then(|v| v.as_str()).unwrap_or("")))),
            ]));
        }

        let table = Table::new(table_rows);
        doc = doc.add_table(table);
    }

    let file = fs::File::create(output_path)?;
    doc.build().pack(file)?;

    Ok(())
}
