use oa_core::AttributeValue;
use std::collections::HashMap;
use super::data::*;
use super::generation::race_class_compatible;

pub fn validate_character(attributes: &HashMap<String, AttributeValue>) -> Vec<String> {
    let mut errors = Vec::new();

    let race_name = get_string(attributes, "race");
    let class_name = get_string(attributes, "class");
    let level = get_int(attributes, "level").unwrap_or(1) as u32;

    if race_name.is_empty() {
        errors.push("Race is required".to_string());
    }
    if class_name.is_empty() {
        errors.push("Class is required".to_string());
    }

    if !race_name.is_empty() && !class_name.is_empty() {
        if let Ok(race) = race_name.parse::<Race>() {
            if let Some(class) = Class::from_str(&class_name) {
                if !race_class_compatible(&race, &class) {
                    errors.push(format!("{} cannot be {}", race.name(), class.name()));
                }

                let allowed = race.allowed_classes();
                if let Some(Some(max)) = allowed.get(class.name()).copied() {
                    if level > max {
                        errors.push(format!(
                            "{} maximum level for {} is {}",
                            race.name(), class.name(), max
                        ));
                    }
                }

                let stats = (
                    get_int(attributes, "strength").unwrap_or(3),
                    get_int(attributes, "dexterity").unwrap_or(3),
                    get_int(attributes, "constitution").unwrap_or(3),
                    get_int(attributes, "intelligence").unwrap_or(3),
                    get_int(attributes, "wisdom").unwrap_or(3),
                    get_int(attributes, "charisma").unwrap_or(3),
                );

                let race_min = race.minimum_abilities();
                if stats.0 < race_min.0 { errors.push(format!("STR below {} minimum ({})", race.name(), race_min.0)); }
                if stats.1 < race_min.1 { errors.push(format!("DEX below {} minimum ({})", race.name(), race_min.1)); }
                if stats.2 < race_min.2 { errors.push(format!("CON below {} minimum ({})", race.name(), race_min.2)); }
                if stats.3 < race_min.3 { errors.push(format!("INT below {} minimum ({})", race.name(), race_min.3)); }
                if stats.4 < race_min.4 { errors.push(format!("WIS below {} minimum ({})", race.name(), race_min.4)); }
                if stats.5 < race_min.5 { errors.push(format!("CHA below {} minimum ({})", race.name(), race_min.5)); }

                let race_max = race.maximum_abilities();
                if stats.0 > race_max.0 { errors.push(format!("STR above {} maximum ({})", race.name(), race_max.0)); }
                if stats.1 > race_max.1 { errors.push(format!("DEX above {} maximum ({})", race.name(), race_max.1)); }
                if stats.2 > race_max.2 { errors.push(format!("CON above {} maximum ({})", race.name(), race_max.2)); }
                if stats.3 > race_max.3 { errors.push(format!("INT above {} maximum ({})", race.name(), race_max.3)); }
                if stats.4 > race_max.4 { errors.push(format!("WIS above {} maximum ({})", race.name(), race_max.4)); }
                if stats.5 > race_max.5 { errors.push(format!("CHA above {} maximum ({})", race.name(), race_max.5)); }

                let class_req = class.ability_requirements();
                if stats.0 < class_req.0 { errors.push(format!("STR below {} requirement ({})", class.name(), class_req.0)); }
                if stats.1 < class_req.1 { errors.push(format!("DEX below {} requirement ({})", class.name(), class_req.1)); }
                if stats.2 < class_req.2 { errors.push(format!("CON below {} requirement ({})", class.name(), class_req.2)); }
                if stats.3 < class_req.3 { errors.push(format!("INT below {} requirement ({})", class.name(), class_req.3)); }
                if stats.4 < class_req.4 { errors.push(format!("WIS below {} requirement ({})", class.name(), class_req.4)); }
                if stats.5 < class_req.5 { errors.push(format!("CHA below {} requirement ({})", class.name(), class_req.5)); }

                let expected_thac0 = thac0(class.group(), level);
                if let Some(actual) = get_int(attributes, "thac0") {
                    if actual != expected_thac0 {
                        errors.push(format!(
                            "THAC0 mismatch: expected {} for {} level {}, got {}",
                            expected_thac0, class.name(), level, actual
                        ));
                    }
                }

                let expected_saves = saving_throws(class.group(), level);
                let save_names = [
                    ("save_paralyze", expected_saves.0),
                    ("save_poison", expected_saves.0),
                    ("save_death", expected_saves.0),
                    ("save_rod", expected_saves.1),
                    ("save_staff", expected_saves.1),
                    ("save_wand", expected_saves.1),
                    ("save_petrification", expected_saves.2),
                    ("save_polymorph", expected_saves.2),
                    ("save_breath_weapon", expected_saves.3),
                    ("save_spell", expected_saves.4),
                ];
                for (name, expected) in save_names.iter() {
                    if let Some(actual) = get_int(attributes, name) {
                        if actual != *expected {
                            errors.push(format!(
                                "Save {} mismatch: expected {}, got {}",
                                name, expected, actual
                            ));
                        }
                    }
                }

                if class.casts_spells() {
                    let expected_slots = match class.spell_type() {
                        Some(SpellType::Arcane) => arcane_spell_slots(level),
                        Some(SpellType::Divine) => divine_spell_slots(level),
                        Some(SpellType::Bardic) => bard_spell_slots(level),
                        None => vec![],
                    };

                    if let Some(AttributeValue::List(slots)) = attributes.get("spell_slots") {
                        if slots.len() != expected_slots.len() {
                            errors.push(format!(
                                "Spell slot count mismatch: expected {}, got {}",
                                expected_slots.len(), slots.len()
                            ));
                        } else {
                            for (i, (expected, actual)) in expected_slots.iter().zip(slots.iter()).enumerate() {
                                let actual_val = match actual {
                                    AttributeValue::Integer(v) => *v as u32,
                                    _ => 0,
                                };
                                if actual_val != *expected {
                                    errors.push(format!(
                                        "Spell level {} slot mismatch: expected {}, got {}",
                                        i + 1, expected, actual_val
                                    ));
                                }
                            }
                        }
                    } else if !expected_slots.is_empty() && expected_slots.iter().any(|&s| s > 0) {
                        errors.push("Missing spell_slots attribute".to_string());
                    }
                }

                if let Some(hp) = get_int(attributes, "hit_points") {
                    let con_bonus = constitution_table(stats.2).0;
                    let warrior_bonus = if class.group() == ClassGroup::Warrior {
                        warrior_hp_bonus(stats.2)
                    } else {
                        0
                    };
                    let min_hp = level as i32 * (1 + con_bonus + warrior_bonus).max(1);
                    let max_hp = level as i32 * (class.hit_die() as i32 + con_bonus + warrior_bonus);
                    if hp < min_hp || hp > max_hp {
                        errors.push(format!(
                            "HP {} out of range {}-{} for {} level {} CON {}",
                            hp, min_hp, max_hp, class.name(), level, stats.2
                        ));
                    }
                }
            } else {
                errors.push(format!("Unknown class: {}", class_name));
            }
        } else {
            errors.push(format!("Unknown race: {}", race_name));
        }
    }

    let required_attrs = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];
    for attr in required_attrs.iter() {
        if !attributes.contains_key(*attr) {
            errors.push(format!("Missing required attribute: {}", attr));
        }
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
