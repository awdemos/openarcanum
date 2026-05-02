use rand::Rng;
use oa_core::AttributeValue;
use std::collections::HashMap;
use super::data::*;

pub fn roll_2d6_plus_6() -> i32 {
    let mut rng = rand::thread_rng();
    (0..2).map(|_| rng.gen_range(1..=6)).sum::<i32>() + 6
}

pub fn roll_stats() -> HashMap<&'static str, i32> {
    let mut stats = HashMap::new();
    for attr in core_attribute_names() {
        stats.insert(attr, roll_2d6_plus_6());
    }
    stats
}

pub fn point_buy_stats(budget: u32) -> HashMap<&'static str, i32> {
    let base = 8;
    let mut stats = HashMap::new();
    for attr in core_attribute_names() {
        stats.insert(attr, base);
    }

    let mut remaining = budget;
    let attrs: Vec<&str> = core_attribute_names();
    let mut rng = rand::thread_rng();

    while remaining > 0 {
        let attr = attrs[rng.gen_range(0..attrs.len())];
        let current = *stats.get(attr).unwrap_or(&base);
        if current < 18 {
            let cost = if current >= 14 { 2 } else { 1 };
            if remaining >= cost {
                if let Some(val) = stats.get_mut(attr) {
                    *val = current + 1;
                }
                remaining -= cost;
            } else {
                break;
            }
        }
    }

    stats
}

pub fn apply_species_modifiers(
    species: &Species,
    stats: &mut HashMap<String, i32>,
) {
    let mods = species.attribute_modifiers();
    for (attr, bonus) in mods {
        if let Some(val) = stats.get_mut(attr) {
            *val = (*val + bonus).clamp(1, 20);
        }
    }
}

pub fn calculate_derived_attributes(
    stats: &HashMap<String, i32>,
    archetype: &Archetype,
    level: u32,
) -> HashMap<String, i32> {
    let mut derived = HashMap::new();

    let body = *stats.get("body").unwrap_or(&10);
    let mind = *stats.get("mind").unwrap_or(&10);
    let spirit = *stats.get("spirit").unwrap_or(&10);
    let speed = *stats.get("speed").unwrap_or(&10);
    let defense = *stats.get("defense").unwrap_or(&10);

    let hit_die = archetype.hit_die() as i32;
    let health_bonus = (body - 10) / 2;
    let health = (hit_die + health_bonus).max(1) * level as i32 + 10;

    let mana_bonus = (spirit - 10) / 2;
    let base_mana = archetype.mana_per_level() * level as i32 + 10;
    let mana = base_mana + mana_bonus * level as i32;

    let initiative = (speed - 10) / 2 + (mind - 10) / 4;

    derived.insert("health".to_string(), health);
    derived.insert("mana".to_string(), mana.max(0));
    derived.insert("initiative".to_string(), initiative);
    derived.insert("defense".to_string(), (10 + defense + (speed - 10) / 2).max(1));

    derived
}

pub fn allocate_skills(
    archetype: &Archetype,
    level: u32,
) -> HashMap<String, i32> {
    let mut skills = HashMap::new();
    let all = all_skills();
    for skill in &all {
        skills.insert(skill.to_string(), 0);
    }

    let mut budget = archetype.starting_skill_points() + level.saturating_sub(1) * 2;
    let mut rng = rand::thread_rng();

    let priorities: Vec<&str> = match archetype {
        Archetype::Warrior => vec!["melee", "athletics", "perception"],
        Archetype::Mage => vec!["magic", "lore", "perception"],
        Archetype::Rogue => vec!["stealth", "melee", "perception"],
        Archetype::Healer => vec!["medicine", "persuasion", "lore"],
        Archetype::Ranger => vec!["ranged", "perception", "athletics"],
        Archetype::Bard => vec!["persuasion", "stealth", "lore"],
        Archetype::Paladin => vec!["melee", "persuasion", "medicine"],
        Archetype::Necromancer => vec!["magic", "lore", "medicine"],
    };

    for skill in &priorities {
        if budget > 0 {
            skills.insert(skill.to_string(), 1);
            budget -= 1;
        }
    }

    let all_skill_names: Vec<String> = all.iter().map(|s| s.to_string()).collect();
    while budget > 0 {
        let skill = &all_skill_names[rng.gen_range(0..all_skill_names.len())];
        let current = *skills.get(skill).unwrap_or(&0);
        if current < 5 {
            skills.insert(skill.clone(), current + 1);
            budget -= 1;
        } else if skills.values().all(|&v| v >= 5) {
            break;
        }
    }

    skills
}

