use rand::Rng;
use oa_core::AttributeValue;
use std::collections::HashMap;

use super::data::*;

pub fn roll_3d6() -> i32 {
    let mut rng = rand::thread_rng();
    (0..3).map(|_| rng.gen_range(1..=6)).sum()
}

pub fn roll_4d6_drop_lowest() -> i32 {
    let mut rng = rand::thread_rng();
    let mut rolls: Vec<i32> = (0..4).map(|_| rng.gen_range(1..=6)).collect();
    rolls.sort_unstable();
    rolls[1..].iter().sum()
}

pub fn roll_stats_3d6() -> (i32, i32, i32, i32, i32, i32) {
    (roll_3d6(), roll_3d6(), roll_3d6(), roll_3d6(), roll_3d6(), roll_3d6())
}

pub fn roll_stats_4d6() -> (i32, i32, i32, i32, i32, i32) {
    (roll_4d6_drop_lowest(), roll_4d6_drop_lowest(), roll_4d6_drop_lowest(),
     roll_4d6_drop_lowest(), roll_4d6_drop_lowest(), roll_4d6_drop_lowest())
}

pub fn roll_stats_ph_method() -> (i32, i32, i32, i32, i32, i32) {
    let mut rolls: Vec<i32> = (0..6).map(|_| roll_4d6_drop_lowest()).collect();
    rolls.sort_unstable_by(|a, b| b.cmp(a));
    (rolls[0], rolls[1], rolls[2], rolls[3], rolls[4], rolls[5])
}

pub fn apply_racial_modifiers(
    race: &Race,
    stats: (i32, i32, i32, i32, i32, i32),
) -> (i32, i32, i32, i32, i32, i32) {
    let mods = race.ability_modifiers();
    (
        (stats.0 + mods.0).clamp(1, 25),
        (stats.1 + mods.1).clamp(1, 25),
        (stats.2 + mods.2).clamp(1, 25),
        (stats.3 + mods.3).clamp(1, 25),
        (stats.4 + mods.4).clamp(1, 25),
        (stats.5 + mods.5).clamp(1, 25),
    )
}

pub fn meets_class_requirements(class: &Class, stats: (i32, i32, i32, i32, i32, i32)) -> bool {
    let req = class.ability_requirements();
    stats.0 >= req.0
        && stats.1 >= req.1
        && stats.2 >= req.2
        && stats.3 >= req.3
        && stats.4 >= req.4
        && stats.5 >= req.5
}

pub fn meets_race_requirements(race: &Race, stats: (i32, i32, i32, i32, i32, i32)) -> bool {
    let req = race.minimum_abilities();
    stats.0 >= req.0
        && stats.1 >= req.1
        && stats.2 >= req.2
        && stats.3 >= req.3
        && stats.4 >= req.4
        && stats.5 >= req.5
}

pub fn race_class_compatible(race: &Race, class: &Class) -> bool {
    let allowed = race.allowed_classes();
    allowed.contains_key(class.name())
}

pub fn alignment_class_compatible(alignment: &Alignment, class: &Class) -> bool {
    class.allowed_alignments().contains(alignment)
}

pub fn roll_hp(class: &Class, con: i32, level: u32) -> i32 {
    let mut rng = rand::thread_rng();
    let hit_die = class.hit_die() as i32;
    let con_bonus = constitution_table(con).0;
    let warrior_bonus = if class.group() == ClassGroup::Warrior {
        warrior_hp_bonus(con)
    } else {
        0
    };
    let total_bonus = con_bonus + warrior_bonus;

    let mut hp = 0;
    for _ in 0..level {
        let roll = rng.gen_range(1..=hit_die);
        hp += (roll + total_bonus).max(1);
    }
    hp
}

pub fn calculate_thac0(class: &Class, level: u32) -> i32 {
    thac0(class.group(), level)
}

pub fn calculate_saving_throws(class: &Class, level: u32) -> HashMap<String, i32> {
    let saves = saving_throws(class.group(), level);
    let mut map = HashMap::new();
    map.insert("paralyze".to_string(), saves.0);
    map.insert("poison".to_string(), saves.0);
    map.insert("death".to_string(), saves.0);
    map.insert("rod".to_string(), saves.1);
    map.insert("staff".to_string(), saves.1);
    map.insert("wand".to_string(), saves.1);
    map.insert("petrification".to_string(), saves.2);
    map.insert("polymorph".to_string(), saves.2);
    map.insert("breath_weapon".to_string(), saves.3);
    map.insert("spell".to_string(), saves.4);
    map
}

