use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Archetype {
    Warrior,
    Mage,
    Rogue,
    Healer,
    Ranger,
    Bard,
    Paladin,
    Necromancer,
}

impl Archetype {
    pub fn name(&self) -> &'static str {
        match self {
            Archetype::Warrior => "Warrior",
            Archetype::Mage => "Mage",
            Archetype::Rogue => "Rogue",
            Archetype::Healer => "Healer",
            Archetype::Ranger => "Ranger",
            Archetype::Bard => "Bard",
            Archetype::Paladin => "Paladin",
            Archetype::Necromancer => "Necromancer",
        }
    }

    pub fn primary_attribute(&self) -> &'static str {
        match self {
            Archetype::Warrior | Archetype::Paladin | Archetype::Ranger => "body",
            Archetype::Mage | Archetype::Necromancer => "mind",
            Archetype::Rogue | Archetype::Bard => "speed",
            Archetype::Healer => "spirit",
        }
    }

    pub fn secondary_attribute(&self) -> &'static str {
        match self {
            Archetype::Warrior => "defense",
            Archetype::Mage | Archetype::Necromancer | Archetype::Healer => "spirit",
            Archetype::Rogue | Archetype::Ranger | Archetype::Bard => "mind",
            Archetype::Paladin => "luck",
        }
    }

    pub fn hit_die(&self) -> u32 {
        match self {
            Archetype::Warrior | Archetype::Paladin => 10,
            Archetype::Ranger => 8,
            Archetype::Mage | Archetype::Necromancer | Archetype::Healer => 4,
            Archetype::Rogue | Archetype::Bard => 6,
        }
    }

    pub fn starting_skill_points(&self) -> u32 {
        match self {
            Archetype::Rogue | Archetype::Bard => 12,
            Archetype::Mage | Archetype::Ranger => 10,
            Archetype::Warrior | Archetype::Paladin | Archetype::Healer | Archetype::Necromancer => 8,
        }
    }

    pub fn starting_gold(&self) -> u32 {
        match self {
            Archetype::Warrior | Archetype::Paladin => 100,
            Archetype::Ranger | Archetype::Rogue => 75,
            Archetype::Mage | Archetype::Necromancer | Archetype::Healer => 50,
            Archetype::Bard => 60,
        }
    }

    pub fn mana_per_level(&self) -> i32 {
        match self {
            Archetype::Mage | Archetype::Necromancer => 8,
            Archetype::Healer | Archetype::Bard | Archetype::Paladin => 5,
            Archetype::Ranger => 3,
            _ => 2,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "warrior" => Some(Archetype::Warrior),
            "mage" => Some(Archetype::Mage),
            "rogue" => Some(Archetype::Rogue),
            "healer" => Some(Archetype::Healer),
            "ranger" => Some(Archetype::Ranger),
            "bard" => Some(Archetype::Bard),
            "paladin" => Some(Archetype::Paladin),
            "necromancer" => Some(Archetype::Necromancer),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Species {
    Human,
    Elf,
    Dwarf,
    Orc,
    Halfling,
    Celestial,
    Fiend,
    Construct,
}

impl Species {
    pub fn name(&self) -> &'static str {
        match self {
            Species::Human => "Human",
            Species::Elf => "Elf",
            Species::Dwarf => "Dwarf",
            Species::Orc => "Orc",
            Species::Halfling => "Halfling",
            Species::Celestial => "Celestial",
            Species::Fiend => "Fiend",
            Species::Construct => "Construct",
        }
    }

    pub fn attribute_modifiers(&self) -> HashMap<&'static str, i32> {
        let mut map = HashMap::new();
        match self {
            Species::Human => {
                map.insert("body", 1);
            }
            Species::Elf => {
                map.insert("mind", 1);
                map.insert("speed", 1);
                map.insert("body", -1);
            }
            Species::Dwarf => {
                map.insert("body", 1);
                map.insert("spirit", 1);
                map.insert("speed", -1);
            }
            Species::Orc => {
                map.insert("body", 2);
                map.insert("mind", -1);
            }
            Species::Halfling => {
                map.insert("speed", 1);
                map.insert("luck", 1);
                map.insert("body", -1);
            }
            Species::Celestial => {
                map.insert("spirit", 1);
                map.insert("luck", 1);
            }
            Species::Fiend => {
                map.insert("mind", 1);
                map.insert("body", 1);
                map.insert("spirit", -1);
            }
            Species::Construct => {
                map.insert("body", 2);
                map.insert("defense", 1);
                map.insert("luck", -2);
                map.insert("spirit", -1);
            }
        }
        map
    }

    pub fn special_traits(&self) -> Vec<&'static str> {
        match self {
            Species::Human => vec!["Adaptable: +1 to any attribute"],
            Species::Elf => vec!["Keen senses", "Low-light vision"],
            Species::Dwarf => vec!["Darkvision", "Stone resilience"],
            Species::Orc => vec!["Ferocious strength", "Intimidating presence"],
            Species::Halfling => vec!["Lucky", "Small size"],
            Species::Celestial => vec!["Holy resistance", "Blessed touch"],
            Species::Fiend => vec!["Fire resistance", "Darkvision"],
            Species::Construct => vec!["No natural healing", "Poison immunity", "Construct body"],
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "human" => Some(Species::Human),
            "elf" => Some(Species::Elf),
            "dwarf" => Some(Species::Dwarf),
            "orc" => Some(Species::Orc),
            "halfling" => Some(Species::Halfling),
            "celestial" => Some(Species::Celestial),
            "fiend" => Some(Species::Fiend),
            "construct" => Some(Species::Construct),
            _ => None,
        }
    }
}

