use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Character, CharacterSheet, Ability, Inventory, Item};

/// Open Game Content (OGC) character format.
/// 
/// Standardized, system-agnostic representation of RPG character data
/// for interoperability between tools and platforms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OgcCharacter {
    pub format_version: String,
    pub identity: OgcIdentity,
    pub system: OgcSystem,
    pub statistics: HashMap<String, OgcStatistic>,
    pub progression: OgcProgression,
    pub combat: OgcCombat,
    pub skills: Vec<OgcSkill>,
    pub features: Vec<OgcFeature>,
    pub equipment: Vec<OgcEquipment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spellcasting: Option<OgcSpellcasting>,
    pub background: OgcBackground,
    pub license: OgcLicense,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcIdentity {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portrait_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcSystem {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcStatistic {
    pub name: String,
    pub abbreviation: String,
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_value: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcProgression {
    pub level: u32,
    pub experience_points: u64,
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
    pub race: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcCombat {
    pub hit_points: i32,
    pub max_hit_points: i32,
    pub armor_class: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thac0: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiative_bonus: Option<i32>,
    pub saving_throws: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcSkill {
    pub name: String,
    pub rank: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcFeature {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OgcEquipment {
    pub name: String,
    pub quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcSpellcasting {
    pub ability: String,
    pub spell_slots: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_spells: Option<Vec<OgcSpell>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_spells: Option<Vec<OgcSpell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcSpell {
    pub name: String,
    pub level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcBackground {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideals: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flaws: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct OgcLicense {
    pub name: String,
    pub url: String,
    pub attribution: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Yaml,
    Markdown,
    Ogc,
}

impl std::str::FromStr for ExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ExportFormat::Json),
            "yaml" | "yml" => Ok(ExportFormat::Yaml),
            "markdown" | "md" => Ok(ExportFormat::Markdown),
            "ogc" | "open" | "ogl" | "srd" => Ok(ExportFormat::Ogc),
            _ => Err(format!("Unknown export format: {}", s)),
        }
    }
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Json => "json",
            ExportFormat::Yaml => "yaml",
            ExportFormat::Markdown => "md",
            ExportFormat::Ogc => "ogc.json",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Json => "application/json",
            ExportFormat::Yaml => "application/yaml",
            ExportFormat::Markdown => "text/markdown",
            ExportFormat::Ogc => "application/json",
        }
    }
}

pub fn character_to_ogc(character: &Character) -> OgcCharacter {
    let mut statistics = HashMap::new();
    
    for attr in &character.attributes {
        let value = attr.computed_value();
        let base = attr.base_value;
        
        statistics.insert(
            attr.definition_id.to_string(),
            OgcStatistic {
                name: attr.definition_id.to_string(),
                abbreviation: attr.definition_id.to_string(),
                value,
                modifier: Some(value - base),
                base_value: Some(base),
                notes: None,
            },
        );
    }

    let mut combat = OgcCombat {
        hit_points: 0,
        max_hit_points: 0,
        armor_class: 10,
        thac0: None,
        attack_bonus: None,
        initiative_bonus: None,
        saving_throws: HashMap::new(),
    };

    if let Some(serde_json::Value::Number(hp)) = character.metadata.get("hit_points") {
        if let Some(hp_i) = hp.as_i64() {
            combat.hit_points = hp_i as i32;
            combat.max_hit_points = hp_i as i32;
        }
    }
    if let Some(serde_json::Value::Number(ac)) = character.metadata.get("armor_class") {
        if let Some(ac_i) = ac.as_i64() {
            combat.armor_class = ac_i as i32;
        }
    }
    if let Some(serde_json::Value::Number(thac0)) = character.metadata.get("thac0") {
        if let Some(t) = thac0.as_i64() {
            combat.thac0 = Some(t as i32);
        }
    }
    if let Some(serde_json::Value::Number(init)) = character.metadata.get("initiative") {
        if let Some(i) = init.as_i64() {
            combat.initiative_bonus = Some(i as i32);
        }
    }

    for (key, value) in &character.metadata {
        if let Some(save_name) = key.strip_prefix("save_") {
            if let serde_json::Value::Number(v) = value {
                if let Some(v_i) = v.as_i64() {
                    combat.saving_throws.insert(save_name.to_string(), v_i as i32);
                }
            }
        }
    }

    let mut features = Vec::new();
    for ability in &character.abilities {
        features.push(OgcFeature {
            name: ability.name.clone(),
            description: Some(ability.description.clone()),
            source: "character".to_string(),
        });
    }

    let mut equipment = Vec::new();
    for item in &character.inventory.items {
        equipment.push(ogc_equipment_from_item(item));
    }

    let spellcasting = extract_spellcasting_json(&character.metadata);

    let (class, subclass, race, alignment) = extract_progression_info(character);

    OgcCharacter {
        format_version: "1.0.0".to_string(),
        identity: OgcIdentity {
            name: character.identity.name.clone(),
            player_name: if character.identity.player_name.is_empty() {
                None
            } else {
                Some(character.identity.player_name.clone())
            },
            portrait_url: character.identity.portrait_url.clone(),
        },
        system: OgcSystem {
            id: character.system_id.to_string(),
            name: "Open Arcanum".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            edition: Some("2nd".to_string()),
        },
        statistics,
        progression: OgcProgression {
            level: character.level,
            experience_points: character.progression.experience_points as u64,
            class,
            subclass,
            race,
            alignment,
        },
        combat,
        skills: Vec::new(),
        features,
        equipment,
        spellcasting,
        background: OgcBackground {
            description: Some(character.identity.background.clone()),
            personality: None,
            ideals: None,
            bonds: None,
            flaws: None,
            notes: Some(character.identity.description.clone()),
        },
        license: OgcLicense {
            name: "Open Game License".to_string(),
            url: "https://www.wizards.com/d20/oglfaq.html".to_string(),
            attribution: "Generated by Open Arcanum".to_string(),
        },
        metadata: Some(character.metadata.clone()),
    }
}

pub fn character_sheet_to_ogc(sheet: &CharacterSheet) -> OgcCharacter {
    let mut statistics = HashMap::new();
    
    for (key, value) in &sheet.attributes {
        let (name, val) = match value {
            crate::AttributeValue::Integer(i) => (key.clone(), *i),
            crate::AttributeValue::String(s) => (key.clone(), s.len() as i32),
            _ => (key.clone(), 0),
        };
        
        statistics.insert(
            key.clone(),
            OgcStatistic {
                name: name.clone(),
                abbreviation: key.clone(),
                value: val,
                modifier: None,
                base_value: None,
                notes: None,
            },
        );
    }

    let mut combat = OgcCombat {
        hit_points: 0,
        max_hit_points: 0,
        armor_class: 10,
        thac0: None,
        attack_bonus: None,
        initiative_bonus: None,
        saving_throws: HashMap::new(),
    };

    if let Some(crate::AttributeValue::Integer(hp)) = sheet.attributes.get("hit_points") {
        combat.hit_points = *hp;
        combat.max_hit_points = *hp;
    }
    if let Some(crate::AttributeValue::Integer(ac)) = sheet.attributes.get("armor_class") {
        combat.armor_class = *ac;
    }
    if let Some(crate::AttributeValue::Integer(thac0)) = sheet.attributes.get("thac0") {
        combat.thac0 = Some(*thac0);
    }

    for (key, value) in &sheet.attributes {
        if let Some(save_name) = key.strip_prefix("save_") {
            if let crate::AttributeValue::Integer(v) = value {
                combat.saving_throws.insert(save_name.to_string(), *v);
            }
        }
    }

    let spellcasting = extract_spellcasting_attrs(&sheet.attributes);

    let race = sheet.attributes.get("race")
        .and_then(|v| match v {
            crate::AttributeValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let class = sheet.attributes.get("class")
        .and_then(|v| match v {
            crate::AttributeValue::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let level = sheet.attributes.get("level")
        .and_then(|v| match v {
            crate::AttributeValue::Integer(i) => Some(*i as u32),
            _ => None,
        })
        .unwrap_or(1);

    OgcCharacter {
        format_version: "1.0.0".to_string(),
        identity: OgcIdentity {
            name: sheet.name.clone(),
            player_name: if sheet.player.is_empty() { None } else { Some(sheet.player.clone()) },
            portrait_url: None,
        },
        system: OgcSystem {
            id: sheet.system_id.to_string(),
            name: "Open Arcanum".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            edition: None,
        },
        statistics,
        progression: OgcProgression {
            level,
            experience_points: 0,
            class,
            subclass: None,
            race,
            alignment: None,
        },
        combat,
        skills: Vec::new(),
        features: Vec::new(),
        equipment: Vec::new(),
        spellcasting,
        background: OgcBackground {
            description: None,
            personality: None,
            ideals: None,
            bonds: None,
            flaws: None,
            notes: None,
        },
        license: OgcLicense {
            name: "Open Game License".to_string(),
            url: "https://www.wizards.com/d20/oglfaq.html".to_string(),
            attribution: "Generated by Open Arcanum".to_string(),
        },
        metadata: Some(sheet.metadata.clone()),
    }
}

pub fn character_to_markdown(character: &Character) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", character.identity.name));
    md.push_str(&format!("**Player:** {}\n\n", character.identity.player_name));
    md.push_str(&format!("**Level:** {}\n\n", character.level));
    
    let (class, _, race, alignment) = extract_progression_info(character);
    md.push_str(&format!("**Race:** {}\n\n", race));
    md.push_str(&format!("**Class:** {}\n\n", class));
    if let Some(ref align) = alignment {
        md.push_str(&format!("**Alignment:** {}\n\n", align));
    }
    md.push_str(&format!("**Background:** {}\n\n", character.identity.background));

    if !character.attributes.is_empty() {
        md.push_str("## Attributes\n\n");
        md.push_str("| Attribute | Base | Computed |\n");
        md.push_str("|-----------|------|----------|\n");
        for attr in &character.attributes {
            md.push_str(&format!(
                "| {} | {} | {} |\n",
                attr.name,
                attr.base_value,
                attr.computed_value()
            ));
        }
        md.push('\n');
    }

    if !character.abilities.is_empty() {
        md.push_str("## Abilities\n\n");
        for ability in &character.abilities {
            md.push_str(&format!("- **{}:** {}\n", ability.name, ability.description));
        }
        md.push('\n');
    }

    if !character.inventory.items.is_empty() {
        md.push_str("## Inventory\n\n");
        md.push_str("| Item | Weight | Value | Equipped |\n");
        md.push_str("|------|--------|-------|----------|\n");
        for item in &character.inventory.items {
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                item.name,
                item.weight,
                item.value,
                if item.equipped { "Yes" } else { "No" }
            ));
        }
        md.push('\n');
    }

    if !character.metadata.is_empty() {
        md.push_str("## Metadata\n\n");
        for (key, value) in &character.metadata {
            md.push_str(&format!("- **{}:** {}\n", key, value));
        }
    }

    md
}

fn format_attribute_value(value: &crate::AttributeValue) -> String {
    match value {
        crate::AttributeValue::Integer(i) => i.to_string(),
        crate::AttributeValue::Float(f) => format!("{:.1}", f),
        crate::AttributeValue::String(s) => s.clone(),
        crate::AttributeValue::Boolean(b) => b.to_string(),
        crate::AttributeValue::List(items) => {
            items.iter()
                .map(format_attribute_value)
                .collect::<Vec<_>>()
                .join(", ")
        }
        crate::AttributeValue::Map(map) => {
            let pairs: Vec<String> = map.iter()
                .map(|(k, v)| format!("{}: {}", k, format_attribute_value(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

pub fn character_sheet_to_markdown(sheet: &CharacterSheet) -> String {
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", sheet.name));
    if !sheet.player.is_empty() {
        md.push_str(&format!("**Player:** {}\n\n", sheet.player));
    }

    let race = sheet.attributes.get("race")
        .and_then(|v| match v {
            crate::AttributeValue::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("Unknown");
    let class = sheet.attributes.get("class")
        .and_then(|v| match v {
            crate::AttributeValue::String(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("Unknown");
    let level = sheet.attributes.get("level")
        .and_then(|v| match v {
            crate::AttributeValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(1);

    md.push_str(&format!("**Race:** {}\n\n", race));
    md.push_str(&format!("**Class:** {}\n\n", class));
    md.push_str(&format!("**Level:** {}\n\n", level));

    if !sheet.attributes.is_empty() {
        md.push_str("## Attributes\n\n");
        md.push_str("| Attribute | Value |\n");
        md.push_str("|-----------|-------|\n");
        for (key, value) in &sheet.attributes {
            let val_str = format_attribute_value(value);
            md.push_str(&format!("| {} | {} |\n", key, val_str));
        }
        md.push('\n');
    }

    if !sheet.metadata.is_empty() {
        md.push_str("## Metadata\n\n");
        for (key, value) in &sheet.metadata {
            md.push_str(&format!("- **{}:** {}\n", key, value));
        }
    }

    md
}

pub fn export_character(character: &Character, format: ExportFormat) -> Result<String, crate::Error> {
    match format {
        ExportFormat::Json => {
            serde_json::to_string_pretty(character)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
        ExportFormat::Yaml => {
            serde_yaml::to_string(character)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
        ExportFormat::Markdown => Ok(character_to_markdown(character)),
        ExportFormat::Ogc => {
            let ogc = character_to_ogc(character);
            serde_json::to_string_pretty(&ogc)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
    }
}

pub fn export_character_sheet(sheet: &CharacterSheet, format: ExportFormat) -> Result<String, crate::Error> {
    match format {
        ExportFormat::Json => {
            serde_json::to_string_pretty(sheet)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
        ExportFormat::Yaml => {
            serde_yaml::to_string(sheet)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
        ExportFormat::Markdown => Ok(character_sheet_to_markdown(sheet)),
        ExportFormat::Ogc => {
            let ogc = character_sheet_to_ogc(sheet);
            serde_json::to_string_pretty(&ogc)
                .map_err(|e| crate::Error::Serialization(e.to_string()))
        }
    }
}

fn extract_progression_info(character: &Character) -> (String, Option<String>, String, Option<String>) {
    let class = character.progression.classes.first()
        .map(|c| c.class_name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let subclass = character.progression.classes.first()
        .and_then(|c| c.subclass.clone());
    
    let race = character.metadata.get("race")
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "Unknown".to_string());
    
    let alignment = character.metadata.get("alignment")
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s.clone()),
            _ => None,
        });
    
    (class, subclass, race, alignment)
}

fn extract_spellcasting_json(metadata: &HashMap<String, serde_json::Value>) -> Option<OgcSpellcasting> {
    if let Some(serde_json::Value::Array(slots)) = metadata.get("spell_slots") {
        let spell_slots: Vec<u32> = slots.iter()
            .filter_map(|v| match v {
                serde_json::Value::Number(n) => n.as_u64().map(|u| u as u32),
                _ => None,
            })
            .collect();
        
        Some(OgcSpellcasting {
            ability: "intelligence".to_string(),
            spell_slots,
            known_spells: None,
            prepared_spells: None,
        })
    } else {
        None
    }
}

fn extract_spellcasting_attrs(metadata: &HashMap<String, crate::AttributeValue>) -> Option<OgcSpellcasting> {
    if let Some(crate::AttributeValue::List(slots)) = metadata.get("spell_slots") {
        let spell_slots: Vec<u32> = slots.iter()
            .filter_map(|v| match v {
                crate::AttributeValue::Integer(i) => Some(*i as u32),
                _ => None,
            })
            .collect();
        
        Some(OgcSpellcasting {
            ability: "intelligence".to_string(),
            spell_slots,
            known_spells: None,
            prepared_spells: None,
        })
    } else {
        None
    }
}

fn ogc_equipment_from_item(item: &Item) -> OgcEquipment {
    OgcEquipment {
        name: item.name.clone(),
        quantity: 1,
        weight: Some(item.weight as f64),
        value: Some(item.value as f64),
        equipped: Some(item.equipped),
        notes: None,
    }
}