pub fn generate_equipment(archetype: &Archetype) -> Vec<String> {
    equipment_for_archetype(archetype)
        .iter()
        .map(|&s| s.to_string())
        .collect()
}

pub fn generate_character_attributes(
    archetype_name: &str,
    species_name: &str,
    level: u32,
    use_rolled_stats: bool,
    point_buy_budget: Option<u32>,
) -> Result<HashMap<String, AttributeValue>, String> {
    let archetype = Archetype::from_str(archetype_name)
        .ok_or_else(|| format!("Unknown archetype: {}", archetype_name))?;
    let species = Species::from_str(species_name)
        .ok_or_else(|| format!("Unknown species: {}", species_name))?;

    let level = level.max(1).min(max_level());

    let mut core_stats: HashMap<String, i32> = if use_rolled_stats {
        roll_stats().into_iter().map(|(k, v)| (k.to_string(), v)).collect()
    } else {
        let budget = point_buy_budget.unwrap_or(30);
        point_buy_stats(budget)
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    };

    apply_species_modifiers(&species, &mut core_stats);

    if !core_stats.contains_key("defense") {
        core_stats.insert("defense".to_string(), 10);
    }
    if !core_stats.contains_key("luck") {
        core_stats.insert("luck".to_string(), 10);
    }

    let derived = calculate_derived_attributes(&core_stats, &archetype, level);
    for (key, value) in derived {
        core_stats.insert(key, value);
    }

    let skills = allocate_skills(&archetype, level);
    let equipment = generate_equipment(&archetype);

    let mut attributes = HashMap::new();
    attributes.insert(
        "archetype".to_string(),
        AttributeValue::String(archetype.name().to_string()),
    );
    attributes.insert(
        "species".to_string(),
        AttributeValue::String(species.name().to_string()),
    );
    attributes.insert("level".to_string(), AttributeValue::Integer(level as i32));

    for (attr, value) in &core_stats {
        attributes.insert(attr.clone(), AttributeValue::Integer(*value));
    }

    for (skill, value) in &skills {
        attributes.insert(format!("skill_{}", skill), AttributeValue::Integer(*value));
    }

    attributes.insert(
        "skills".to_string(),
        AttributeValue::Map(
            skills
                .into_iter()
                .map(|(k, v)| (k, AttributeValue::Integer(v)))
                .collect(),
        ),
    );

    attributes.insert(
        "equipment".to_string(),
        AttributeValue::List(
            equipment.into_iter().map(AttributeValue::String).collect(),
        ),
    );

    attributes.insert(
        "special_traits".to_string(),
        AttributeValue::List(
            species
                .special_traits()
                .iter()
                .map(|&s| AttributeValue::String(s.to_string()))
                .collect(),
        ),
    );

    attributes.insert(
        "xp".to_string(),
        AttributeValue::Integer(xp_for_level(level) as i32),
    );
    attributes.insert(
        "xp_next".to_string(),
        AttributeValue::Integer(xp_for_level(level + 1) as i32),
    );
    attributes.insert(
        "available_skill_points".to_string(),
        AttributeValue::Integer(0),
    );
    attributes.insert(
        "starting_gold".to_string(),
        AttributeValue::Integer(archetype.starting_gold() as i32),
    );

    Ok(attributes)
}
