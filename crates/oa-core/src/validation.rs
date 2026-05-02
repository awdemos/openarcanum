use serde::{Deserialize, Serialize};

use crate::{Character, Error, GameSystem};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValidationError {
    pub code: String,
    pub field: String,
    pub message: String,
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValidationWarning {
    pub code: String,
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

pub trait CharacterValidator: Send + Sync {
    fn validate(&self, character: &Character, system: &GameSystem) -> ValidationReport;
}

pub struct CompositeValidator {
    validators: Vec<Box<dyn CharacterValidator>>,
}

impl CompositeValidator {
    pub fn new(validators: Vec<Box<dyn CharacterValidator>>) -> Self {
        Self { validators }
    }
}

impl CharacterValidator for CompositeValidator {
    fn validate(&self, character: &Character, system: &GameSystem) -> ValidationReport {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for validator in &self.validators {
            let report = validator.validate(character, system);
            errors.extend(report.errors);
            warnings.extend(report.warnings);
        }

        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

pub struct AttributeRangeValidator;

impl CharacterValidator for AttributeRangeValidator {
    fn validate(&self, character: &Character, system: &GameSystem) -> ValidationReport {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for attr_def in &system.attributes {
            if let Some(attr) = character.get_attribute(&attr_def.name) {
                match &attr_def.value_type {
                    crate::ValueType::Integer { min, max } => {
                        let value = attr.computed_value();
                        if value < *min {
                            errors.push(ValidationError {
                                code: "attribute_below_minimum".to_string(),
                                field: attr_def.name.clone(),
                                message: format!(
                                    "Attribute {} value {} is below minimum {}",
                                    attr_def.name, value, min
                                ),
                                rule_id: None,
                            });
                        }
                        if value > *max {
                            errors.push(ValidationError {
                                code: "attribute_above_maximum".to_string(),
                                field: attr_def.name.clone(),
                                message: format!(
                                    "Attribute {} value {} exceeds maximum {}",
                                    attr_def.name, value, max
                                ),
                                rule_id: None,
                            });
                        }
                    }
                    _ => {}
                }
            } else if attr_def.constraints.iter().any(|c| c.constraint_type == crate::ConstraintType::Requires) {
                errors.push(ValidationError {
                    code: "missing_required_attribute".to_string(),
                    field: attr_def.name.clone(),
                    message: format!("Required attribute {} is missing", attr_def.name),
                    rule_id: None,
                });
            }
        }

        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

pub struct LevelValidator;

impl CharacterValidator for LevelValidator {
    fn validate(&self, character: &Character, system: &GameSystem) -> ValidationReport {
        let mut errors = Vec::new();
        let total_level = character.total_level();

        if total_level < system.supported_levels.min {
            errors.push(ValidationError {
                code: "level_below_minimum".to_string(),
                field: "level".to_string(),
                message: format!(
                    "Total level {} is below system minimum {}",
                    total_level, system.supported_levels.min
                ),
                rule_id: None,
            });
        }

        if total_level > system.supported_levels.max {
            errors.push(ValidationError {
                code: "level_above_maximum".to_string(),
                field: "level".to_string(),
                message: format!(
                    "Total level {} exceeds system maximum {}",
                    total_level, system.supported_levels.max
                ),
                rule_id: None,
            });
        }

        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings: Vec::new(),
        }
    }
}
