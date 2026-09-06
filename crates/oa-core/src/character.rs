use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Character {
    pub id: Uuid,
    pub identity: Identity,
    pub system_id: Uuid,
    pub level: u32,
    pub attributes: Vec<CharacterAttribute>,
    pub abilities: Vec<Ability>,
    pub inventory: Inventory,
    pub progression: Progression,
    pub state: CharacterState,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Identity {
    pub name: String,
    pub player_name: String,
    pub portrait_url: Option<String>,
    pub background: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct CharacterAttribute {
    pub definition_id: Uuid,
    pub name: String,
    pub base_value: i32,
    pub modifiers: Vec<Modifier>,
    pub current_value: i32,
}

impl CharacterAttribute {
    pub fn computed_value(&self) -> i32 {
        self.base_value + self.modifiers.iter().map(|m| m.value).sum::<i32>()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Modifier {
    pub source: String,
    pub value: i32,
    pub modifier_type: ModifierType,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModifierType {
    Racial,
    Class,
    Feat,
    Equipment,
    Temporary,
    Innate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Ability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub ability_type: AbilityType,
    pub source: String,
    pub uses_per_day: Option<u32>,
    pub action_cost: ActionCost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AbilityType {
    Spell,
    Feat,
    ClassFeature,
    RacialTrait,
    Skill,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionCost {
    Action,
    BonusAction,
    Reaction,
    FreeAction,
    Minute(u32),
    Hour(u32),
    Special(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct Inventory {
    pub items: Vec<Item>,
    pub currency: HashMap<String, u32>,
    pub carrying_capacity: u32,
    pub current_weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub weight: u32,
    pub value: u32,
    pub equipped: bool,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Weapon,
    Armor,
    Shield,
    Tool,
    Consumable,
    Container,
    Treasure,
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct Progression {
    pub experience_points: u32,
    pub classes: Vec<ClassLevel>,
    pub feats: Vec<Feat>,
    pub skill_ranks: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ClassLevel {
    pub class_id: Uuid,
    pub class_name: String,
    pub level: u32,
    pub subclass: Option<String>,
    pub hit_die: u32,
    pub features_gained: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Feat {
    pub id: Uuid,
    pub name: String,
    pub prerequisites: Vec<String>,
    pub benefit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct CharacterState {
    pub current_hit_points: i32,
    pub max_hit_points: i32,
    pub temporary_hit_points: i32,
    pub conditions: Vec<Condition>,
    pub status_effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Condition {
    pub name: String,
    pub severity: u32,
    pub duration: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct StatusEffect {
    pub name: String,
    pub source: String,
    pub modifiers: Vec<Modifier>,
    pub duration: Option<String>,
}

impl Character {
    pub fn new(name: String, player_name: String, system_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            identity: Identity {
                name,
                player_name,
                portrait_url: None,
                background: String::new(),
                description: String::new(),
            },
            system_id,
            level: 1,
            attributes: Vec::new(),
            abilities: Vec::new(),
            inventory: Inventory::default(),
            progression: Progression::default(),
            state: CharacterState::default(),
            metadata: HashMap::new(),
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<&CharacterAttribute> {
        self.attributes.iter().find(|a| a.name == name)
    }

    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut CharacterAttribute> {
        self.attributes.iter_mut().find(|a| a.name == name)
    }

    pub fn add_modifier(&mut self, attribute_name: &str, modifier: Modifier) -> Result<(), Error> {
        let attr = self
            .get_attribute_mut(attribute_name)
            .ok_or_else(|| Error::Validation(format!("Attribute '{}' not found", attribute_name)))?;
        attr.modifiers.push(modifier);
        attr.current_value = attr.computed_value();
        Ok(())
    }

    pub fn total_level(&self) -> u32 {
        self.progression.classes.iter().map(|c| c.level).sum()
    }

    pub fn effective_hit_points(&self) -> i32 {
        self.state.current_hit_points + self.state.temporary_hit_points
    }
}
