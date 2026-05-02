use oa_core::AttributeValue;
use std::collections::HashMap;
use super::data::*;

pub fn validate_character(attributes: &HashMap<String, AttributeValue>) -> Vec<String> {
    let mut errors = Vec::new();

    let required = [
        "archetype", "species", "level", "body", "mind", "spirit", "speed", "defense", "health",
        "mana", "luck",
    ];
    for attr in required.iter() {
        if !attributes.contains_key(*attr) {
            errors.push(format!("Missing required attribute: {}", attr));
        }
    }

    let archetype_name = get_string(attributes, "archetype");
    let species_name = get_string(attributes, "species");
    let level = get_int(attributes, "level").unwrap_or(1) as u32;

    if archetype_name.is_empty() {
        errors.push("Archetype is required".to_string());
    }

    if species_name.is_empty() {
        errors.push("Species is required".to_string());
    }

    if !archetype_name.is_empty() && Archetype::from_str(&archetype_name).is_none() {
        errors.push(format!("Unknown archetype: {}", archetype_name));
    }

    if !species_name.is_empty() && Species::from_str(&species_name).is_none() {
        errors.push(format!("Unknown species: {}", species_name));
    }

    if level < 1 || level > max_level() {
        errors.push(format!("Level must be between 1 and {}", max_level()));
    }

    for attr in attribute_names() {
        if let Some(val) = get_int(attributes, attr) {
            if let Some((min, max)) = attribute_min_max(attr) {
                if val < min {
                    errors.push(format!("{} below minimum ({})", attr, min));
                }
                if val > max {
                    errors.push(format!("{} above maximum ({})", attr, max));
                }
            }
        }
    }

    let mut total_skill_points = 0;
    for skill in all_skills() {
        let key = format!("skill_{}", skill);
        if let Some(val) = get_int(attributes, &key) {
            if val < 0 {
                errors.push(format!("Skill {} cannot be negative", skill));
            }
            if val > 10 {
                errors.push(format!("Skill {} above maximum (10)", skill));
            }
            total_skill_points += val;
        }
    }

    let expected_budget = skill_point_budget(level) as i32;
    if total_skill_points > expected_budget + 5 {
        errors.push(format!(
            "Total skill points ({}) exceed expected budget for level {}",
            total_skill_points, level
        ));
    }

    errors
}

fn get_string(attributes: &HashMap<String, AttributeValue>, key: &str) -> String {
    match attributes.get(key) {
        Some(AttributeValue::String(s)) => s.clone(),
        _ => String::new(),
    }
}

fn get_int(attributes: &HashMap<String, AttributeValue>, key: &str) -> Option<i32> {
    match attributes.get(key) {
        Some(AttributeValue::Integer(i)) => Some(*i),
        Some(AttributeValue::Float(f)) => Some(*f as i32),
        _ => None,
    }
}
