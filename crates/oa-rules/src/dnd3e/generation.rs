use rand::Rng;
use oa_core::AttributeValue;
use std::collections::HashMap;

use super::data::*;

pub fn roll_4d6_drop_lowest() -> i32 {
    let mut rng = rand::thread_rng();
    let mut rolls: Vec<i32> = (0..4).map(|_| rng.gen_range(1..=6)).collect();
    rolls.sort_unstable();
    rolls[1..].iter().sum()
}

pub fn roll_stats_4d6() -> (i32, i32, i32, i32, i32, i32) {
    let roll = roll_4d6_drop_lowest;
    (roll(), roll(), roll(), roll(), roll(), roll())
}

pub fn standard_array() -> (i32, i32, i32, i32, i32, i32) {
    (15, 14, 13, 12, 10, 8)
}

pub fn point_buy_array(budget: u32) -> Result<(i32, i32, i32, i32, i32, i32), String> {
    let standard = standard_array();
    let cost = point_buy_cost(standard);
    if cost == budget {
        Ok(standard)
    } else {
        Err(format!("Point buy budget {} does not match standard array cost {}", budget, cost))
    }
}

pub fn point_buy_cost(stats: (i32, i32, i32, i32, i32, i32)) -> u32 {
    let scores = [stats.0, stats.1, stats.2, stats.3, stats.4, stats.5];
    scores.iter().map(|&s| point_buy_score_cost(s)).sum()
}

fn point_buy_score_cost(score: i32) -> u32 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 6,
        15 => 8,
        16 => 10,
        17 => 13,
        18 => 16,
        _ => 0,
    }
}

pub fn apply_racial_modifiers(
    race: &Race,
    stats: (i32, i32, i32, i32, i32, i32),
) -> (i32, i32, i32, i32, i32, i32) {
    let mods = race.ability_modifiers();
    (
        (stats.0 + mods.0).clamp(3, 25),
        (stats.1 + mods.1).clamp(3, 25),
        (stats.2 + mods.2).clamp(3, 25),
        (stats.3 + mods.3).clamp(3, 25),
        (stats.4 + mods.4).clamp(3, 25),
        (stats.5 + mods.5).clamp(3, 25),
    )
}

pub fn meets_class_alignment_requirements(class: &Class, alignment: &Alignment) -> bool {
    match class.required_alignment() {
        Some(req) => alignment.satisfies(req),
        None => true,
    }
}

pub fn roll_hp(class: &Class, con_mod: i32, level: u32) -> i32 {
    let mut rng = rand::thread_rng();
    let hit_die = class.hit_die();

    let mut hp = first_level_hp(hit_die, con_mod);
    for _ in 1..level {
        let roll = rng.gen_range(1..=hit_die as i32);
        hp += (roll + con_mod).max(1);
    }
    hp
}

pub fn calculate_bab(class: &Class, level: u32) -> i32 {
    base_attack_bonus(class.bab_progression(), level)
}

pub fn calculate_saves(class: &Class, level: u32) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("fortitude".to_string(), save_bonus(class.fortitude_progression(), level));
    map.insert("reflex".to_string(), save_bonus(class.reflex_progression(), level));
    map.insert("will".to_string(), save_bonus(class.will_progression(), level));
    map
}

pub fn calculate_spell_slots(class: &Class, level: u32) -> HashMap<String, Vec<u32>> {
    let mut map = HashMap::new();
    if !class.casts_spells() {
        return map;
    }

    let slots = match class {
        Class::Wizard | Class::Sorcerer | Class::Cleric | Class::Druid => {
            full_caster_spell_slots(level)
        }
        Class::Bard => bard_spell_slots(level),
        Class::Paladin | Class::Ranger => half_caster_spell_slots(level),
        _ => vec![],
    };

    if !slots.is_empty() {
        map.insert("spell_slots".to_string(), slots);
    }
    map
}

pub fn calculate_skills(
    class: &Class,
    level: u32,
    int_mod: i32,
    is_human: bool,
) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let class_skills = class.class_skills();
    let skill_points = (class.skill_points_per_level() + int_mod).max(1);
    let total_points = if level == 1 {
        (skill_points + if is_human { 1 } else { 0 }) * 4
    } else {
        (skill_points + if is_human { 1 } else { 0 }) * (level as i32)
    };

    let max_rank = max_class_skill_rank(level);
    let cross_max = max_cross_class_skill_rank(level);

    let all_skills_list = all_skills();
    let points_per_skill = total_points / (all_skills_list.len() as i32).max(1);

    for skill in all_skills_list.iter() {
        let is_class = class_skills.contains(&skill.name());
        let rank = if is_class {
            (points_per_skill).min(max_rank)
        } else {
            (points_per_skill / 2).min(cross_max)
        };
        map.insert(skill.name().to_string(), rank);
    }

    map.insert("skill_points_total".to_string(), total_points);
    map.insert("skill_points_remaining".to_string(), 0);
    map
}

pub fn calculate_feats(class: &Class, level: u32) -> Vec<String> {
    let mut feats = Vec::new();

    let normal = normal_feat_count(level);
    for _ in 0..normal {
        feats.push("General Feat".to_string());
    }

    if class.bonus_feats() {
        let bonus = fighter_bonus_feat_count(level);
        for _ in 0..bonus {
            feats.push("Fighter Bonus Feat".to_string());
        }
    }

    feats
}

pub fn calculate_base_ac(
    dex_mod: i32,
    size_mod: i32,
) -> i32 {
    super::data::calculate_ac(dex_mod, 0, 0, size_mod, 0, 0, 0)
}

pub fn roll_starting_gold(class: &Class) -> u32 {
    let mut rng = rand::thread_rng();
    let (dice_count, die_size, multiplier) = starting_gold(class);
    let roll: u32 = (0..dice_count).map(|_| rng.gen_range(1..=die_size)).sum();
    roll * multiplier
}

