#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    HalfElf,
    HalfOrc,
    Dragonborn,
    Gnome,
    Tiefling,
}

impl Race {
    pub fn name(&self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::Elf => "Elf",
            Race::Dwarf => "Dwarf",
            Race::Halfling => "Halfling",
            Race::HalfElf => "Half-Elf",
            Race::HalfOrc => "Half-Orc",
            Race::Dragonborn => "Dragonborn",
            Race::Gnome => "Gnome",
            Race::Tiefling => "Tiefling",
        }
    }

    pub fn ability_increases(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (1, 1, 1, 1, 1, 1),
            Race::Elf => (0, 2, 0, 0, 0, 0),
            Race::Dwarf => (0, 0, 2, 0, 0, 0),
            Race::Halfling => (0, 2, 0, 0, 0, 0),
            Race::HalfElf => (0, 0, 0, 0, 0, 2),
            Race::HalfOrc => (2, 0, 1, 0, 0, 0),
            Race::Dragonborn => (2, 0, 0, 0, 0, 1),
            Race::Gnome => (0, 0, 0, 2, 0, 0),
            Race::Tiefling => (0, 0, 0, 1, 0, 2),
        }
    }

    pub fn size(&self) -> &'static str {
        match self {
            Race::Halfling | Race::Gnome => "Small",
            _ => "Medium",
        }
    }

    pub fn speed(&self) -> i32 {
        match self {
            Race::Dwarf | Race::Halfling | Race::Gnome => 25,
            _ => 30,
        }
    }

    pub fn darkvision(&self) -> bool {
        !matches!(self, Race::Human)
    }

    pub fn special_traits(&self) -> Vec<&'static str> {
        match self {
            Race::Human => vec!["Extra Language"],
            Race::Elf => vec!["Darkvision 60ft", "Keen Senses", "Fey Ancestry", "Trance"],
            Race::Dwarf => vec!["Darkvision 60ft", "Dwarven Resilience", "Stonecunning"],
            Race::Halfling => vec!["Lucky", "Brave", "Halfling Nimbleness"],
            Race::HalfElf => vec!["Darkvision 60ft", "Fey Ancestry", "Skill Versatility"],
            Race::HalfOrc => vec!["Darkvision 60ft", "Relentless Endurance", "Savage Attacks"],
            Race::Dragonborn => vec!["Draconic Ancestry", "Breath Weapon", "Damage Resistance"],
            Race::Gnome => vec!["Darkvision 60ft", "Gnome Cunning"],
            Race::Tiefling => vec!["Darkvision 60ft", "Hellish Resistance", "Infernal Legacy"],
        }
    }

    pub fn has_flexible_ability_increases(&self) -> bool {
        matches!(self, Race::HalfElf)
    }
}

impl std::str::FromStr for Race {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" => Ok(Race::Human),
            "elf" => Ok(Race::Elf),
            "dwarf" => Ok(Race::Dwarf),
            "halfling" => Ok(Race::Halfling),
            "half-elf" | "halfelf" => Ok(Race::HalfElf),
            "half-orc" | "halforc" => Ok(Race::HalfOrc),
            "dragonborn" => Ok(Race::Dragonborn),
            "gnome" => Ok(Race::Gnome),
            "tiefling" => Ok(Race::Tiefling),
            _ => Err(format!("Unknown race: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Barbarian => "Barbarian",
            Class::Bard => "Bard",
            Class::Cleric => "Cleric",
            Class::Druid => "Druid",
            Class::Fighter => "Fighter",
            Class::Monk => "Monk",
            Class::Paladin => "Paladin",
            Class::Ranger => "Ranger",
            Class::Rogue => "Rogue",
            Class::Sorcerer => "Sorcerer",
            Class::Warlock => "Warlock",
            Class::Wizard => "Wizard",
        }
    }

    pub fn hit_die(&self) -> u32 {
        match self {
            Class::Barbarian => 12,
            Class::Fighter | Class::Paladin | Class::Ranger => 10,
            Class::Bard | Class::Cleric | Class::Druid | Class::Monk | Class::Rogue | Class::Warlock => 8,
            Class::Sorcerer | Class::Wizard => 6,
        }
    }