pub fn calculate_spell_slots(class: &Class, level: u32) -> HashMap<String, Vec<u32>> {
    let mut map = HashMap::new();
    if !class.casts_spells() {
        return map;
    }

    let slots = match class.spell_type() {
        Some(SpellType::Arcane) => arcane_spell_slots(level),
        Some(SpellType::Divine) => divine_spell_slots(level),
        None => vec![],
    };

    map.insert("spell_slots".to_string(), slots);
    map
}

pub fn calculate_ability_derived_stats(
    stats: (i32, i32, i32, i32, i32, i32),
) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let str_data = strength_table(stats.0);
    let dex_data = dexterity_table(stats.1);
    let con_data = constitution_table(stats.2);
    let int_data = intelligence_table(stats.3);
    let wis_data = wisdom_table(stats.4);
    let cha_data = charisma_table(stats.5);

    map.insert("str_hit_prob".to_string(), str_data.0);
    map.insert("str_damage".to_string(), str_data.1);
    map.insert("str_weight_allow".to_string(), str_data.2);
    map.insert("str_max_press".to_string(), str_data.3);
    map.insert("str_open_doors".to_string(), str_data.4);
    map.insert("str_bend_bars".to_string(), str_data.5);

    map.insert("dex_reaction".to_string(), dex_data.0);
    map.insert("dex_missile".to_string(), dex_data.1);
    map.insert("dex_defensive".to_string(), dex_data.2);

    map.insert("con_hp_bonus".to_string(), con_data.0);
    map.insert("con_system_shock".to_string(), con_data.1);
    map.insert("con_resurrection".to_string(), con_data.2);

    map.insert("int_languages".to_string(), int_data.0);
    map.insert("int_spell_level".to_string(), int_data.1);
    map.insert("int_learn_chance".to_string(), int_data.2);
    map.insert("int_max_spells".to_string(), int_data.3);

    map.insert("wis_defense".to_string(), wis_data.0);
    map.insert("wis_spell_failure".to_string(), wis_data.2);

    map.insert("cha_max_henchmen".to_string(), cha_data.0);
    map.insert("cha_loyalty".to_string(), cha_data.1);
    map.insert("cha_reaction".to_string(), cha_data.2);

    map
}

