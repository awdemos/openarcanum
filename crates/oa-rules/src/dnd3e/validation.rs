use oa_core::AttributeValue;
use std::collections::HashMap;
use super::data::*;
use super::generation;

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

    let required_attrs = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];
    for attr in required_attrs.iter() {
        if !attributes.contains_key(*attr) {
            errors.push(format!("Missing required attribute: {}", attr));
        }
    }

    if !race_name.is_empty() && !class_name.is_empty() {
        if let Ok(race) = race_name.parse::<Race>() {
            if let Some(class) = Class::from_str(&class_name) {
                let stats = (
                    get_int(attributes, "strength").unwrap_or(3),
                    get_int(attributes, "dexterity").unwrap_or(3),
                    get_int(attributes, "constitution").unwrap_or(3),
                    get_int(attributes, "intelligence").unwrap_or(3),
                    get_int(attributes, "wisdom").unwrap_or(3),
                    get_int(attributes, "charisma").unwrap_or(3),
                );

                for (i, attr) in ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"].iter().enumerate() {
                    let score = match i {
                        0 => stats.0,
                        1 => stats.1,
                        2 => stats.2,
                        3 => stats.3,
                        4 => stats.4,
                        5 => stats.5,
                        _ => 0,
                    };
                    if score < 3 {
                        errors.push(format!("{} below minimum of 3", attr));
                    }
                    if score > 25 {
                        errors.push(format!("{} above maximum of 25", attr));
                    }
                }

                let expected_bab = generation::calculate_bab(&class, level);
                if let Some(actual) = get_int(attributes, "base_attack_bonus") {
                    if actual != expected_bab {
                        errors.push(format!(
                            "BAB mismatch: expected {} for {} level {}, got {}",
                            expected_bab, class.name(), level, actual
                        ));
                    }
                }

                let expected_saves = generation::calculate_saves(&class, level);
                for (name, expected) in expected_saves.iter() {
                    if let Some(actual) = get_int(attributes, &format!("save_{}", name)) {
                        if actual != *expected {
                            errors.push(format!(
                                "Save {} mismatch: expected {}, got {}",
                                name, expected, actual
                            ));
                        }
                    }
                }

                if class.casts_spells() {
                    let expected_slots = generation::calculate_spell_slots(&class, level);
                    if let Some(AttributeValue::List(slots)) = attributes.get("spell_slots") {
                        if let Some(expected) = expected_slots.get("spell_slots") {
                            if slots.len() != expected.len() {
                                errors.push(format!(
                                    "Spell slot count mismatch: expected {}, got {}",
                                    expected.len(), slots.len()
                                ));
                            } else {
                                for (i, (expected_val, actual)) in expected.iter().zip(slots.iter()).enumerate() {
                                    let actual_val = match actual {
                                        AttributeValue::Integer(v) => *v as u32,
                                        _ => 0,
                                    };
                                    if actual_val != *expected_val {
                                        errors.push(format!(
                                            "Spell level {} slot mismatch: expected {}, got {}",
                                            i + 1, expected_val, actual_val
                                        ));
                                    }
                                }
                            }
                        }
                    } else if level > 0 {
                        let expected_slots_check = generation::calculate_spell_slots(&class, level);
                        if expected_slots_check.get("spell_slots").map(|s| !s.is_empty()).unwrap_or(false) {
                            errors.push("Missing spell_slots attribute".to_string());
                        }
                    }
                }

                if let Some(AttributeValue::Map(skills)) = attributes.get("skills") {
                    let max_rank = max_class_skill_rank(level);
                    let cross_max = max_cross_class_skill_rank(level);
                    let class_skills = class.class_skills();

                    for (skill_name, value) in skills.iter() {
                        if skill_name == "skill_points_total" || skill_name == "skill_points_remaining" {
                            continue;
                        }
                        let rank = match value {
                            AttributeValue::Integer(v) => *v,
                            _ => 0,
                        };
                        let is_class = class_skills.contains(&skill_name.as_str());
                        let max_allowed = if is_class { max_rank } else { cross_max };
                        if rank > max_allowed {
                            errors.push(format!(
                                "Skill {} rank {} exceeds maximum {} (class skill: {})",
                                skill_name, rank, max_allowed, is_class
                            ));
                        }
                        if rank < 0 {
                            errors.push(format!("Skill {} rank {} is negative", skill_name, rank));
                        }
                    }
                }

                if let Some(AttributeValue::List(feats)) = attributes.get("feats") {
                    let expected_count = if class.bonus_feats() {
                        normal_feat_count(level) + fighter_bonus_feat_count(level)
                    } else {
                        normal_feat_count(level)
                    };
                    if feats.len() != expected_count as usize {
                        errors.push(format!(
                            "Feat count mismatch: expected {}, got {}",
                            expected_count, feats.len()
                        ));
                    }
                }

                if let Some(hp) = get_int(attributes, "hit_points") {
                    let con_mod = ability_modifier(stats.2);
                    let hit_die = class.hit_die();
                    let min_hp = first_level_hp(hit_die, con_mod) + ((level.saturating_sub(1)) as i32 * (1 + con_mod).max(1));
                    let max_hp = first_level_hp(hit_die, con_mod) + ((level.saturating_sub(1)) as i32 * (hit_die as i32 + con_mod));
                    if hp < min_hp || hp > max_hp {
                        errors.push(format!(
                            "HP {} out of range {}-{} for {} level {} CON {}",
                            hp, min_hp, max_hp, class.name(), level, stats.2
                        ));
                    }
                }

                if let Some(ac) = get_int(attributes, "armor_class") {
                    let expected_ac = generation::calculate_base_ac(ability_modifier(stats.1), race.size().modifier());
                    if ac != expected_ac {
                        errors.push(format!(
                            "AC mismatch: expected {}, got {}",
                            expected_ac, ac
                        ));
                    }
                }

                let alignment_str = get_string(attributes, "alignment");
                if !alignment_str.is_empty() {
                    if let Some(alignment) = Alignment::from_str(&alignment_str) {
                        if !generation::meets_class_alignment_requirements(&class, &alignment) {
                            errors.push(format!(
                                "Alignment {} does not meet {} requirements",
                                alignment.name(), class.name()
                            ));
                        }
                    } else {
                        errors.push(format!("Unknown alignment: {}", alignment_str));
                    }
                }
            } else {
                errors.push(format!("Unknown class: {}", class_name));
            }
        } else {
            errors.push(format!("Unknown race: {}", race_name));
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
