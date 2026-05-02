use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRegistry {
    pub schemas: Vec<SchemaInfo>,
}

impl SchemaRegistry {
    pub fn default_registry() -> Self {
        Self {
            schemas: vec![
                character_schema_info(),
                system_schema_info(),
                generation_options_schema_info(),
            ],
        }
    }
}

fn character_schema_info() -> SchemaInfo {
    SchemaInfo {
        name: "character".to_string(),
        version: "0.1.0".to_string(),
        description: "A fully realized RPG character with identity, attributes, abilities, inventory, and progression".to_string(),
        fields: vec![
            SchemaField { name: "id".to_string(), field_type: "uuid".to_string(), required: true, description: "Unique character identifier".to_string() },
            SchemaField { name: "identity".to_string(), field_type: "object".to_string(), required: true, description: "Character identity (name, background, etc.)".to_string() },
            SchemaField { name: "system_id".to_string(), field_type: "uuid".to_string(), required: true, description: "Game system this character belongs to".to_string() },
            SchemaField { name: "level".to_string(), field_type: "integer".to_string(), required: true, description: "Character level".to_string() },
            SchemaField { name: "attributes".to_string(), field_type: "array".to_string(), required: true, description: "Character attributes with base values and modifiers".to_string() },
            SchemaField { name: "abilities".to_string(), field_type: "array".to_string(), required: false, description: "Character abilities, spells, feats, and features".to_string() },
            SchemaField { name: "inventory".to_string(), field_type: "object".to_string(), required: false, description: "Character inventory and equipment".to_string() },
            SchemaField { name: "progression".to_string(), field_type: "object".to_string(), required: false, description: "Character class levels, experience, and feats".to_string() },
            SchemaField { name: "state".to_string(), field_type: "object".to_string(), required: false, description: "Current character state (HP, conditions, etc.)".to_string() },
            SchemaField { name: "metadata".to_string(), field_type: "object".to_string(), required: false, description: "Additional system-specific metadata".to_string() },
        ],
    }
}

fn system_schema_info() -> SchemaInfo {
    SchemaInfo {
        name: "game_system".to_string(),
        version: "0.1.0".to_string(),
        description: "Definition of an RPG game system including attributes, constraints, and level ranges".to_string(),
        fields: vec![
            SchemaField { name: "id".to_string(), field_type: "uuid".to_string(), required: true, description: "Unique system identifier".to_string() },
            SchemaField { name: "name".to_string(), field_type: "string".to_string(), required: true, description: "System name".to_string() },
            SchemaField { name: "version".to_string(), field_type: "string".to_string(), required: true, description: "System version".to_string() },
            SchemaField { name: "publisher".to_string(), field_type: "string".to_string(), required: true, description: "System publisher".to_string() },
            SchemaField { name: "license".to_string(), field_type: "enum".to_string(), required: true, description: "License type (ogl, cc_by_sa, mit, apache2, proprietary, other)".to_string() },
            SchemaField { name: "attributes".to_string(), field_type: "array".to_string(), required: true, description: "System attribute definitions".to_string() },
            SchemaField { name: "supported_levels".to_string(), field_type: "object".to_string(), required: true, description: "Min and max supported levels".to_string() },
        ],
    }
}

fn generation_options_schema_info() -> SchemaInfo {
    SchemaInfo {
        name: "generation_options".to_string(),
        version: "0.1.0".to_string(),
        description: "Options for character generation".to_string(),
        fields: vec![
            SchemaField { name: "random_seed".to_string(), field_type: "integer".to_string(), required: false, description: "Seed for reproducible generation".to_string() },
            SchemaField { name: "point_buy_budget".to_string(), field_type: "integer".to_string(), required: false, description: "Point buy budget for attributes".to_string() },
            SchemaField { name: "use_rolled_stats".to_string(), field_type: "boolean".to_string(), required: false, description: "Use rolled stats instead of point buy".to_string() },
            SchemaField { name: "starting_level".to_string(), field_type: "integer".to_string(), required: true, description: "Starting character level".to_string() },
            SchemaField { name: "restrictions".to_string(), field_type: "array".to_string(), required: false, description: "Generation restrictions".to_string() },
        ],
    }
}

pub fn validate_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, crate::Error> {
    serde_json::from_str(json).map_err(|e| crate::Error::Schema(format!("JSON validation failed: {}", e)))
}