pub fn generate_character_attributes(
    race_name: &str,
    class_name: &str,
    level: u32,
    use_rolled: bool,
    point_buy: Option<u32>,
) -> Result<HashMap<String, AttributeValue>, String> {
    let race = race_name.parse::<Race>()?;
    let class = Class::from_str(class_name).ok_or_else(|| format!("Unknown class: {}", class_name))?;

    let raw_stats = if let Some(budget) = point_buy {
        point_buy_array(budget).unwrap_or_else(|_| {
            if use_rolled {
                roll_stats_4d6()
            } else {
                standard_array()
            }
        })
    } else if use_rolled {
        roll_stats_4d6()
    } else {
        standard_array()
    };

    let stats = apply_racial_modifiers(&race, raw_stats);
    let (str, dex, con, int, wis, cha) = stats;

    let str_mod = ability_modifier(str);
    let dex_mod = ability_modifier(dex);
    let con_mod = ability_modifier(con);
    let int_mod = ability_modifier(int);
    let wis_mod = ability_modifier(wis);
    let cha_mod = ability_modifier(cha);

    let hp = roll_hp(&class, con_mod, level);
    let bab = calculate_bab(&class, level);
    let saves = calculate_saves(&class, level);
    let spells = calculate_spell_slots(&class, level);
    let skills = calculate_skills(&class, level, int_mod, matches!(race, Race::Human));
    let feats = calculate_feats(&class, level);
    let gold = roll_starting_gold(&class);
    let size_mod = race.size().modifier();
    let ac = calculate_base_ac(dex_mod, size_mod);

    let mut attributes = HashMap::new();
    attributes.insert("race".to_string(), AttributeValue::String(race.name()));
    attributes.insert("class".to_string(), AttributeValue::String(class.name().to_string()));
    attributes.insert("level".to_string(), AttributeValue::Integer(level as i32));
    attributes.insert("strength".to_string(), AttributeValue::Integer(str));
    attributes.insert("dexterity".to_string(), AttributeValue::Integer(dex));
    attributes.insert("constitution".to_string(), AttributeValue::Integer(con));
    attributes.insert("intelligence".to_string(), AttributeValue::Integer(int));
    attributes.insert("wisdom".to_string(), AttributeValue::Integer(wis));
    attributes.insert("charisma".to_string(), AttributeValue::Integer(cha));
    attributes.insert("str_mod".to_string(), AttributeValue::Integer(str_mod));
    attributes.insert("dex_mod".to_string(), AttributeValue::Integer(dex_mod));
    attributes.insert("con_mod".to_string(), AttributeValue::Integer(con_mod));
    attributes.insert("int_mod".to_string(), AttributeValue::Integer(int_mod));
    attributes.insert("wis_mod".to_string(), AttributeValue::Integer(wis_mod));
    attributes.insert("cha_mod".to_string(), AttributeValue::Integer(cha_mod));
    attributes.insert("hit_points".to_string(), AttributeValue::Integer(hp));
    attributes.insert("base_attack_bonus".to_string(), AttributeValue::Integer(bab));
    attributes.insert("armor_class".to_string(), AttributeValue::Integer(ac));
    attributes.insert("starting_gold".to_string(), AttributeValue::Integer(gold as i32));
    attributes.insert("size".to_string(), AttributeValue::String(race.size().name().to_string()));
    attributes.insert("size_modifier".to_string(), AttributeValue::Integer(size_mod));
    attributes.insert("speed".to_string(), AttributeValue::Integer(race.base_speed() as i32));
    attributes.insert("favored_class".to_string(), AttributeValue::String(race.favored_class().to_string()));

    for (key, value) in saves {
        attributes.insert(format!("save_{}", key), AttributeValue::Integer(value));
    }

    if let Some(spell_ability) = class.spellcasting_ability() {
        let spell_mod = ability_modifier(match spell_ability {
            Ability::Str => str,
            Ability::Dex => dex,
            Ability::Con => con,
            Ability::Int => int,
            Ability::Wis => wis,
            Ability::Cha => cha,
        });
        let spell_dc = 10 + spell_mod;
        attributes.insert("spell_save_dc".to_string(), AttributeValue::Integer(spell_dc));
        attributes.insert("spellcasting_ability".to_string(), AttributeValue::String(spell_ability.abbreviation().to_string()));
    }

    if let Some(slots) = spells.get("spell_slots") {
        attributes.insert(
            "spell_slots".to_string(),
            AttributeValue::List(slots.iter().map(|&v| AttributeValue::Integer(v as i32)).collect()),
        );
    }

    let mut skills_map = HashMap::new();
    for (key, value) in skills {
        skills_map.insert(key, AttributeValue::Integer(value));
    }
    attributes.insert("skills".to_string(), AttributeValue::Map(skills_map));

    attributes.insert(
        "feats".to_string(),
        AttributeValue::List(feats.iter().map(|f| AttributeValue::String(f.clone())).collect()),
    );

    let num_attacks_val = num_attacks(bab);
    attributes.insert("num_attacks".to_string(), AttributeValue::Integer(num_attacks_val as i32));

    let attack_bonuses_val = attack_bonuses(bab);
    attributes.insert(
        "attack_bonuses".to_string(),
        AttributeValue::List(attack_bonuses_val.iter().map(|&v| AttributeValue::Integer(v)).collect()),
    );

    attributes.insert(
        "special_traits".to_string(),
        AttributeValue::List(
            race.special_traits().iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );

    attributes.insert("xp_for_next_level".to_string(), AttributeValue::Integer(xp_for_level(level + 1) as i32));

    Ok(attributes)
}
