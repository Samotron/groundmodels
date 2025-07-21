use groundmodels_core::{convert_agsi_file, ConvertType};
use std::fs;

fn main() {
    // Example AGSi JSON data
    let agsi_json = r#"
    {
        "agsiModel": [
            {
                "agsiModelElement": [
                    {
                        "elementName": "Clay Layer",
                        "agsiDataParameterValue": [
                            {
                                "codeID": "UnitWeight",
                                "valueNumeric": 18.0
                            },
                            {
                                "codeID": "AngleFriction",
                                "valueNumeric": 25.0
                            },
                            {
                                "codeID": "Cohesion",
                                "valueNumeric": 10.0
                            },
                            {
                                "codeID": "YoungsModulus",
                                "valueNumeric": 15000.0
                            }
                        ]
                    },
                    {
                        "elementName": "Sand Layer",
                        "agsiDataParameterValue": [
                            {
                                "codeID": "UnitWeight",
                                "valueNumeric": 20.0
                            },
                            {
                                "codeID": "AngleFriction", 
                                "valueNumeric": 35.0
                            },
                            {
                                "codeID": "YoungsModulus",
                                "valueNumeric": 50000.0
                            }
                        ]
                    }
                ]
            }
        ]
    }
    "#;

    // Write example file
    fs::write("example_agsi.json", agsi_json).expect("Failed to write example file");

    // Convert to SoilParams
    println!("=== Converting to SoilParams ===");
    match convert_agsi_file("example_agsi.json", ConvertType::SoilParams, None) {
        Ok(_) => println!("Conversion to SoilParams successful!"),
        Err(e) => println!("Error converting to SoilParams: {}", e),
    }

    println!("\n=== Converting to GroundModel ===");
    // Convert to GroundModel
    match convert_agsi_file("example_agsi.json", ConvertType::GroundModel, Some("ground_model_output.json")) {
        Ok(_) => println!("Conversion to GroundModel successful!"),
        Err(e) => println!("Error converting to GroundModel: {}", e),
    }

    // Clean up
    let _ = fs::remove_file("example_agsi.json");
    let _ = fs::remove_file("ground_model_output.json");
}