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
    (
        roll_4d6_drop_lowest(),
        roll_4d6_drop_lowest(),
        roll_4d6_drop_lowest(),
        roll_4d6_drop_lowest(),
        roll_4d6_drop_lowest(),
        roll_4d6_drop_lowest(),
    )
}

pub fn standard_array() -> (i32, i32, i32, i32, i32, i32) {
    (15, 14, 13, 12, 10, 8)
}

pub fn point_buy_cost(score: i32) -> i32 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        _ => 99,
    }
}

pub fn point_buy_total(scores: (i32, i32, i32, i32, i32, i32)) -> i32 {
    point_buy_cost(scores.0)
        + point_buy_cost(scores.1)
        + point_buy_cost(scores.2)
        + point_buy_cost(scores.3)
        + point_buy_cost(scores.4)
        + point_buy_cost(scores.5)
}

pub fn generate_point_buy_stats(budget: u32) -> (i32, i32, i32, i32, i32, i32) {
    let mut rng = rand::thread_rng();
    let mut scores = vec![8, 8, 8, 8, 8, 8];
    let mut remaining = budget as i32;

    for _ in 0..1000 {
        if remaining <= 0 {
            break;
        }
        let idx = rng.gen_range(0..6);
        let current = scores[idx];
        if current >= 15 {
            continue;
        }
        let cost = point_buy_cost(current + 1) - point_buy_cost(current);
        if cost <= remaining {
            scores[idx] = current + 1;
            remaining -= cost;
        }
    }

    scores.sort_unstable_by(|a, b| b.cmp(a));
    (
        scores[0], scores[1], scores[2],
        scores[3], scores[4], scores[5],
    )
}

pub fn apply_racial_increases(
    race: &Race,
    stats: (i32, i32, i32, i32, i32, i32),
) -> (i32, i32, i32, i32, i32, i32) {
    let inc = race.ability_increases();
    (
        (stats.0 + inc.0).clamp(1, 20),
        (stats.1 + inc.1).clamp(1, 20),
        (stats.2 + inc.2).clamp(1, 20),
        (stats.3 + inc.3).clamp(1, 20),
        (stats.4 + inc.4).clamp(1, 20),
        (stats.5 + inc.5).clamp(1, 20),
    )
}

pub fn roll_hp(class: &Class, con: i32, level: u32) -> i32 {
    let mut rng = rand::thread_rng();
    let hit_die = class.hit_die() as i32;
    let con_mod = ability_modifier(con);
    let mut hp = hit_die + con_mod;
    hp = hp.max(1);

    for _ in 1..level {
        let roll = rng.gen_range(1..=hit_die);
        hp += (roll + con_mod).max(1);
    }
    hp
}

pub fn calculate_spell_slots(class: &Class, level: u32) -> HashMap<String, Vec<u32>> {
    let mut map = HashMap::new();
    if !class.casts_spells() {
        return map;
    }

    let slots = match class.spell_progression() {
        SpellProgression::Full => full_caster_spell_slots(level),
        SpellProgression::Half => half_caster_spell_slots(level),
        SpellProgression::PactMagic => warlock_spell_slots(level),
        SpellProgression::None => vec![],
    };

    map.insert("spell_slots".to_string(), slots);
    map
}

