use oa_core::AttributeValue;
use std::collections::HashMap;
use super::data::*;

pub fn validate_character(attributes: &HashMap<String, AttributeValue>) -> Vec<String> {
    let mut errors = Vec::new();

    let race_name = get_string(attributes, "race");
    let class_name = get_string(attributes, "class");
    let background_name = get_string(attributes, "background");
    let level = get_int(attributes, "level").unwrap_or(1) as u32;

    if race_name.is_empty() {
        errors.push("Race is required".to_string());
    }
    if class_name.is_empty() {
        errors.push("Class is required".to_string());
    }
    if background_name.is_empty() {
        errors.push("Background is required".to_string());
    }

    let stats = (
        get_int(attributes, "strength").unwrap_or(10),
        get_int(attributes, "dexterity").unwrap_or(10),
        get_int(attributes, "constitution").unwrap_or(10),
        get_int(attributes, "intelligence").unwrap_or(10),
        get_int(attributes, "wisdom").unwrap_or(10),
        get_int(attributes, "charisma").unwrap_or(10),
    );

    for (name, value) in [
        ("strength", stats.0),
        ("dexterity", stats.1),
        ("constitution", stats.2),
        ("intelligence", stats.3),
        ("wisdom", stats.4),
        ("charisma", stats.5),
    ]
    .iter()
    {
        if *value < 1 || *value > 30 {
            errors.push(format!("{} must be between 1 and 30, got {}", name, value));
        }
    }

    if !race_name.is_empty() {
        if race_name.parse::<Race>().is_err() {
            errors.push(format!("Unknown race: {}", race_name));
        }
    }

    if !class_name.is_empty() {
        if let Some(class) = Class::from_str(&class_name) {
            let expected_saves = calculate_expected_saves(&class, stats, level);
            for (save_name, expected) in expected_saves.iter() {
                if let Some(actual) = get_int(attributes, save_name) {
                    if actual != *expected {
                        errors.push(format!(
                            "Save {} mismatch: expected {}, got {}",
                            save_name, expected, actual
                        ));
                    }
                }
            }

            if class.casts_spells() {
                let expected_slots = match class.spell_progression() {
                    SpellProgression::Full => full_caster_spell_slots(level),
                    SpellProgression::Half => half_caster_spell_slots(level),
                    SpellProgression::PactMagic => warlock_spell_slots(level),
                    SpellProgression::None => vec![],
                };

                if let Some(AttributeValue::List(slots)) = attributes.get("spell_slots") {
                    if slots.len() != expected_slots.len() {
                        errors.push(format!(
                            "Spell slot count mismatch: expected {}, got {}",
                            expected_slots.len(),
                            slots.len()
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
                                    i + 1,
                                    expected,
                                    actual_val
                                ));
                            }
                        }
                    }
                } else if !expected_slots.is_empty() && expected_slots.iter().any(|&s| s > 0) {
                    errors.push("Missing spell_slots attribute".to_string());
                }
            }

            if let Some(hp) = get_int(attributes, "hit_points") {
                let con_mod = ability_modifier(stats.2);
                let hit_die = class.hit_die() as i32;
                let min_hp = hit_die + con_mod + ((level.saturating_sub(1)) as i32 * (1 + con_mod)).max(0);
                let max_hp = hit_die + con_mod + ((level.saturating_sub(1)) as i32 * (hit_die + con_mod));
                let clamped_min = min_hp.max(1);
                let clamped_max = max_hp.max(clamped_min);
                if hp < clamped_min || hp > clamped_max {
                    errors.push(format!(
                        "HP {} out of range {}-{} for {} level {} CON {}",
                        hp, clamped_min, clamped_max, class.name(), level, stats.2
                    ));
                }
            }

            let expected_prof = proficiency_bonus(level);
            if let Some(actual) = get_int(attributes, "proficiency_bonus") {
                if actual != expected_prof {
                    errors.push(format!(
                        "Proficiency bonus mismatch: expected {}, got {}",
                        expected_prof, actual
                    ));
                }
            }
        } else {
            errors.push(format!("Unknown class: {}", class_name));
        }
    }

    if !background_name.is_empty() {
        if Background::from_str(&background_name).is_none() {
            errors.push(format!("Unknown background: {}", background_name));
        }
    }

    let required_attrs = [
        "strength", "dexterity", "constitution",
        "intelligence", "wisdom", "charisma",
    ];
    for attr in required_attrs.iter() {
        if !attributes.contains_key(*attr) {
            errors.push(format!("Missing required attribute: {}", attr));
        }
    }

    errors
}

fn calculate_expected_saves(
    class: &Class,
    stats: (i32, i32, i32, i32, i32, i32),
    level: u32,
) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let prof = proficiency_bonus(level);
    let (save1, save2) = class.saving_throw_proficiencies();
    let abilities = [
        ("strength", stats.0),
        ("dexterity", stats.1),
        ("constitution", stats.2),
        ("intelligence", stats.3),
        ("wisdom", stats.4),
        ("charisma", stats.5),
    ];

    for (name, score) in abilities.iter() {
        let mut value = ability_modifier(*score);
        if *name == save1 || *name == save2 {
            value += prof;
        }
        map.insert(format!("save_{}", name), value);
    }

    map
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
