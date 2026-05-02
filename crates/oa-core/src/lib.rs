pub mod character;
pub mod error;
pub mod export;
pub mod import;
pub mod rules;
pub mod schema;
pub mod system;
pub mod validation;

pub use character::*;
pub use error::*;
pub use export::*;
pub use import::*;
pub use rules::*;
pub use schema::*;
pub use system::*;
pub use validation::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct GameSystem {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub license: License,
    pub attributes: Vec<AttributeDefinition>,
    pub supported_levels: LevelRange,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum License {
    Ogl,
    CcBySa,
    CcBy,
    Mit,
    Apache2,
    Proprietary,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AttributeDefinition {
    pub id: Uuid,
    pub name: String,
    pub abbreviation: String,
    pub description: String,
    pub value_type: ValueType,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Integer { min: i32, max: i32 },
    Dice { notation: String },
    Enum { values: Vec<String> },
    Boolean,
    String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub target: String,
    pub condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    Min,
    Max,
    Equal,
    DependsOn,
    ExclusiveWith,
    Requires,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct LevelRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CharacterSheet {
    pub id: Uuid,
    pub name: String,
    pub player: String,
    pub system_id: Uuid,
    pub created_at: String,
    pub updated_at: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum AttributeValue {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    String(String),
    List(Vec<AttributeValue>),
    Map(HashMap<String, AttributeValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CharacterTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub system_id: Uuid,
    pub required_attributes: Vec<String>,
    pub optional_attributes: Vec<String>,
    pub default_values: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GenerationResult {
    pub character: CharacterSheet,
    pub warnings: Vec<String>,
    pub applied_rules: Vec<String>,
    pub validation_errors: Vec<String>,
}

pub trait RuleEngine: Send + Sync {
    fn system(&self) -> &GameSystem;
    fn validate(&self, character: &CharacterSheet) -> Result<(), Vec<String>>;
    fn generate(&self, template: &CharacterTemplate, options: GenerationOptions) -> Result<GenerationResult, Error>;
    fn apply_level_up(&self, character: &mut CharacterSheet, choices: Vec<LevelUpChoice>) -> Result<(), Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct GenerationOptions {
    pub random_seed: Option<u64>,
    pub point_buy_budget: Option<u32>,
    pub use_rolled_stats: bool,
    pub starting_level: u32,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct LevelUpChoice {
    pub category: String,
    pub selection: String,
    pub target_id: Option<String>,
}