    pub fn saving_throw_proficiencies(&self) -> (&'static str, &'static str) {
        match self {
            Class::Barbarian => ("strength", "constitution"),
            Class::Bard => ("dexterity", "charisma"),
            Class::Cleric => ("wisdom", "charisma"),
            Class::Druid => ("intelligence", "wisdom"),
            Class::Fighter => ("strength", "constitution"),
            Class::Monk => ("strength", "dexterity"),
            Class::Paladin => ("wisdom", "charisma"),
            Class::Ranger => ("strength", "dexterity"),
            Class::Rogue => ("dexterity", "intelligence"),
            Class::Sorcerer => ("constitution", "charisma"),
            Class::Warlock => ("wisdom", "charisma"),
            Class::Wizard => ("intelligence", "wisdom"),
        }
    }

    pub fn primary_ability(&self) -> &'static str {
        match self {
            Class::Barbarian => "strength",
            Class::Bard => "charisma",
            Class::Cleric => "wisdom",
            Class::Druid => "wisdom",
            Class::Fighter => "strength",
            Class::Monk => "dexterity",
            Class::Paladin => "strength",
            Class::Ranger => "dexterity",
            Class::Rogue => "dexterity",
            Class::Sorcerer => "charisma",
            Class::Warlock => "charisma",
            Class::Wizard => "intelligence",
        }
    }

    pub fn spellcasting_ability(&self) -> Option<&'static str> {
        match self {
            Class::Bard | Class::Sorcerer | Class::Warlock | Class::Paladin => Some("charisma"),
            Class::Cleric | Class::Druid | Class::Ranger => Some("wisdom"),
            Class::Wizard => Some("intelligence"),
            _ => None,
        }
    }

    pub fn casts_spells(&self) -> bool {
        self.spellcasting_ability().is_some()
    }

    pub fn spell_progression(&self) -> SpellProgression {
        match self {
            Class::Bard | Class::Cleric | Class::Druid | Class::Sorcerer | Class::Wizard => SpellProgression::Full,
            Class::Paladin | Class::Ranger => SpellProgression::Half,
            Class::Warlock => SpellProgression::PactMagic,
            _ => SpellProgression::None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "barbarian" => Some(Class::Barbarian),
            "bard" => Some(Class::Bard),
            "cleric" => Some(Class::Cleric),
            "druid" => Some(Class::Druid),
            "fighter" => Some(Class::Fighter),
            "monk" => Some(Class::Monk),
            "paladin" => Some(Class::Paladin),
            "ranger" => Some(Class::Ranger),
            "rogue" => Some(Class::Rogue),
            "sorcerer" => Some(Class::Sorcerer),
            "warlock" => Some(Class::Warlock),
            "wizard" => Some(Class::Wizard),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellProgression {
    None,
    Half,
    Full,
    PactMagic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Background {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

impl Background {
    pub fn name(&self) -> &'static str {
        match self {
            Background::Acolyte => "Acolyte",
            Background::Charlatan => "Charlatan",
            Background::Criminal => "Criminal",
            Background::Entertainer => "Entertainer",
            Background::FolkHero => "Folk Hero",
            Background::GuildArtisan => "Guild Artisan",
            Background::Hermit => "Hermit",
            Background::Noble => "Noble",
            Background::Outlander => "Outlander",
            Background::Sage => "Sage",
            Background::Sailor => "Sailor",
            Background::Soldier => "Soldier",
            Background::Urchin => "Urchin",
        }
    }

    pub fn skill_proficiencies(&self) -> Vec<&'static str> {
        match self {
            Background::Acolyte => vec!["insight", "religion"],
            Background::Charlatan => vec!["deception", "sleight_of_hand"],
            Background::Criminal => vec!["deception", "stealth"],
            Background::Entertainer => vec!["acrobatics", "performance"],
            Background::FolkHero => vec!["animal_handling", "survival"],
            Background::GuildArtisan => vec!["insight", "persuasion"],
            Background::Hermit => vec!["medicine", "religion"],
            Background::Noble => vec!["history", "persuasion"],
            Background::Outlander => vec!["athletics", "survival"],
            Background::Sage => vec!["arcana", "history"],
            Background::Sailor => vec!["athletics", "perception"],
            Background::Soldier => vec!["athletics", "intimidation"],
            Background::Urchin => vec!["sleight_of_hand", "stealth"],
        }
    }

    pub fn starting_equipment(&self) -> Vec<&'static str> {
        match self {
            Background::Acolyte => vec!["Holy symbol", "Prayer book", "5 sticks of incense", "Vestments", "Common clothes", "15 gp"],
            Background::Charlatan => vec!["Fine clothes", "Disguise kit", "Con tools", "15 gp"],
            Background::Criminal => vec!["Crowbar", "Dark common clothes", "15 gp"],
            Background::Entertainer => vec!["Musical instrument", "Favor of admirer", "Costume", "15 gp"],
            Background::FolkHero => vec!["Artisan's tools", "Shovel", "Iron pot", "Common clothes", "10 gp"],
            Background::GuildArtisan => vec!["Artisan's tools", "Letter of introduction", "Traveler's clothes", "15 gp"],
            Background::Hermit => vec!["Scroll case", "Winter blanket", "Common clothes", "Herbalism kit", "5 gp"],
            Background::Noble => vec!["Fine clothes", "Signet ring", "Scroll of pedigree", "25 gp"],
            Background::Outlander => vec!["Staff", "Hunting trap", "Trophy from animal", "Traveler's clothes", "10 gp"],
            Background::Sage => vec!["Ink and quill", "Small knife", "Letter from colleague", "Common clothes", "10 gp"],
            Background::Sailor => vec!["Club", "50 feet of silk rope", "Lucky charm", "Common clothes", "10 gp"],
            Background::Soldier => vec!["Insignia of rank", "Trophy from fallen enemy", "Bone dice or deck of cards", "Common clothes", "10 gp"],
            Background::Urchin => vec!["Small knife", "Map of home city", "Pet mouse", "Token of parents", "Common clothes", "10 gp"],
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "acolyte" => Some(Background::Acolyte),
            "charlatan" => Some(Background::Charlatan),
            "criminal" => Some(Background::Criminal),
            "entertainer" => Some(Background::Entertainer),
            "folk hero" | "folkhero" => Some(Background::FolkHero),
            "guild artisan" | "guildartisan" => Some(Background::GuildArtisan),
            "hermit" => Some(Background::Hermit),
            "noble" => Some(Background::Noble),
            "outlander" => Some(Background::Outlander),
            "sage" => Some(Background::Sage),
            "sailor" => Some(Background::Sailor),
            "soldier" => Some(Background::Soldier),
            "urchin" => Some(Background::Urchin),
            _ => None,
        }
    }
}

pub fn ability_modifier(score: i32) -> i32 {
    (score - 10) / 2
}

pub fn proficiency_bonus(level: u32) -> i32 {
    match level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 2,
    }
}

