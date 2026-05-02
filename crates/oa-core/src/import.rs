use serde::Deserialize;
use std::collections::HashMap;

use crate::{Character, CharacterSheet, Error, AttributeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    DndBeyond,
    FantasyGrounds,
    Ogc,
    Json,
}

impl std::str::FromStr for ImportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dndbeyond" | "dnd_beyond" | "beyond" => Ok(ImportFormat::DndBeyond),
            "fantasy_grounds" | "fg" | "fantasygrounds" => Ok(ImportFormat::FantasyGrounds),
            "ogc" | "open" | "ogl" | "srd" => Ok(ImportFormat::Ogc),
            "json" => Ok(ImportFormat::Json),
            _ => Err(format!("Unknown import format: {}", s)),
        }
    }
}

pub fn import_character(data: &str, format: ImportFormat) -> Result<Character, Error> {
    match format {
        ImportFormat::DndBeyond => import_dndbeyond(data),
        ImportFormat::FantasyGrounds => import_fantasy_grounds(data),
        ImportFormat::Ogc => import_ogc(data),
        ImportFormat::Json => import_json(data),
    }
}

pub fn import_character_sheet(data: &str, format: ImportFormat) -> Result<CharacterSheet, Error> {
    match format {
        ImportFormat::DndBeyond => import_dndbeyond_sheet(data),
        ImportFormat::FantasyGrounds => import_fantasy_grounds_sheet(data),
        ImportFormat::Ogc => import_ogc_sheet(data),
        ImportFormat::Json => import_json_sheet(data),
    }
}

fn import_dndbeyond(data: &str) -> Result<Character, Error> {
    let beyond: DndBeyondCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("D&D Beyond parse error: {}", e)))?;
    beyond.try_into()
}

fn import_dndbeyond_sheet(data: &str) -> Result<CharacterSheet, Error> {
    let beyond: DndBeyondCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("D&D Beyond parse error: {}", e)))?;
    Ok(beyond.into_sheet())
}

fn import_fantasy_grounds(data: &str) -> Result<Character, Error> {
    let fg: FantasyGroundsCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("Fantasy Grounds parse error: {}", e)))?;
    fg.try_into()
}

fn import_fantasy_grounds_sheet(data: &str) -> Result<CharacterSheet, Error> {
    let fg: FantasyGroundsCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("Fantasy Grounds parse error: {}", e)))?;
    Ok(fg.into_sheet())
}

fn import_ogc(data: &str) -> Result<Character, Error> {
    let ogc: crate::export::OgcCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("OGC parse error: {}", e)))?;
    ogc.try_into()
}

fn import_ogc_sheet(data: &str) -> Result<CharacterSheet, Error> {
    let ogc: crate::export::OgcCharacter = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("OGC parse error: {}", e)))?;
    Ok(ogc.into_sheet())
}

fn import_json(data: &str) -> Result<Character, Error> {
    let sheet: CharacterSheet = serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("JSON parse error: {}", e)))?;
    Ok(sheet_to_character(sheet))
}

fn import_json_sheet(data: &str) -> Result<CharacterSheet, Error> {
    serde_json::from_str(data)
        .map_err(|e| Error::Serialization(format!("JSON parse error: {}", e)))
}

