#[cfg(test)]
mod tests {
    use crate::soil_description::{
        generate_description, parse_soil_description, validate_soil_description, Consistency,
        Density, MaterialType, SoilType, ValidationOptions,
    };

    #[test]
    fn parses_cohesive_soil_with_consistency() {
        let desc = parse_soil_description("Firm brown clay, moist");
        assert_eq!(desc.material_type, Some(MaterialType::Soil));
        assert_eq!(desc.primary_soil_type, Some(SoilType::Clay));
        assert_eq!(desc.consistency, Some(Consistency::Firm));
        assert!(desc.moisture_content.is_some());
        assert!(!desc.strength_parameters.is_empty());
    }

    #[test]
    fn parses_granular_soil_with_density() {
        let desc = parse_soil_description("Medium dense sand");
        assert_eq!(desc.primary_soil_type, Some(SoilType::Sand));
        assert_eq!(desc.density, Some(Density::MediumDense));
        assert!(!desc.strength_parameters.is_empty());
    }

    #[test]
    fn parses_rock_strength_and_weathering() {
        let desc = parse_soil_description("Strong limestone, slightly weathered");
        assert_eq!(desc.material_type, Some(MaterialType::Rock));
        assert!(desc.rock_strength.is_some());
        assert!(desc.weathering_grade.is_some());
        // Strength parameters are derived from rock strength when it matches expected forms
    }

    #[test]
    fn validation_catches_invalid_combo() {
        let desc = parse_soil_description("Firm dense clay");
        let result = validate_soil_description(&desc, ValidationOptions::default());
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn generator_round_trip_non_empty() {
        let desc = parse_soil_description("Soft clay, moist");
        let generated = generate_description(&desc);
        assert!(!generated.is_empty());
    }
}