pub fn calculate_saving_throws(
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

pub fn calculate_attack_bonus(
    class: &Class,
    stats: (i32, i32, i32, i32, i32, i32),
    level: u32,
) -> i32 {
    let prof = proficiency_bonus(level);
    let primary = match class.primary_ability() {
        "strength" => ability_modifier(stats.0),
        "dexterity" => ability_modifier(stats.1),
        "constitution" => ability_modifier(stats.2),
        "intelligence" => ability_modifier(stats.3),
        "wisdom" => ability_modifier(stats.4),
        "charisma" => ability_modifier(stats.5),
        _ => 0,
    };
    prof + primary
}

pub fn calculate_passive_perception(wis: i32, has_proficiency: bool, level: u32) -> i32 {
    let base = 10 + ability_modifier(wis);
    if has_proficiency {
        base + proficiency_bonus(level)
    } else {
        base
    }
}

pub fn calculate_skill_modifiers(
    class: &Class,
    background: &Background,
    stats: (i32, i32, i32, i32, i32, i32),
    level: u32,
) -> HashMap<String, i32> {
    let mut profs: HashMap<&str, bool> = HashMap::new();

    for skill in class_skills(class) {
        profs.entry(skill).or_insert(false);
    }

    for skill in background.skill_proficiencies() {
        profs.insert(skill, true);
    }

    let mut skills = HashMap::new();
    let skill_ability_map: HashMap<&str, &str> = [
        ("acrobatics", "dexterity"),
        ("animal_handling", "wisdom"),
        ("arcana", "intelligence"),
        ("athletics", "strength"),
        ("deception", "charisma"),
        ("history", "intelligence"),
        ("insight", "wisdom"),
        ("intimidation", "charisma"),
        ("investigation", "intelligence"),
        ("medicine", "wisdom"),
        ("nature", "intelligence"),
        ("perception", "wisdom"),
        ("performance", "charisma"),
        ("persuasion", "charisma"),
        ("religion", "intelligence"),
        ("sleight_of_hand", "dexterity"),
        ("stealth", "dexterity"),
        ("survival", "wisdom"),
    ]
    .iter()
    .cloned()
    .collect();

    let ability_mods = [
        ("strength", ability_modifier(stats.0)),
        ("dexterity", ability_modifier(stats.1)),
        ("constitution", ability_modifier(stats.2)),
        ("intelligence", ability_modifier(stats.3)),
        ("wisdom", ability_modifier(stats.4)),
        ("charisma", ability_modifier(stats.5)),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, i32>>();

    let prof = proficiency_bonus(level);

    for skill in all_skills() {
        let ability = skill_ability_map.get(skill).copied().unwrap_or("strength");
        let base = ability_mods.get(ability).copied().unwrap_or(0);
        let bonus = if profs.get(skill).copied().unwrap_or(false) {
            prof
        } else {
            0
        };
        skills.insert(skill.to_string(), base + bonus);
    }

    skills
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
    background_name: &str,
    level: u32,
    use_rolled_stats: bool,
    point_buy_budget: Option<u32>,
) -> Result<HashMap<String, AttributeValue>, String> {
    let race = race_name.parse::<Race>().map_err(|e| e)?;
    let class = Class::from_str(class_name).ok_or_else(|| format!("Unknown class: {}", class_name))?;
    let background = Background::from_str(background_name).unwrap_or(Background::Soldier);

    let raw_stats = if let Some(budget) = point_buy_budget {
        generate_point_buy_stats(budget)
    } else if use_rolled_stats {
        roll_stats_4d6()
    } else {
        standard_array()
    };

    let stats = apply_racial_increases(&race, raw_stats);
    let (str, dex, con, int, wis, cha) = stats;

    let hp = roll_hp(&class, con, level);
    let saves = calculate_saving_throws(&class, stats, level);
    let spells = calculate_spell_slots(&class, level);
    let skills = calculate_skill_modifiers(&class, &background, stats, level);
    let attack_bonus = calculate_attack_bonus(&class, stats, level);
    let gold = roll_starting_gold(&class);

    let mut attributes = HashMap::new();
    attributes.insert("race".to_string(), AttributeValue::String(race.name().to_string()));
    attributes.insert("class".to_string(), AttributeValue::String(class.name().to_string()));
    attributes.insert("background".to_string(), AttributeValue::String(background.name().to_string()));
    attributes.insert("level".to_string(), AttributeValue::Integer(level as i32));
    attributes.insert("strength".to_string(), AttributeValue::Integer(str));
    attributes.insert("dexterity".to_string(), AttributeValue::Integer(dex));
    attributes.insert("constitution".to_string(), AttributeValue::Integer(con));
    attributes.insert("intelligence".to_string(), AttributeValue::Integer(int));
    attributes.insert("wisdom".to_string(), AttributeValue::Integer(wis));
    attributes.insert("charisma".to_string(), AttributeValue::Integer(cha));
    attributes.insert("hit_points".to_string(), AttributeValue::Integer(hp));
    attributes.insert("armor_class".to_string(), AttributeValue::Integer(BASE_AC));
    attributes.insert("starting_gold".to_string(), AttributeValue::Integer(gold as i32));
    attributes.insert("proficiency_bonus".to_string(), AttributeValue::Integer(proficiency_bonus(level)));
    attributes.insert("attack_bonus".to_string(), AttributeValue::Integer(attack_bonus));
    attributes.insert("speed".to_string(), AttributeValue::Integer(race.speed()));
    attributes.insert("size".to_string(), AttributeValue::String(race.size().to_string()));
    attributes.insert("darkvision".to_string(), AttributeValue::Boolean(race.darkvision()));

    attributes.insert(
        "str_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(str)),
    );
    attributes.insert(
        "dex_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(dex)),
    );
    attributes.insert(
        "con_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(con)),
    );
    attributes.insert(
        "int_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(int)),
    );
    attributes.insert(
        "wis_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(wis)),
    );
    attributes.insert(
        "cha_modifier".to_string(),
        AttributeValue::Integer(ability_modifier(cha)),
    );

    for (key, value) in saves {
        attributes.insert(key, AttributeValue::Integer(value));
    }

    for (key, value) in skills {
        attributes.insert(format!("skill_{}", key), AttributeValue::Integer(value));
    }

    if let Some(slots) = spells.get("spell_slots") {
        attributes.insert(
            "spell_slots".to_string(),
            AttributeValue::List(
                slots.iter().map(|&s| AttributeValue::Integer(s as i32)).collect()
            ),
        );
    }

    if class.spell_progression() == SpellProgression::PactMagic {
        attributes.insert(
            "pact_magic_slot_level".to_string(),
            AttributeValue::Integer(warlock_slot_level(level) as i32),
        );
    }

    if let Some(spell_ability) = class.spellcasting_ability() {
        attributes.insert(
            "spellcasting_ability".to_string(),
            AttributeValue::String(spell_ability.to_string()),
        );
        let spell_save_dc = 8 + proficiency_bonus(level) + ability_modifier(match spell_ability {
            "strength" => str,
            "dexterity" => dex,
            "constitution" => con,
            "intelligence" => int,
            "wisdom" => wis,
            "charisma" => cha,
            _ => 10,
        });
        attributes.insert("spell_save_dc".to_string(), AttributeValue::Integer(spell_save_dc));
    }

    attributes.insert(
        "armor_proficiencies".to_string(),
        AttributeValue::List(
            armor_proficiencies(&class).iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );
    attributes.insert(
        "weapon_proficiencies".to_string(),
        AttributeValue::List(
            weapon_proficiencies(&class).iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );

    attributes.insert(
        "special_traits".to_string(),
        AttributeValue::List(
            race.special_traits().iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );

    attributes.insert(
        "background_equipment".to_string(),
        AttributeValue::List(
            background.starting_equipment().iter().map(|&s| AttributeValue::String(s.to_string())).collect()
        ),
    );

    attributes.insert(
        "passive_perception".to_string(),
        AttributeValue::Integer(calculate_passive_perception(wis, true, level)),
    );

    Ok(attributes)
}