fn sheet_to_character(sheet: CharacterSheet) -> Character {
    use crate::{Identity, CharacterAttribute, Inventory, Progression, CharacterState, ClassLevel};
    use uuid::Uuid;

    let id = Uuid::parse_str(&sheet.id.to_string()).unwrap_or_else(|_| Uuid::new_v4());

    let attributes: Vec<CharacterAttribute> = sheet.attributes.iter().map(|(name, value)| {
        let base = match value {
            AttributeValue::Integer(i) => *i,
            AttributeValue::Float(f) => *f as i32,
            AttributeValue::String(s) => s.parse::<i32>().unwrap_or(0),
            _ => 0,
        };
        CharacterAttribute {
            definition_id: Uuid::new_v4(),
            name: name.clone(),
            base_value: base,
            modifiers: Vec::new(),
            current_value: base,
        }
    }).collect();

    let class = sheet.attributes.get("class")
        .and_then(|v| match v {
            AttributeValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let level = sheet.attributes.get("level")
        .and_then(|v| match v {
            AttributeValue::Integer(i) => Some(*i as u32),
            _ => None,
        })
        .unwrap_or(1);

    let hit_points = sheet.attributes.get("hit_points")
        .and_then(|v| match v {
            AttributeValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(0);

    Character {
        id,
        identity: Identity {
            name: sheet.name.clone(),
            player_name: sheet.player.clone(),
            portrait_url: None,
            background: String::new(),
            description: sheet.metadata.get("template_description")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default(),
        },
        system_id: sheet.system_id,
        level,
        attributes,
        abilities: Vec::new(),
        inventory: Inventory {
            items: Vec::new(),
            currency: HashMap::new(),
            carrying_capacity: 0,
            current_weight: 0,
        },
        progression: Progression {
            classes: vec![ClassLevel {
                class_id: Uuid::new_v4(),
                class_name: class,
                subclass: None,
                level,
                hit_die: 8,
                features_gained: Vec::new(),
            }],
            experience_points: 0,
            feats: Vec::new(),
            skill_ranks: HashMap::new(),
        },
        state: CharacterState {
            current_hit_points: hit_points,
            max_hit_points: hit_points,
            temporary_hit_points: 0,
            conditions: Vec::new(),
            status_effects: Vec::new(),
        },
        metadata: sheet.metadata.clone(),
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DndBeyondCharacter {
    name: String,
    #[serde(default)]
    classes: Vec<DndBeyondClass>,
    #[serde(default)]
    race: DndBeyondRace,
    #[serde(default)]
    stats: Vec<DndBeyondStat>,
    #[serde(default)]
    hit_points: Option<i32>,
    #[serde(default)]
    armor_class: Option<i32>,
    #[serde(default)]
    alignment_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
struct DndBeyondClass {
    definition: DndBeyondClassDef,
    level: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct DndBeyondClassDef {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct DndBeyondRace {
    #[serde(default)]
    full_name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct DndBeyondStat {
    id: i32,
    value: i32,
}

impl DndBeyondCharacter {
    fn into_sheet(self) -> CharacterSheet {
        let mut attributes = HashMap::new();
        
        for stat in &self.stats {
            let name = match stat.id {
                1 => "strength",
                2 => "dexterity",
                3 => "constitution",
                4 => "intelligence",
                5 => "wisdom",
                6 => "charisma",
                _ => continue,
            };
            attributes.insert(name.to_string(), AttributeValue::Integer(stat.value));
        }

        attributes.insert("race".to_string(), AttributeValue::String(self.race.full_name));
        
        if let Some(class) = self.classes.first() {
            attributes.insert("class".to_string(), AttributeValue::String(class.definition.name.clone()));
            attributes.insert("level".to_string(), AttributeValue::Integer(class.level as i32));
        }

        if let Some(hp) = self.hit_points {
            attributes.insert("hit_points".to_string(), AttributeValue::Integer(hp));
        }

        if let Some(ac) = self.armor_class {
            attributes.insert("armor_class".to_string(), AttributeValue::Integer(ac));
        }

        CharacterSheet {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            player: String::new(),
            system_id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            attributes,
            metadata: HashMap::new(),
        }
    }
}

impl TryFrom<DndBeyondCharacter> for Character {
    type Error = Error;

    fn try_from(beyond: DndBeyondCharacter) -> Result<Self, Self::Error> {
        let sheet = beyond.into_sheet();
        Ok(sheet_to_character(sheet))
    }
}

#[derive(Debug, Clone, Deserialize)]
struct FantasyGroundsCharacter {
    name: String,
    #[serde(default)]
    race: String,
    #[serde(default)]
    class: String,
    #[serde(default)]
    level: u32,
    #[serde(default)]
    strength: i32,
    #[serde(default)]
    dexterity: i32,
    #[serde(default)]
    constitution: i32,
    #[serde(default)]
    intelligence: i32,
    #[serde(default)]
    wisdom: i32,
    #[serde(default)]
    charisma: i32,
    #[serde(default)]
    hit_points: i32,
    #[serde(default)]
    armor_class: i32,
}

impl FantasyGroundsCharacter {
    fn into_sheet(self) -> CharacterSheet {
        let mut attributes = HashMap::new();
        attributes.insert("strength".to_string(), AttributeValue::Integer(self.strength));
        attributes.insert("dexterity".to_string(), AttributeValue::Integer(self.dexterity));
        attributes.insert("constitution".to_string(), AttributeValue::Integer(self.constitution));
        attributes.insert("intelligence".to_string(), AttributeValue::Integer(self.intelligence));
        attributes.insert("wisdom".to_string(), AttributeValue::Integer(self.wisdom));
        attributes.insert("charisma".to_string(), AttributeValue::Integer(self.charisma));
        attributes.insert("race".to_string(), AttributeValue::String(self.race));
        attributes.insert("class".to_string(), AttributeValue::String(self.class));
        attributes.insert("level".to_string(), AttributeValue::Integer(self.level as i32));
        attributes.insert("hit_points".to_string(), AttributeValue::Integer(self.hit_points));
        attributes.insert("armor_class".to_string(), AttributeValue::Integer(self.armor_class));

        CharacterSheet {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            player: String::new(),
            system_id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            attributes,
            metadata: HashMap::new(),
        }
    }
}

impl TryFrom<FantasyGroundsCharacter> for Character {
    type Error = Error;

    fn try_from(fg: FantasyGroundsCharacter) -> Result<Self, Self::Error> {
        let sheet = fg.into_sheet();
        Ok(sheet_to_character(sheet))
    }
}

impl TryFrom<crate::export::OgcCharacter> for Character {
    type Error = Error;

    fn try_from(ogc: crate::export::OgcCharacter) -> Result<Self, Self::Error> {
        let sheet = ogc.into_sheet();
        Ok(sheet_to_character(sheet))
    }
}

impl crate::export::OgcCharacter {
    fn into_sheet(self) -> CharacterSheet {
        let mut attributes = HashMap::new();
        
        for (key, stat) in self.statistics {
            attributes.insert(key, AttributeValue::Integer(stat.value));
        }

        attributes.insert("level".to_string(), AttributeValue::Integer(self.progression.level as i32));

        CharacterSheet {
            id: uuid::Uuid::new_v4(),
            name: self.identity.name,
            player: self.identity.player_name.unwrap_or_default(),
            system_id: uuid::Uuid::parse_str(&self.system.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            attributes,
            metadata: self.metadata.unwrap_or_default(),
        }
    }
}