pub fn xp_for_level(level: u32) -> u32 {
    if level <= 1 {
        0
    } else {
        (level - 1) * 1000
    }
}

pub fn max_level() -> u32 {
    20
}

pub fn base_attributes() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("body", 10);
    map.insert("mind", 10);
    map.insert("spirit", 10);
    map.insert("speed", 10);
    map.insert("defense", 10);
    map.insert("health", 10);
    map.insert("mana", 10);
    map.insert("luck", 10);
    map
}

pub fn attribute_names() -> Vec<&'static str> {
    vec!["body", "mind", "spirit", "speed", "defense", "health", "mana", "luck"]
}

pub fn core_attribute_names() -> Vec<&'static str> {
    vec!["body", "mind", "spirit", "speed", "luck"]
}

pub fn attribute_min_max(attr: &str) -> Option<(i32, i32)> {
    match attr {
        "body" | "mind" | "spirit" | "speed" | "defense" | "luck" => Some((1, 20)),
        "health" | "mana" => Some((1, 200)),
        _ => None,
    }
}

pub fn all_skills() -> Vec<&'static str> {
    vec![
        "melee",
        "ranged",
        "magic",
        "stealth",
        "persuasion",
        "lore",
        "craft",
        "athletics",
        "perception",
        "medicine",
    ]
}

pub fn skill_point_budget(level: u32) -> u32 {
    10 + level.saturating_sub(1) * 2
}

pub fn equipment_for_archetype(archetype: &Archetype) -> Vec<&'static str> {
    match archetype {
        Archetype::Warrior => vec!["Longsword", "Shield", "Chain armor", "Healing potion"],
        Archetype::Mage => vec!["Staff", "Spellbook", "Robes", "Mana potion"],
        Archetype::Rogue => vec!["Dagger", "Lockpicks", "Leather armor", "Smoke bomb"],
        Archetype::Healer => vec!["Mace", "Holy symbol", "Leather armor", "Healing herbs"],
        Archetype::Ranger => vec!["Short bow", "Quiver", "Leather armor", "Trail rations"],
        Archetype::Bard => vec!["Rapier", "Lute", "Fine clothes", "Inspiration charm"],
        Archetype::Paladin => vec!["Longsword", "Holy symbol", "Plate armor", "Healing potion"],
        Archetype::Necromancer => vec!["Bone staff", "Grimoire", "Dark robes", "Soul gem"],
    }
}