pub fn calculate_rogue_skills(class: &Class, level: u32, dex: i32) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    if class.group() != ClassGroup::Rogue {
        return map;
    }

    let bases = thief_skill_bases();
    let advancement = thief_skill_advancement(level);
    let dex_adj = dexterity_thief_adjustment(dex);

    map.insert("pick_pockets".to_string(), (bases.0 + advancement.0 + dex_adj.0).min(95));
    map.insert("open_locks".to_string(), (bases.1 + advancement.1 + dex_adj.1).min(95));
    map.insert("find_remove_traps".to_string(), (bases.2 + advancement.2 + dex_adj.2).min(95));
    map.insert("move_silently".to_string(), (bases.3 + advancement.3 + dex_adj.3).min(95));
    map.insert("hide_in_shadows".to_string(), (bases.4 + advancement.4 + dex_adj.4).min(95));
    map.insert("detect_noise".to_string(), (bases.5 + advancement.5).min(95));
    map.insert("climb_walls".to_string(), (bases.6 + advancement.6).min(95));
    map.insert("read_languages".to_string(), (bases.7 + advancement.7).min(95));

    map
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
    alignment_name: &str,
    level: u32,
    method: &str,
) -> Result<HashMap<String, AttributeValue>, String> {
    let race = race_name.parse::<Race>()?;
    let class = Class::from_str(class_name).ok_or_else(|| format!("Unknown class: {}", class_name))?;
    let alignment = Alignment::from_str(alignment_name)
        .ok_or_else(|| format!("Unknown alignment: {}", alignment_name))?;

    if !race_class_compatible(&race, &class) {
        return Err(format!("{} cannot be {}", race.name(), class.name()));
    }

    if !alignment_class_compatible(&alignment, &class) {
        return Err(format!("{} cannot be {}", alignment.name(), class.name()));
    }

    let mut stats = (0, 0, 0, 0, 0, 0);
    for _ in 0..100 {
        let raw_stats = match method {
            "4d6" => roll_stats_4d6(),
            "phb" => roll_stats_ph_method(),
            _ => roll_stats_3d6(),
        };
        if !meets_race_requirements(&race, raw_stats) {
            continue;
        }
        let adjusted = apply_racial_modifiers(&race, raw_stats);
        if meets_class_requirements(&class, adjusted) {
            stats = adjusted;
            break;
        }
    }

    if stats == (0, 0, 0, 0, 0, 0) {
        return Err(format!(
            "Could not generate valid stats for {} {} after 100 attempts",
            race.name(),
            class.name()
        ));
    }

    let (str, dex, con, int, wis, cha) = stats;
    let hp = roll_hp(&class, con, level);
    let thac0 = calculate_thac0(&class, level);
    let saves = calculate_saving_throws(&class, level);
    let spells = calculate_spell_slots(&class, level);
    let derived = calculate_ability_derived_stats(stats);
    let rogue_skills = calculate_rogue_skills(&class, level, dex);
    let gold = roll_starting_gold(&class);

    let max_level = race.allowed_classes().get(class.name()).copied().flatten();
    if let Some(max) = max_level {
        if level > max {
            return Err(format!(
                "{} maximum level for {} is {}",
                race.name(),
                class.name(),
                max
            ));
        }
    }

    let mut attributes = HashMap::new();
    attributes.insert("race".to_string(), AttributeValue::String(race.name().to_string()));
    attributes.insert("class".to_string(), AttributeValue::String(class.name().to_string()));
    attributes.insert("alignment".to_string(), AttributeValue::String(alignment.name().to_string()));
    attributes.insert("level".to_string(), AttributeValue::Integer(level as i32));
    attributes.insert("strength".to_string(), AttributeValue::Integer(str));
    attributes.insert("dexterity".to_string(), AttributeValue::Integer(dex));
    attributes.insert("constitution".to_string(), AttributeValue::Integer(con));
    attributes.insert("intelligence".to_string(), AttributeValue::Integer(int));
    attributes.insert("wisdom".to_string(), AttributeValue::Integer(wis));
    attributes.insert("charisma".to_string(), AttributeValue::Integer(cha));
    attributes.insert("hit_points".to_string(), AttributeValue::Integer(hp));
    attributes.insert("thac0".to_string(), AttributeValue::Integer(thac0));
    attributes.insert("armor_class".to_string(), AttributeValue::Integer(BASE_AC));
    attributes.insert("starting_gold".to_string(), AttributeValue::Integer(gold as i32));

    for (key, value) in saves {
        attributes.insert(format!("save_{}", key), AttributeValue::Integer(value));
    }

    for (key, value) in derived {
        attributes.insert(key, AttributeValue::Integer(value));
    }

    for (key, value) in rogue_skills {
        attributes.insert(key, AttributeValue::Integer(value));
    }

    if let Some(slots) = spells.get("spell_slots") {
        attributes.insert(
            "spell_slots".to_string(),
            AttributeValue::List(
                slots.iter().map(|&s| AttributeValue::Integer(s as i32)).collect()
            ),
        );
    }

    let prime_reqs = class.prime_requisites();
    let prime_scores: Vec<i32> = prime_reqs.iter().map(|&attr| match attr {
        "str" => str,
        "dex" => dex,
        "con" => con,
        "int" => int,
        "wis" => wis,
        "cha" => cha,
        _ => 0,
    }).collect();
    let xp_bonus_pct = xp_bonus(&prime_scores);
    attributes.insert("xp_bonus".to_string(), AttributeValue::Integer(xp_bonus_pct));

    let weapon_profs = class.initial_weapon_proficiencies();
    attributes.insert("weapon_proficiency_slots".to_string(), AttributeValue::Integer(weapon_profs as i32));
    attributes.insert("non_proficiency_penalty".to_string(), AttributeValue::Integer(class.non_proficiency_penalty()));

    attributes.insert(
        "special_abilities".to_string(),
        AttributeValue::List(
            race.special_abilities().iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );

    Ok(attributes)
}