pub fn full_caster_spell_slots(caster_level: u32) -> Vec<u32> {
    let table: [Vec<u32>; 20] = [
        vec![2],
        vec![3],
        vec![4, 2],
        vec![4, 3],
        vec![4, 3, 2],
        vec![4, 3, 3],
        vec![4, 3, 3, 1],
        vec![4, 3, 3, 2],
        vec![4, 3, 3, 3, 1],
        vec![4, 3, 3, 3, 2],
        vec![4, 3, 3, 3, 2, 1],
        vec![4, 3, 3, 3, 2, 1],
        vec![4, 3, 3, 3, 2, 1, 1],
        vec![4, 3, 3, 3, 2, 1, 1],
        vec![4, 3, 3, 3, 2, 1, 1, 1],
        vec![4, 3, 3, 3, 2, 1, 1, 1],
        vec![4, 3, 3, 3, 2, 1, 1, 1, 1],
        vec![4, 3, 3, 3, 3, 1, 1, 1, 1],
        vec![4, 3, 3, 3, 3, 2, 1, 1, 1],
        vec![4, 3, 3, 3, 3, 2, 2, 1, 1],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn half_caster_spell_slots(caster_level: u32) -> Vec<u32> {
    let table: [Vec<u32>; 20] = [
        vec![],
        vec![2],
        vec![3],
        vec![3],
        vec![4, 2],
        vec![4, 2],
        vec![4, 3],
        vec![4, 3],
        vec![4, 3, 2],
        vec![4, 3, 2],
        vec![4, 3, 3],
        vec![4, 3, 3],
        vec![4, 3, 3, 1],
        vec![4, 3, 3, 1],
        vec![4, 3, 3, 2],
        vec![4, 3, 3, 2],
        vec![4, 3, 3, 3, 1],
        vec![4, 3, 3, 3, 1],
        vec![4, 3, 3, 3, 2],
        vec![4, 3, 3, 3, 2],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn warlock_spell_slots(level: u32) -> Vec<u32> {
    match level {
        1..=2 => vec![1],
        3..=4 => vec![2],
        5..=10 => vec![2],
        11..=16 => vec![3],
        17..=20 => vec![4],
        _ => vec![1],
    }
}

pub fn warlock_slot_level(level: u32) -> u32 {
    match level {
        1..=2 => 1,
        3..=4 => 2,
        5..=6 => 3,
        7..=8 => 4,
        9..=20 => 5,
        _ => 1,
    }
}

pub fn starting_gold(class: &Class) -> (u32, u32, u32) {
    match class {
        Class::Barbarian => (2, 4, 10),
        Class::Bard | Class::Rogue | Class::Sorcerer | Class::Warlock | Class::Wizard => (3, 4, 10),
        Class::Cleric | Class::Druid | Class::Monk | Class::Ranger => (4, 4, 10),
        Class::Fighter | Class::Paladin => (5, 4, 10),
    }
}

pub fn class_skills(class: &Class) -> Vec<&'static str> {
    match class {
        Class::Barbarian => vec!["animal_handling", "athletics", "intimidation", "nature", "perception", "survival"],
        Class::Bard => vec!["acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "nature", "perception", "performance", "persuasion", "religion", "sleight_of_hand", "stealth"],
        Class::Cleric => vec!["history", "insight", "medicine", "persuasion", "religion"],
        Class::Druid => vec!["arcana", "animal_handling", "insight", "medicine", "nature", "perception", "religion", "survival"],
        Class::Fighter => vec!["acrobatics", "animal_handling", "athletics", "history", "insight", "intimidation", "perception", "survival"],
        Class::Monk => vec!["acrobatics", "athletics", "history", "insight", "religion", "stealth"],
        Class::Paladin => vec!["athletics", "insight", "intimidation", "medicine", "persuasion", "religion"],
        Class::Ranger => vec!["animal_handling", "athletics", "insight", "investigation", "nature", "perception", "stealth", "survival"],
        Class::Rogue => vec!["acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "perception", "performance", "persuasion", "sleight_of_hand", "stealth"],
        Class::Sorcerer => vec!["arcana", "deception", "insight", "intimidation", "persuasion", "religion"],
        Class::Warlock => vec!["arcana", "deception", "history", "intimidation", "investigation", "nature", "religion"],
        Class::Wizard => vec!["arcana", "history", "insight", "investigation", "medicine", "religion"],
    }
}

pub fn class_skill_choices(class: &Class) -> u32 {
    match class {
        Class::Barbarian | Class::Fighter | Class::Sorcerer | Class::Wizard | Class::Monk => 2,
        Class::Bard | Class::Ranger | Class::Rogue => 3,
        Class::Cleric | Class::Druid | Class::Paladin | Class::Warlock => 2,
    }
}

pub fn armor_proficiencies(class: &Class) -> Vec<&'static str> {
    match class {
        Class::Barbarian | Class::Fighter | Class::Paladin => vec!["light", "medium", "heavy", "shields"],
        Class::Cleric | Class::Ranger => vec!["light", "medium", "shields"],
        Class::Druid => vec!["light", "medium", "shields"],
        Class::Warlock => vec!["light"],
        _ => vec![],
    }
}

pub fn weapon_proficiencies(class: &Class) -> Vec<&'static str> {
    match class {
        Class::Barbarian | Class::Fighter => vec!["simple", "martial"],
        Class::Bard | Class::Rogue => vec!["simple", "hand_crossbow", "longsword", "rapier", "shortsword"],
        Class::Cleric => vec!["simple"],
        Class::Druid => vec!["clubs", "daggers", "darts", "javelins", "maces", "quarterstaffs", "scimitars", "sickles", "slings"],
        Class::Monk => vec!["simple", "shortswords"],
        Class::Paladin | Class::Ranger => vec!["simple", "martial"],
        Class::Sorcerer | Class::Warlock | Class::Wizard => vec!["daggers", "darts", "slings", "quarterstaffs", "light_crossbows"],
    }
}

pub const BASE_AC: i32 = 10;

pub fn all_skills() -> Vec<&'static str> {
    vec![
        "acrobatics", "animal_handling", "arcana", "athletics", "deception",
        "history", "insight", "intimidation", "investigation", "medicine",
        "nature", "perception", "performance", "persuasion", "religion",
        "sleight_of_hand", "stealth", "survival",
    ]
}

pub fn all_abilities() -> Vec<&'static str> {
    vec!["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
}
