use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_FUZZY_THRESHOLD: f64 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaterialType {
    Soil,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Consistency {
    VerySoft,
    Soft,
    Firm,
    Stiff,
    VeryStiff,
    Hard,
    SoftToFirm,
    FirmToStiff,
    StiffToVeryStiff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Density {
    VeryLoose,
    Loose,
    MediumDense,
    Dense,
    VeryDense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum SoilType {
    Clay,
    Silt,
    Sand,
    Gravel,
    Peat,
    Organic,
    Cobbles,
    Boulders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum RockType {
    Limestone,
    Sandstone,
    Mudstone,
    Shale,
    Granite,
    Basalt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoistureContent {
    Dry,
    Moist,
    Wet,
    Saturated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Plasticity {
    #[serde(rename = "Non-Plastic")]
    NonPlastic,
    #[serde(rename = "Low Plasticity")]
    LowPlasticity,
    #[serde(rename = "Intermediate Plasticity")]
    IntermediatePlasticity,
    #[serde(rename = "High Plasticity")]
    HighPlasticity,
    #[serde(rename = "Extremely High Plasticity")]
    ExtremelyHighPlasticity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrengthParameterType {
    Ucs,
    UndrainedShear,
    SptNValue,
    FrictionAngle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryConstituent {
    pub amount: Option<String>,
    pub soil_type: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StrengthRange {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub typical_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthParameters {
    pub parameter_type: StrengthParameterType,
    pub value_range: StrengthRange,
    pub confidence: f64,
    pub units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellingCorrection {
    pub original: String,
    pub corrected: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilDescription {
    pub raw_description: String,
    pub material_type: Option<MaterialType>,
    pub confidence: f64,
    pub is_valid: bool,
    pub consistency: Option<Consistency>,
    pub density: Option<Density>,
    pub primary_soil_type: Option<SoilType>,
    pub rock_strength: Option<String>,
    pub weathering_grade: Option<String>,
    pub rock_structure: Option<String>,
    pub primary_rock_type: Option<RockType>,
    pub secondary_constituents: Vec<SecondaryConstituent>,
    pub color: Option<String>,
    pub moisture_content: Option<MoistureContent>,
    pub particle_size: Option<String>,
    pub plasticity: Option<Plasticity>,
    pub strength_parameters: Vec<StrengthParameters>,
    pub spelling_corrections: Vec<SpellingCorrection>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilDescriptionBuilder {
    pub raw_description: String,
    pub material_type: Option<MaterialType>,
    pub confidence: f64,
    pub is_valid: bool,
    pub consistency: Option<Consistency>,
    pub density: Option<Density>,
    pub primary_soil_type: Option<SoilType>,
    pub rock_strength: Option<String>,
    pub weathering_grade: Option<String>,
    pub rock_structure: Option<String>,
    pub primary_rock_type: Option<RockType>,
    pub secondary_constituents: Vec<SecondaryConstituent>,
    pub color: Option<String>,
    pub moisture_content: Option<MoistureContent>,
    pub particle_size: Option<String>,
    pub plasticity: Option<Plasticity>,
    pub strength_parameters: Vec<StrengthParameters>,
    pub spelling_corrections: Vec<SpellingCorrection>,
    pub warnings: Vec<String>,
}

pub fn create_empty_builder(raw_description: &str) -> SoilDescriptionBuilder {
    SoilDescriptionBuilder {
        raw_description: raw_description.to_string(),
        material_type: None,
        confidence: 0.0,
        is_valid: false,
        consistency: None,
        density: None,
        primary_soil_type: None,
        rock_strength: None,
        weathering_grade: None,
        rock_structure: None,
        primary_rock_type: None,
        secondary_constituents: Vec::new(),
        color: None,
        moisture_content: None,
        particle_size: None,
        plasticity: None,
        strength_parameters: Vec::new(),
        spelling_corrections: Vec::new(),
        warnings: Vec::new(),
    }
}

pub fn build_soil_description(builder: SoilDescriptionBuilder) -> SoilDescription {
    SoilDescription {
        raw_description: builder.raw_description,
        material_type: builder.material_type,
        confidence: builder.confidence,
        is_valid: builder.is_valid,
        consistency: builder.consistency,
        density: builder.density,
        primary_soil_type: builder.primary_soil_type,
        rock_strength: builder.rock_strength,
        weathering_grade: builder.weathering_grade,
        rock_structure: builder.rock_structure,
        primary_rock_type: builder.primary_rock_type,
        secondary_constituents: builder.secondary_constituents,
        color: builder.color,
        moisture_content: builder.moisture_content,
        particle_size: builder.particle_size,
        plasticity: builder.plasticity,
        strength_parameters: builder.strength_parameters,
        spelling_corrections: builder.spelling_corrections,
        warnings: builder.warnings,
    }
}

pub fn is_cohesive(soil_type: SoilType) -> bool {
    matches!(
        soil_type,
        SoilType::Clay | SoilType::Silt | SoilType::Peat | SoilType::Organic
    )
}

pub fn is_granular(soil_type: SoilType) -> bool {
    matches!(
        soil_type,
        SoilType::Sand | SoilType::Gravel | SoilType::Cobbles | SoilType::Boulders
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_type: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub corrected_from: Option<String>,
    pub similarity_score: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct FuzzyMatchResult<'a> {
    pub matched: &'a str,
    pub score: f64,
}

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a = a.to_lowercase();
    let b = b.to_lowercase();

    if a == b {
        return 0;
    }
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0; b.len() + 1];

    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] =
                std::cmp::min(std::cmp::min(prev[j + 1] + 1, curr[j] + 1), prev[j] + cost);
        }
        prev.clone_from(&curr);
    }

    prev[b.len()]
}

pub fn similarity(a: &str, b: &str) -> f64 {
    let max_len = std::cmp::max(a.len(), b.len());
    if max_len == 0 {
        return 1.0;
    }
    let distance = levenshtein_distance(a, b) as f64;
    1.0 - (distance / max_len as f64)
}

pub fn fuzzy_match<'a>(
    target: &str,
    options: &'a [String],
    threshold: f64,
) -> Option<FuzzyMatchResult<'a>> {
    let mut best: Option<FuzzyMatchResult<'a>> = None;
    for opt in options {
        let score = similarity(target, opt);
        if score >= threshold {
            match best {
                Some(b) if score <= b.score => {}
                _ => {
                    best = Some(FuzzyMatchResult {
                        matched: opt,
                        score,
                    })
                }
            }
        }
    }
    best
}

fn typo_dictionary() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("firn", "firm"),
        ("frim", "firm"),
        ("stif", "stiff"),
        ("stiif", "stiff"),
        ("vrey", "very"),
        ("vrry", "very"),
        ("clai", "clay"),
        ("caly", "clay"),
        ("claey", "clay"),
        ("snad", "sand"),
        ("snda", "sand"),
        ("gravle", "gravel"),
        ("gravelley", "gravelly"),
        ("limstone", "limestone"),
        ("limstn", "limestone"),
        ("sandston", "sandstone"),
        ("mudstn", "mudstone"),
        ("mudston", "mudstone"),
        ("granit", "granite"),
        ("weatherd", "weathered"),
        ("wethered", "weathered"),
        ("slighly", "slightly"),
        ("moderatly", "moderately"),
        ("dens", "dense"),
        ("loos", "loose"),
        ("medim", "medium"),
    ])
}

fn check_typo_dictionary(word: &str) -> Option<&'static str> {
    let lower = word.to_lowercase();
    typo_dictionary().get(lower.as_str()).copied()
}

const SINGLE_COLORS: [&str; 14] = [
    "brown", "gray", "grey", "red", "yellow", "orange", "black", "white", "green", "blue", "pink",
    "purple", "tan", "buff",
];

const PROPORTION_WORDS: [&str; 3] = ["slightly", "moderately", "very"];
const ADJECTIVE_WORDS: [&str; 4] = ["sandy", "silty", "gravelly", "clayey"];
const MOISTURE_TERMS: [&str; 4] = ["dry", "moist", "wet", "saturated"];
const PARTICLE_SIZE_TERMS: [&str; 3] = ["fine", "medium", "coarse"];
const CONSISTENCY_TERMS: [&str; 6] = ["very soft", "soft", "firm", "stiff", "very stiff", "hard"];
const DENSITY_TERMS: [&str; 5] = ["very loose", "loose", "medium dense", "dense", "very dense"];
const SOIL_TYPE_TERMS: [&str; 8] = [
    "clay", "silt", "sand", "gravel", "peat", "organic", "cobbles", "boulders",
];
const ROCK_TYPE_TERMS: [&str; 6] = [
    "limestone",
    "sandstone",
    "mudstone",
    "shale",
    "granite",
    "basalt",
];

#[derive(Debug, Clone, Copy)]
struct MultiWordPattern {
    pattern: &'static str,
    token_type: &'static str,
}

const CONSISTENCY_RANGE_PATTERNS: [MultiWordPattern; 5] = [
    MultiWordPattern {
        pattern: "soft to firm",
        token_type: "consistency-range",
    },
    MultiWordPattern {
        pattern: "firm to stiff",
        token_type: "consistency-range",
    },
    MultiWordPattern {
        pattern: "stiff to very stiff",
        token_type: "consistency-range",
    },
    MultiWordPattern {
        pattern: "very soft to soft",
        token_type: "consistency-range",
    },
    MultiWordPattern {
        pattern: "very stiff to hard",
        token_type: "consistency-range",
    },
];
const CONSISTENCY_PATTERNS: [MultiWordPattern; 2] = [
    MultiWordPattern {
        pattern: "very soft",
        token_type: "consistency",
    },
    MultiWordPattern {
        pattern: "very stiff",
        token_type: "consistency",
    },
];
const DENSITY_PATTERNS: [MultiWordPattern; 3] = [
    MultiWordPattern {
        pattern: "very loose",
        token_type: "density",
    },
    MultiWordPattern {
        pattern: "medium dense",
        token_type: "density",
    },
    MultiWordPattern {
        pattern: "very dense",
        token_type: "density",
    },
];
const ROCK_STRENGTH_PATTERNS: [MultiWordPattern; 5] = [
    MultiWordPattern {
        pattern: "extremely strong",
        token_type: "rock-strength",
    },
    MultiWordPattern {
        pattern: "very strong",
        token_type: "rock-strength",
    },
    MultiWordPattern {
        pattern: "very weak",
        token_type: "rock-strength",
    },
    MultiWordPattern {
        pattern: "moderately strong",
        token_type: "rock-strength",
    },
    MultiWordPattern {
        pattern: "moderately weak",
        token_type: "rock-strength",
    },
];
const WEATHERING_PATTERNS: [MultiWordPattern; 5] = [
    MultiWordPattern {
        pattern: "completely weathered",
        token_type: "weathering-grade",
    },
    MultiWordPattern {
        pattern: "highly weathered",
        token_type: "weathering-grade",
    },
    MultiWordPattern {
        pattern: "moderately weathered",
        token_type: "weathering-grade",
    },
    MultiWordPattern {
        pattern: "slightly weathered",
        token_type: "weathering-grade",
    },
    MultiWordPattern {
        pattern: "fresh",
        token_type: "weathering-grade",
    },
];
const PARTICLE_SIZE_PATTERNS: [MultiWordPattern; 3] = [
    MultiWordPattern {
        pattern: "fine to medium",
        token_type: "particle-size",
    },
    MultiWordPattern {
        pattern: "medium to coarse",
        token_type: "particle-size",
    },
    MultiWordPattern {
        pattern: "fine to coarse",
        token_type: "particle-size",
    },
];
const COLOR_PATTERNS: [MultiWordPattern; 13] = [
    MultiWordPattern {
        pattern: "dark brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "light brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "reddish brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "yellowish brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "greyish brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "grayish brown",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "dark gray",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "dark grey",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "light gray",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "light grey",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "brownish gray",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "brownish grey",
        token_type: "color",
    },
    MultiWordPattern {
        pattern: "yellowish",
        token_type: "color",
    },
];
const PLASTICITY_PATTERNS: [MultiWordPattern; 6] = [
    MultiWordPattern {
        pattern: "extremely high plasticity",
        token_type: "plasticity",
    },
    MultiWordPattern {
        pattern: "high plasticity",
        token_type: "plasticity",
    },
    MultiWordPattern {
        pattern: "intermediate plasticity",
        token_type: "plasticity",
    },
    MultiWordPattern {
        pattern: "low plasticity",
        token_type: "plasticity",
    },
    MultiWordPattern {
        pattern: "non-plastic",
        token_type: "plasticity",
    },
    MultiWordPattern {
        pattern: "non plastic",
        token_type: "plasticity",
    },
];

fn match_pattern(input: &str, pos: usize, pattern: &str) -> Option<usize> {
    let input_lower = input.to_lowercase();
    let pattern_lower = pattern.to_lowercase();
    if input_lower[pos..].starts_with(&pattern_lower) {
        let end = pos + pattern_lower.len();
        let next_char = input.chars().nth(end);
        if end >= input.len() || next_char.map_or(true, |c| c.is_whitespace() || ",;.)".contains(c))
        {
            return Some(end);
        }
    }
    None
}

fn try_multi_word_patterns(input: &str, pos: usize) -> Option<(Token, usize)> {
    fn try_group(input: &str, pos: usize, group: &[MultiWordPattern]) -> Option<(Token, usize)> {
        for pattern in group.iter() {
            if let Some(end) = match_pattern(input, pos, pattern.pattern) {
                return Some((
                    Token {
                        token_type: pattern.token_type.to_string(),
                        value: input[pos..end].to_string(),
                        start: pos,
                        end,
                        corrected_from: None,
                        similarity_score: None,
                    },
                    end,
                ));
            }
        }
        None
    }

    try_group(input, pos, &CONSISTENCY_RANGE_PATTERNS)
        .or_else(|| try_group(input, pos, &PLASTICITY_PATTERNS))
        .or_else(|| try_group(input, pos, &ROCK_STRENGTH_PATTERNS))
        .or_else(|| try_group(input, pos, &CONSISTENCY_PATTERNS))
        .or_else(|| try_group(input, pos, &DENSITY_PATTERNS))
        .or_else(|| try_group(input, pos, &WEATHERING_PATTERNS))
        .or_else(|| try_group(input, pos, &PARTICLE_SIZE_PATTERNS))
        .or_else(|| try_group(input, pos, &COLOR_PATTERNS))
}

fn classify_word(word: &str) -> (String, Option<String>, Option<f64>) {
    let lower = word.to_lowercase();

    if let Some(typo) = check_typo_dictionary(&lower) {
        let (token_type, _, _) = classify_word(typo);
        return (token_type, Some(typo.to_string()), Some(1.0));
    }

    if CONSISTENCY_TERMS.iter().any(|t| *t == lower) {
        return ("consistency".to_string(), None, None);
    }
    if DENSITY_TERMS.iter().any(|t| *t == lower) {
        return ("density".to_string(), None, None);
    }
    if lower == "strong" || lower == "weak" {
        return ("rock-strength".to_string(), None, None);
    }
    if SINGLE_COLORS.iter().any(|t| *t == lower) {
        return ("color".to_string(), None, None);
    }
    if PROPORTION_WORDS.iter().any(|t| *t == lower) {
        return ("proportion".to_string(), None, None);
    }
    if ADJECTIVE_WORDS.iter().any(|t| *t == lower) {
        return ("adjective".to_string(), None, None);
    }
    if MOISTURE_TERMS.iter().any(|t| *t == lower) {
        return ("moisture-content".to_string(), None, None);
    }
    if PARTICLE_SIZE_TERMS.iter().any(|t| *t == lower) {
        return ("particle-size".to_string(), None, None);
    }
    if SOIL_TYPE_TERMS.iter().any(|t| *t == lower) {
        return ("soil-type".to_string(), None, None);
    }
    if ROCK_TYPE_TERMS.iter().any(|t| *t == lower) {
        return ("rock-type".to_string(), None, None);
    }

    let all_terms: Vec<String> = SOIL_TYPE_TERMS
        .iter()
        .chain(ROCK_TYPE_TERMS.iter())
        .chain(SINGLE_COLORS.iter())
        .chain(PROPORTION_WORDS.iter())
        .chain(ADJECTIVE_WORDS.iter())
        .chain(MOISTURE_TERMS.iter())
        .chain(PARTICLE_SIZE_TERMS.iter())
        .map(|s| (*s).to_string())
        .collect();

    if let Some(result) = fuzzy_match(&lower, &all_terms, DEFAULT_FUZZY_THRESHOLD) {
        let (token_type, _, _) = classify_word(result.matched);
        return (
            token_type,
            Some(result.matched.to_string()),
            Some(result.score),
        );
    }

    ("unknown".to_string(), None, None)
}

pub fn tokenize(input: &str, max_tokens: Option<usize>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let len = input.len();
    let bytes = input.as_bytes();

    while pos < len {
        if let Some(max) = max_tokens {
            if tokens.len() >= max {
                break;
            }
        }

        while pos < len {
            let ch = bytes[pos] as char;
            if !ch.is_whitespace() {
                break;
            }
            pos += 1;
        }

        if pos >= len {
            break;
        }

        let current_char = bytes[pos] as char;
        if ",;().".contains(current_char) {
            pos += 1;
            continue;
        }

        if let Some((token, end)) = try_multi_word_patterns(input, pos) {
            tokens.push(token);
            pos = end;
            continue;
        }

        let mut end = pos;
        while end < len {
            let c = bytes[end] as char;
            if c.is_ascii_alphabetic() || c == '-' {
                end += 1;
            } else {
                break;
            }
        }

        if end > pos {
            let word = &input[pos..end];
            let (token_type, corrected_value, score) = classify_word(word);
            let mut token = Token {
                token_type,
                value: word.to_string(),
                start: pos,
                end,
                corrected_from: None,
                similarity_score: None,
            };

            if let Some(corrected) = corrected_value {
                token.corrected_from = Some(word.to_string());
                token.value = corrected;
            }
            if let Some(sc) = score {
                token.similarity_score = Some(sc);
            }

            tokens.push(token);
            pos = end;
            continue;
        }

        pos += 1;
    }

    tokens
}

fn consistency_from_string(s: &str) -> Option<Consistency> {
    match s.to_lowercase().as_str() {
        "very soft" => Some(Consistency::VerySoft),
        "soft" => Some(Consistency::Soft),
        "firm" => Some(Consistency::Firm),
        "stiff" => Some(Consistency::Stiff),
        "very stiff" => Some(Consistency::VeryStiff),
        "hard" => Some(Consistency::Hard),
        "soft to firm" => Some(Consistency::SoftToFirm),
        "firm to stiff" => Some(Consistency::FirmToStiff),
        "stiff to very stiff" => Some(Consistency::StiffToVeryStiff),
        _ => None,
    }
}

fn density_from_string(s: &str) -> Option<Density> {
    match s.to_lowercase().as_str() {
        "very loose" => Some(Density::VeryLoose),
        "loose" => Some(Density::Loose),
        "medium dense" => Some(Density::MediumDense),
        "dense" => Some(Density::Dense),
        "very dense" => Some(Density::VeryDense),
        _ => None,
    }
}

fn soil_type_from_string(s: &str) -> Option<SoilType> {
    match s.to_lowercase().as_str() {
        "clay" => Some(SoilType::Clay),
        "silt" => Some(SoilType::Silt),
        "sand" => Some(SoilType::Sand),
        "gravel" => Some(SoilType::Gravel),
        "peat" => Some(SoilType::Peat),
        "organic" => Some(SoilType::Organic),
        "cobbles" => Some(SoilType::Cobbles),
        "boulders" => Some(SoilType::Boulders),
        _ => None,
    }
}

fn rock_type_from_string(s: &str) -> Option<RockType> {
    match s.to_lowercase().as_str() {
        "limestone" => Some(RockType::Limestone),
        "sandstone" => Some(RockType::Sandstone),
        "mudstone" => Some(RockType::Mudstone),
        "shale" => Some(RockType::Shale),
        "granite" => Some(RockType::Granite),
        "basalt" => Some(RockType::Basalt),
        _ => None,
    }
}

pub fn consistency_to_display(c: Consistency) -> String {
    format!("{:?}", c).to_lowercase().replace('_', " ")
}

pub fn parse_soil_description(description: &str) -> SoilDescription {
    let mut builder = create_empty_builder(description);
    builder.material_type = Some(MaterialType::Soil);
    builder.confidence = 1.0;
    builder.is_valid = true;

    if description.trim().is_empty() {
        builder.confidence = 0.5;
        return build_soil_description(builder);
    }

    let tokens = tokenize(description, None);
    let mut pending_proportion: Option<String> = None;

    for tok in tokens {
        let tok_value = tok.value;
        match tok.token_type.as_str() {
            "consistency" | "consistency-range" => {
                if let Some(c) = consistency_from_string(&tok_value) {
                    builder.consistency = Some(c);
                }
            }
            "density" => {
                if let Some(d) = density_from_string(&tok_value) {
                    builder.density = Some(d);
                }
            }
            "proportion" => {
                pending_proportion = Some(tok_value.clone());
            }
            "adjective" => {
                if let Some(amount) = pending_proportion.take() {
                    builder.secondary_constituents.push(SecondaryConstituent {
                        amount: Some(amount),
                        soil_type: tok_value.clone(),
                    });
                } else {
                    builder.secondary_constituents.push(SecondaryConstituent {
                        amount: None,
                        soil_type: tok_value.clone(),
                    });
                }
            }
            "soil-type" => {
                if let Some(st) = soil_type_from_string(&tok_value) {
                    builder.primary_soil_type = Some(st);
                }
                builder.material_type = Some(MaterialType::Soil);
            }
            "rock-type" => {
                if let Some(rt) = rock_type_from_string(&tok_value) {
                    builder.primary_rock_type = Some(rt);
                }
                builder.material_type = Some(MaterialType::Rock);
            }
            "rock-strength" => {
                builder.rock_strength = Some(tok_value.replace(' ', "-"));
                builder.material_type = Some(MaterialType::Rock);
            }
            "weathering-grade" => {
                builder.weathering_grade = Some(tok_value.replace(' ', "-"));
            }
            "rock-structure" => {
                builder.rock_structure = Some(tok_value.clone());
                builder.material_type = Some(MaterialType::Rock);
            }
            "color" => {
                builder.color = Some(tok_value.clone());
            }
            "moisture-content" => {
                builder.moisture_content = match tok_value.to_lowercase().as_str() {
                    "dry" => Some(MoistureContent::Dry),
                    "moist" => Some(MoistureContent::Moist),
                    "wet" => Some(MoistureContent::Wet),
                    "saturated" => Some(MoistureContent::Saturated),
                    _ => None,
                };
            }
            "plasticity" => {
                builder.plasticity = match tok_value.to_lowercase().as_str() {
                    "non-plastic" | "non plastic" => Some(Plasticity::NonPlastic),
                    "low plasticity" => Some(Plasticity::LowPlasticity),
                    "intermediate plasticity" => Some(Plasticity::IntermediatePlasticity),
                    "high plasticity" => Some(Plasticity::HighPlasticity),
                    "extremely high plasticity" => Some(Plasticity::ExtremelyHighPlasticity),
                    _ => None,
                };
            }
            "particle-size" => {
                builder.particle_size = Some(tok_value.clone());
            }
            _ => {}
        }

        if let Some(original) = tok.corrected_from {
            builder.spelling_corrections.push(SpellingCorrection {
                original,
                corrected: tok_value.clone(),
                score: tok.similarity_score.unwrap_or(1.0),
            });
        }
    }

    let mut confidence = 1.0;
    for _ in &builder.spelling_corrections {
        confidence *= 0.8;
    }
    if builder.primary_soil_type.is_none() && builder.primary_rock_type.is_none() {
        confidence *= 0.7;
    }
    builder.confidence = confidence;

    if let Some(strength) = derive_strength_parameters(&builder) {
        builder.strength_parameters.push(strength);
    }

    build_soil_description(builder)
}

pub fn derive_strength_parameters(builder: &SoilDescriptionBuilder) -> Option<StrengthParameters> {
    if builder.material_type == Some(MaterialType::Soil) {
        if let Some(soil) = builder.primary_soil_type {
            if is_cohesive(soil) {
                if let Some(consistency) = builder.consistency {
                    if let Some(range) = get_consistency_range_value(consistency) {
                        return Some(StrengthParameters {
                            parameter_type: StrengthParameterType::UndrainedShear,
                            value_range: range,
                            confidence: 0.8,
                            units: "kPa".to_string(),
                        });
                    }
                }
            }

            if is_granular(soil) {
                if let Some(density) = builder.density {
                    if let Some(range) = density_to_spt_range(density) {
                        return Some(StrengthParameters {
                            parameter_type: StrengthParameterType::SptNValue,
                            value_range: range,
                            confidence: 0.8,
                            units: "blows".to_string(),
                        });
                    }
                }
            }
        }
    }

    if builder.material_type == Some(MaterialType::Rock) {
        if let Some(strength) = &builder.rock_strength {
            if let Some(range) = rock_strength_to_ucs_range(strength) {
                return Some(StrengthParameters {
                    parameter_type: StrengthParameterType::Ucs,
                    value_range: range,
                    confidence: 0.8,
                    units: "MPa".to_string(),
                });
            }
        }
    }

    None
}

fn get_consistency_range_value(consistency: Consistency) -> Option<StrengthRange> {
    match consistency {
        Consistency::SoftToFirm => {
            let soft = consistency_range(Consistency::Soft)?;
            let firm = consistency_range(Consistency::Firm)?;
            Some(StrengthRange {
                lower_bound: soft.lower_bound,
                upper_bound: firm.upper_bound,
                typical_value: (soft.typical_value + firm.typical_value) / 2.0,
            })
        }
        Consistency::FirmToStiff => {
            let firm = consistency_range(Consistency::Firm)?;
            let stiff = consistency_range(Consistency::Stiff)?;
            Some(StrengthRange {
                lower_bound: firm.lower_bound,
                upper_bound: stiff.upper_bound,
                typical_value: (firm.typical_value + stiff.typical_value) / 2.0,
            })
        }
        Consistency::StiffToVeryStiff => {
            let stiff = consistency_range(Consistency::Stiff)?;
            let very = consistency_range(Consistency::VeryStiff)?;
            Some(StrengthRange {
                lower_bound: stiff.lower_bound,
                upper_bound: very.upper_bound,
                typical_value: (stiff.typical_value + very.typical_value) / 2.0,
            })
        }
        _ => consistency_range(consistency),
    }
}

fn consistency_range(consistency: Consistency) -> Option<StrengthRange> {
    match consistency {
        Consistency::VerySoft => Some(StrengthRange {
            lower_bound: 0.0,
            upper_bound: 12.0,
            typical_value: 6.0,
        }),
        Consistency::Soft => Some(StrengthRange {
            lower_bound: 12.0,
            upper_bound: 25.0,
            typical_value: 18.0,
        }),
        Consistency::Firm => Some(StrengthRange {
            lower_bound: 25.0,
            upper_bound: 50.0,
            typical_value: 37.0,
        }),
        Consistency::Stiff => Some(StrengthRange {
            lower_bound: 50.0,
            upper_bound: 100.0,
            typical_value: 75.0,
        }),
        Consistency::VeryStiff => Some(StrengthRange {
            lower_bound: 100.0,
            upper_bound: 200.0,
            typical_value: 150.0,
        }),
        Consistency::Hard => Some(StrengthRange {
            lower_bound: 200.0,
            upper_bound: 400.0,
            typical_value: 300.0,
        }),
        _ => None,
    }
}

fn density_to_spt_range(density: Density) -> Option<StrengthRange> {
    match density {
        Density::VeryLoose => Some(StrengthRange {
            lower_bound: 0.0,
            upper_bound: 4.0,
            typical_value: 2.0,
        }),
        Density::Loose => Some(StrengthRange {
            lower_bound: 4.0,
            upper_bound: 10.0,
            typical_value: 7.0,
        }),
        Density::MediumDense => Some(StrengthRange {
            lower_bound: 10.0,
            upper_bound: 30.0,
            typical_value: 20.0,
        }),
        Density::Dense => Some(StrengthRange {
            lower_bound: 30.0,
            upper_bound: 50.0,
            typical_value: 40.0,
        }),
        Density::VeryDense => Some(StrengthRange {
            lower_bound: 50.0,
            upper_bound: 100.0,
            typical_value: 75.0,
        }),
    }
}

fn rock_strength_to_ucs_range(strength: &str) -> Option<StrengthRange> {
    match strength {
        "very-weak" => Some(StrengthRange {
            lower_bound: 0.25,
            upper_bound: 1.0,
            typical_value: 0.6,
        }),
        "weak" => Some(StrengthRange {
            lower_bound: 1.0,
            upper_bound: 5.0,
            typical_value: 2.5,
        }),
        "moderately-weak" => Some(StrengthRange {
            lower_bound: 5.0,
            upper_bound: 12.5,
            typical_value: 8.0,
        }),
        "moderately-strong" => Some(StrengthRange {
            lower_bound: 12.5,
            upper_bound: 50.0,
            typical_value: 25.0,
        }),
        "moderately" => Some(StrengthRange {
            lower_bound: 12.5,
            upper_bound: 50.0,
            typical_value: 25.0,
        }),
        "strong" => Some(StrengthRange {
            lower_bound: 50.0,
            upper_bound: 100.0,
            typical_value: 75.0,
        }),
        "very-strong" => Some(StrengthRange {
            lower_bound: 100.0,
            upper_bound: 200.0,
            typical_value: 150.0,
        }),
        "extremely-strong" => Some(StrengthRange {
            lower_bound: 200.0,
            upper_bound: 500.0,
            typical_value: 300.0,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ValidationOptions {
    pub strict: bool,
    pub require_strength_params: bool,
    pub require_primary_type: bool,
    pub check_correlations: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        ValidationOptions {
            strict: false,
            require_strength_params: false,
            require_primary_type: false,
            check_correlations: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeMinMax {
    min: f64,
    max: f64,
}

fn consistency_cu_ranges() -> HashMap<Consistency, RangeMinMax> {
    HashMap::from([
        (
            Consistency::VerySoft,
            RangeMinMax {
                min: 0.0,
                max: 12.0,
            },
        ),
        (
            Consistency::Soft,
            RangeMinMax {
                min: 12.0,
                max: 25.0,
            },
        ),
        (
            Consistency::SoftToFirm,
            RangeMinMax {
                min: 20.0,
                max: 40.0,
            },
        ),
        (
            Consistency::Firm,
            RangeMinMax {
                min: 25.0,
                max: 50.0,
            },
        ),
        (
            Consistency::FirmToStiff,
            RangeMinMax {
                min: 40.0,
                max: 75.0,
            },
        ),
        (
            Consistency::Stiff,
            RangeMinMax {
                min: 50.0,
                max: 100.0,
            },
        ),
        (
            Consistency::StiffToVeryStiff,
            RangeMinMax {
                min: 75.0,
                max: 150.0,
            },
        ),
        (
            Consistency::VeryStiff,
            RangeMinMax {
                min: 100.0,
                max: 200.0,
            },
        ),
        (
            Consistency::Hard,
            RangeMinMax {
                min: 200.0,
                max: 500.0,
            },
        ),
    ])
}

fn density_spt_ranges() -> HashMap<Density, RangeMinMax> {
    HashMap::from([
        (Density::VeryLoose, RangeMinMax { min: 0.0, max: 4.0 }),
        (
            Density::Loose,
            RangeMinMax {
                min: 4.0,
                max: 10.0,
            },
        ),
        (
            Density::MediumDense,
            RangeMinMax {
                min: 10.0,
                max: 30.0,
            },
        ),
        (
            Density::Dense,
            RangeMinMax {
                min: 30.0,
                max: 50.0,
            },
        ),
        (
            Density::VeryDense,
            RangeMinMax {
                min: 50.0,
                max: 100.0,
            },
        ),
    ])
}

fn rock_strength_ucs_ranges() -> HashMap<&'static str, RangeMinMax> {
    HashMap::from([
        (
            "extremely-weak",
            RangeMinMax {
                min: 0.25,
                max: 1.0,
            },
        ),
        ("very-weak", RangeMinMax { min: 1.0, max: 5.0 }),
        (
            "weak",
            RangeMinMax {
                min: 5.0,
                max: 25.0,
            },
        ),
        (
            "medium-strong",
            RangeMinMax {
                min: 25.0,
                max: 50.0,
            },
        ),
        (
            "strong",
            RangeMinMax {
                min: 50.0,
                max: 100.0,
            },
        ),
        (
            "very-strong",
            RangeMinMax {
                min: 100.0,
                max: 250.0,
            },
        ),
        (
            "extremely-strong",
            RangeMinMax {
                min: 250.0,
                max: 500.0,
            },
        ),
    ])
}

fn validate_strength_parameters(
    params: &StrengthParameters,
    index: usize,
) -> (Vec<ValidationError>, Vec<String>) {
    let mut errors = Vec::new();
    let warnings = Vec::new();
    let field_prefix = format!("strength_parameters[{}]", index);

    if params.value_range.lower_bound > params.value_range.upper_bound {
        errors.push(ValidationError {
            code: "INVALID_STRENGTH_RANGE".to_string(),
            message: format!(
                "Strength parameter {}: lower_bound exceeds upper_bound",
                index
            ),
            field: Some(format!("{}.value_range", field_prefix)),
        });
    }

    let StrengthRange {
        lower_bound,
        upper_bound,
        typical_value,
    } = params.value_range;
    if typical_value < lower_bound || typical_value > upper_bound {
        errors.push(ValidationError {
            code: "TYPICAL_VALUE_OUT_OF_RANGE".to_string(),
            message: format!(
                "Strength parameter {}: typical_value not within bounds",
                index
            ),
            field: Some(format!("{}.value_range.typical_value", field_prefix)),
        });
    }

    if params.confidence < 0.0 || params.confidence > 1.0 {
        errors.push(ValidationError {
            code: "INVALID_STRENGTH_CONFIDENCE".to_string(),
            message: format!("Strength parameter {}: confidence must be 0-1", index),
            field: Some(format!("{}.confidence", field_prefix)),
        });
    }

    (errors, warnings)
}

fn check_correlations(desc: &SoilDescription) -> (Vec<ValidationError>, Vec<String>) {
    let errors = Vec::new();
    let mut warnings = Vec::new();

    for param in &desc.strength_parameters {
        match param.parameter_type {
            StrengthParameterType::UndrainedShear => {
                if let Some(consistency) = desc.consistency {
                    if let Some(range) = consistency_cu_ranges().get(&consistency) {
                        let typical = param.value_range.typical_value;
                        if typical < range.min {
                            warnings.push(format!(
                                "Cu value ({:.1} kPa) seems low for {:?} consistency (expected {:.0}-{:.0} kPa)",
                                typical, consistency, range.min, range.max
                            ));
                        } else if typical > range.max {
                            warnings.push(format!(
                                "Cu value ({:.1} kPa) seems high for {:?} consistency (expected {:.0}-{:.0} kPa)",
                                typical, consistency, range.min, range.max
                            ));
                        }
                    }
                }
            }
            StrengthParameterType::SptNValue => {
                if let Some(density) = desc.density {
                    if let Some(range) = density_spt_ranges().get(&density) {
                        let typical = param.value_range.typical_value;
                        if typical < range.min {
                            warnings.push(format!(
                                "SPT N-value ({:.1}) seems low for {:?} density (expected {:.0}-{:.0})",
                                typical, density, range.min, range.max
                            ));
                        } else if typical > range.max {
                            warnings.push(format!(
                                "SPT N-value ({:.1}) seems high for {:?} density (expected {:.0}-{:.0})",
                                typical, density, range.min, range.max
                            ));
                        }
                    }
                }
            }
            StrengthParameterType::Ucs => {
                if let Some(strength) = &desc.rock_strength {
                    if let Some(range) = rock_strength_ucs_ranges().get(strength.as_str()) {
                        let typical = param.value_range.typical_value;
                        if typical < range.min {
                            warnings.push(format!(
                                "UCS value ({:.1} MPa) seems low for {} rock (expected {:.1}-{:.1} MPa)",
                                typical, strength, range.min, range.max
                            ));
                        } else if typical > range.max {
                            warnings.push(format!(
                                "UCS value ({:.1} MPa) seems high for {} rock (expected {:.1}-{:.1} MPa)",
                                typical, strength, range.min, range.max
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    (errors, warnings)
}

fn check_unusual_combinations(desc: &SoilDescription) -> Vec<String> {
    let mut warnings = Vec::new();

    if let Some(soil) = desc.primary_soil_type {
        if is_granular(soil) {
            if let Some(plasticity) = desc.plasticity {
                if matches!(
                    plasticity,
                    Plasticity::HighPlasticity | Plasticity::ExtremelyHighPlasticity
                ) {
                    warnings.push(format!(
                        "Unusual combination: high plasticity in granular soil ({:?})",
                        soil
                    ));
                }
            }
        }
    }

    if let Some(weathering) = &desc.weathering_grade {
        if weathering == "fresh" {
            if let Some(strength) = &desc.rock_strength {
                if ["extremely-weak", "very-weak", "weak"].contains(&strength.as_str()) {
                    warnings.push(format!(
                        "Unusual combination: fresh rock with {} strength",
                        strength
                    ));
                }
            }
        }
        if weathering == "completely-weathered" {
            if let Some(strength) = &desc.rock_strength {
                if ["strong", "very-strong", "extremely-strong"].contains(&strength.as_str()) {
                    warnings.push(format!(
                        "Unusual combination: completely weathered rock with {} strength",
                        strength
                    ));
                }
            }
        }
    }

    warnings
}

pub fn validate_soil_description(
    desc: &SoilDescription,
    opts: ValidationOptions,
) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if desc.material_type == Some(MaterialType::Soil) {
        if let Some(soil) = desc.primary_soil_type {
            if is_cohesive(soil) {
                if desc.density.is_some() {
                    errors.push(ValidationError {
                        code: "COHESIVE_WITH_DENSITY".to_string(),
                        message: format!("Cohesive soil ({:?}) should not have density", soil),
                        field: Some("density".to_string()),
                    });
                }
                if desc.consistency.is_none() {
                    warnings.push(format!(
                        "Cohesive soil ({:?}) should have consistency",
                        soil
                    ));
                }
            }
            if is_granular(soil) {
                if desc.consistency.is_some() {
                    errors.push(ValidationError {
                        code: "GRANULAR_WITH_CONSISTENCY".to_string(),
                        message: format!("Granular soil ({:?}) should not have consistency", soil),
                        field: Some("consistency".to_string()),
                    });
                }
                if desc.density.is_none() {
                    warnings.push(format!("Granular soil ({:?}) should have density", soil));
                }
            }
        }
    }

    if desc.material_type == Some(MaterialType::Rock) {
        if desc.consistency.is_some() {
            errors.push(ValidationError {
                code: "ROCK_WITH_CONSISTENCY".to_string(),
                message: "Rock should not have consistency".to_string(),
                field: Some("consistency".to_string()),
            });
        }
        if desc.density.is_some() {
            errors.push(ValidationError {
                code: "ROCK_WITH_DENSITY".to_string(),
                message: "Rock should not have density".to_string(),
                field: Some("density".to_string()),
            });
        }
        if desc.primary_soil_type.is_some() {
            errors.push(ValidationError {
                code: "ROCK_WITH_SOIL_TYPE".to_string(),
                message: "Rock should not have soil type".to_string(),
                field: Some("primary_soil_type".to_string()),
            });
        }
    }

    if desc.material_type == Some(MaterialType::Soil) {
        if desc.rock_strength.is_some() {
            errors.push(ValidationError {
                code: "SOIL_WITH_ROCK_STRENGTH".to_string(),
                message: "Soil should not have rock strength".to_string(),
                field: Some("rock_strength".to_string()),
            });
        }
        if desc.primary_rock_type.is_some() {
            errors.push(ValidationError {
                code: "SOIL_WITH_ROCK_TYPE".to_string(),
                message: "Soil should not have rock type".to_string(),
                field: Some("primary_rock_type".to_string()),
            });
        }
        if desc.weathering_grade.is_some() {
            errors.push(ValidationError {
                code: "SOIL_WITH_WEATHERING".to_string(),
                message: "Soil should not have weathering grade".to_string(),
                field: Some("weathering_grade".to_string()),
            });
        }
        if desc.rock_structure.is_some() {
            errors.push(ValidationError {
                code: "SOIL_WITH_ROCK_STRUCTURE".to_string(),
                message: "Soil should not have rock structure".to_string(),
                field: Some("rock_structure".to_string()),
            });
        }
    }

    if opts.require_primary_type {
        match desc.material_type {
            Some(MaterialType::Soil) if desc.primary_soil_type.is_none() => {
                errors.push(ValidationError {
                    code: "MISSING_SOIL_TYPE".to_string(),
                    message: "Soil description missing primary soil type".to_string(),
                    field: Some("primary_soil_type".to_string()),
                })
            }
            Some(MaterialType::Rock) if desc.primary_rock_type.is_none() => {
                errors.push(ValidationError {
                    code: "MISSING_ROCK_TYPE".to_string(),
                    message: "Rock description missing primary rock type".to_string(),
                    field: Some("primary_rock_type".to_string()),
                })
            }
            _ => {}
        }
    }

    if opts.require_strength_params {
        if desc.strength_parameters.is_empty() {
            let can_derive = desc.consistency.is_some()
                || desc.density.is_some()
                || desc.rock_strength.is_some();
            if can_derive {
                warnings.push(
                    "Strength parameters could be derived from description but are missing"
                        .to_string(),
                );
            }
        }
    }

    if desc.confidence < 0.0 || desc.confidence > 1.0 {
        errors.push(ValidationError {
            code: "INVALID_CONFIDENCE".to_string(),
            message: "Confidence must be between 0 and 1".to_string(),
            field: Some("confidence".to_string()),
        });
    }

    for (i, param) in desc.strength_parameters.iter().enumerate() {
        let (mut e, mut w) = validate_strength_parameters(param, i);
        errors.append(&mut e);
        warnings.append(&mut w);
    }

    if opts.check_correlations {
        let (mut e, mut w) = check_correlations(desc);
        errors.append(&mut e);
        warnings.append(&mut w);
    }

    warnings.extend(check_unusual_combinations(desc));

    if opts.strict {
        let mut strict_errors = Vec::new();
        for warning in warnings {
            strict_errors.push(ValidationError {
                code: "STRICT_WARNING".to_string(),
                message: warning,
                field: None,
            });
        }
        return ValidationResult {
            is_valid: strict_errors.is_empty() && errors.is_empty(),
            errors: [errors, strict_errors].concat(),
            warnings: Vec::new(),
        };
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}

pub fn is_valid_description(desc: &SoilDescription, opts: ValidationOptions) -> bool {
    validate_soil_description(desc, opts).is_valid
}

pub fn generate_description(desc: &SoilDescription) -> String {
    let mut parts: Vec<String> = Vec::new();

    if desc.material_type == Some(MaterialType::Rock) {
        if let Some(rs) = &desc.rock_strength {
            parts.push(rs.replace('-', " "));
        }
        if let Some(w) = &desc.weathering_grade {
            parts.push(w.replace('-', " "));
        }
        if let Some(s) = &desc.rock_structure {
            parts.push(s.to_string());
        }
        if let Some(c) = &desc.color {
            parts.push(c.to_string());
        }
        if let Some(rt) = &desc.primary_rock_type {
            parts.push(format!("{:?}", rt).to_uppercase());
        }
    } else {
        if let Some(c) = &desc.consistency {
            parts.push(consistency_to_display(*c));
        }
        if let Some(d) = &desc.density {
            parts.push(format!("{:?}", d).to_lowercase().replace('_', " "));
        }
        if let Some(c) = &desc.color {
            parts.push(c.to_string());
        }
        for sec in &desc.secondary_constituents {
            if let Some(amount) = &sec.amount {
                parts.push(format!("{} {}", amount, sec.soil_type));
            } else {
                parts.push(sec.soil_type.to_string());
            }
        }
        if let Some(m) = &desc.moisture_content {
            parts.push(format!("{:?}", m).to_lowercase());
        }
        if let Some(p) = &desc.plasticity {
            parts.push(format!("{:?}", p));
        }
        if let Some(p) = &desc.particle_size {
            parts.push(p.to_string());
        }
        if let Some(st) = &desc.primary_soil_type {
            parts.push(format!("{:?}", st).to_uppercase());
        }
    }

    parts.join(" ")
}
