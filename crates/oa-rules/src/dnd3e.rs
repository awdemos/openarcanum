pub mod data;
pub mod generation;
pub mod validation;

use oa_core::{
    CharacterSheet, Error, GameSystem, GenerationOptions, GenerationResult, LevelUpChoice,
    RuleEngine, AttributeDefinition, ValueType, Constraint, ConstraintType, LevelRange, License,
};
use std::collections::HashMap;
use uuid::Uuid;

pub use data::*;
pub use generation::*;
pub use validation::*;

pub struct Dnd3eEngine {
    system: GameSystem,
}

impl Dnd3eEngine {
    pub fn new() -> Self {
        Self {
            system: system_definition(),
        }
    }
}

impl RuleEngine for Dnd3eEngine {
    fn system(&self) -> &GameSystem {
        &self.system
    }

    fn validate(&self, character: &CharacterSheet) -> Result<(), Vec<String>> {
        let errors = validation::validate_character(&character.attributes);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn generate(
        &self,
        template: &oa_core::CharacterTemplate,
        options: GenerationOptions,
    ) -> Result<GenerationResult, Error> {
        let race = template.default_values
            .get("race")
            .and_then(|v| match v {
                oa_core::AttributeValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "Human".to_string());

        let class = template.default_values
            .get("class")
            .and_then(|v| match v {
                oa_core::AttributeValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "Fighter".to_string());

        let level = options.starting_level.max(1).min(20);
        let use_rolled = options.use_rolled_stats;
        let point_buy = options.point_buy_budget;

        let attributes = match generation::generate_character_attributes(
            &race, &class, level, use_rolled, point_buy
        ) {
            Ok(attrs) => attrs,
            Err(e) => {
                return Ok(GenerationResult {
                    character: CharacterSheet {
                        id: Uuid::new_v4(),
                        name: template.name.clone(),
                        player: String::new(),
                        system_id: self.system.id,
                        created_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                        attributes: HashMap::new(),
                        metadata: HashMap::new(),
                    },
                    warnings: vec![e.clone()],
                    applied_rules: Vec::new(),
                    validation_errors: vec![e],
                });
            }
        };

        let mut metadata = HashMap::new();
        metadata.insert("template_id".to_string(), serde_json::json!(template.id.to_string()));
        let method = if point_buy.is_some() {
            "point_buy"
        } else if use_rolled {
            "4d6_drop_lowest"
        } else {
            "standard_array"
        };
        metadata.insert("generation_method".to_string(), serde_json::json!(method));

        let mut applied_rules = vec![
            format!("Generated stats using {}", method),
            format!("Applied {} racial modifiers", race),
            format!("Set class to {} with {} hit die", class, data::Class::from_str(&class).map(|c| c.hit_die()).unwrap_or(8)),
            format!("Calculated HP for {} level {}", class, level),
            format!("Calculated BAB and saves for {} level {}", class, level),
            format!("Allocated skill points for {} level {}", class, level),
            format!("Selected {} feats", data::normal_feat_count(level)),
        ];

        if data::Class::from_str(&class).map(|c| c.casts_spells()).unwrap_or(false) {
            applied_rules.push(format!("Calculated spell slots for {} level {}", class, level));
        }

        if data::Class::from_str(&class).map(|c| c.bonus_feats()).unwrap_or(false) {
            applied_rules.push(format!("Applied Fighter bonus feats"));
        }

        let character = CharacterSheet {
            id: Uuid::new_v4(),
            name: template.name.clone(),
            player: String::new(),
            system_id: self.system.id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            attributes,
            metadata,
        };

        Ok(GenerationResult {
            character,
            warnings: Vec::new(),
            applied_rules,
            validation_errors: Vec::new(),
        })
    }

    fn apply_level_up(
        &self,
        character: &mut CharacterSheet,
        choices: Vec<LevelUpChoice>,
    ) -> Result<(), Error> {
        let class_name = character.attributes
            .get("class")
            .and_then(|v| match v {
                oa_core::AttributeValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "Fighter".to_string());

        let current_level = character.attributes
            .get("level")
            .and_then(|v| match v {
                oa_core::AttributeValue::Integer(i) => Some(*i as u32),
                _ => None,
            })
            .unwrap_or(1);

        let new_level = (current_level + 1).min(20);

        if let Some(class) = data::Class::from_str(&class_name) {
            let con = character.attributes
                .get("constitution")
                .and_then(|v| match v {
                    oa_core::AttributeValue::Integer(i) => Some(*i),
                    _ => None,
                })
                .unwrap_or(10);
            let con_mod = data::ability_modifier(con);

            let new_hp = generation::roll_hp(&class, con_mod, 1);
            let current_hp = character.attributes
                .get("hit_points")
                .and_then(|v| match v {
                    oa_core::AttributeValue::Integer(i) => Some(*i),
                    _ => None,
                })
                .unwrap_or(0);
            character.attributes.insert(
                "hit_points".to_string(),
                oa_core::AttributeValue::Integer(current_hp + new_hp),
            );

            let new_bab = data::base_attack_bonus(class.bab_progression(), new_level);
            character.attributes.insert(
                "base_attack_bonus".to_string(),
                oa_core::AttributeValue::Integer(new_bab),
            );

            let new_saves = generation::calculate_saves(&class, new_level);
            for (key, value) in new_saves {
                character.attributes.insert(
                    format!("save_{}", key),
                    oa_core::AttributeValue::Integer(value),
                );
            }

            let num_attacks_val = data::num_attacks(new_bab);
            character.attributes.insert(
                "num_attacks".to_string(),
                oa_core::AttributeValue::Integer(num_attacks_val as i32),
            );

            let attack_bonuses_val = data::attack_bonuses(new_bab);
            character.attributes.insert(
                "attack_bonuses".to_string(),
                oa_core::AttributeValue::List(
                    attack_bonuses_val.iter().map(|&v| oa_core::AttributeValue::Integer(v)).collect()
                ),
            );

            if class.casts_spells() {
                let new_slots = generation::calculate_spell_slots(&class, new_level);
                if let Some(slots) = new_slots.get("spell_slots") {
                    character.attributes.insert(
                        "spell_slots".to_string(),
                        oa_core::AttributeValue::List(
                            slots.iter().map(|&s| oa_core::AttributeValue::Integer(s as i32)).collect()
                        ),
                    );
                }
            }

            let new_feats = generation::calculate_feats(&class, new_level);
            character.attributes.insert(
                "feats".to_string(),
                oa_core::AttributeValue::List(
                    new_feats.iter().map(|f| oa_core::AttributeValue::String(f.clone())).collect()
                ),
            );
        }

        for choice in choices {
            character.attributes.insert(
                format!("level_up_choice_{}", choice.category),
                oa_core::AttributeValue::String(choice.selection),
            );
        }

        character.attributes.insert(
            "level".to_string(),
            oa_core::AttributeValue::Integer(new_level as i32),
        );

        Ok(())
    }
}

pub fn system_definition() -> GameSystem {
    let id = Uuid::parse_str("dnd3e-0000-0000-0000-000000000001")
        .unwrap_or_else(|_| Uuid::new_v4());

    GameSystem {
        id,
        name: "Dungeons & Dragons 3.5 Edition".to_string(),
        version: "3.5".to_string(),
        publisher: "Wizards of the Coast".to_string(),
        license: License::Ogl,
        attributes: vec![
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Strength".to_string(),
                abbreviation: "STR".to_string(),
                description: "Physical power and melee combat ability".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "strength".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Dexterity".to_string(),
                abbreviation: "DEX".to_string(),
                description: "Agility, reflexes, and ranged combat ability".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "dexterity".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Constitution".to_string(),
                abbreviation: "CON".to_string(),
                description: "Health, stamina, and vital force".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "constitution".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Intelligence".to_string(),
                abbreviation: "INT".to_string(),
                description: "Mental acuity, memory, and reasoning".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "intelligence".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Wisdom".to_string(),
                abbreviation: "WIS".to_string(),
                description: "Perception, insight, and willpower".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "wisdom".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
            AttributeDefinition {
                id: Uuid::new_v4(),
                name: "Charisma".to_string(),
                abbreviation: "CHA".to_string(),
                description: "Personality, persuasiveness, and leadership".to_string(),
                value_type: ValueType::Integer { min: 3, max: 25 },
                constraints: vec![
                    Constraint {
                        constraint_type: ConstraintType::Requires,
                        target: "charisma".to_string(),
                        condition: "required".to_string(),
                    },
                ],
            },
        ],
        supported_levels: LevelRange { min: 1, max: 20 },
    }
}
