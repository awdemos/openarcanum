use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Race {
    Human,
    ElfHigh,
    ElfWood,
    ElfDrow,
    DwarfHill,
    DwarfMountain,
    HalflingLightfoot,
    HalflingStout,
    Gnome,
    HalfElf,
    HalfOrc,
}

impl Race {
    pub fn name(&self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::ElfHigh => "High Elf",
            Race::ElfWood => "Wood Elf",
            Race::ElfDrow => "Drow",
            Race::DwarfHill => "Hill Dwarf",
            Race::DwarfMountain => "Mountain Dwarf",
            Race::HalflingLightfoot => "Lightfoot Halfling",
            Race::HalflingStout => "Stout Halfling",
            Race::Gnome => "Gnome",
            Race::HalfElf => "Half-Elf",
            Race::HalfOrc => "Half-Orc",
        }
    }

    pub fn ability_modifiers(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (0, 0, 0, 0, 0, 0),
            Race::ElfHigh => (0, 1, -1, 0, 0, 0),
            Race::ElfWood => (0, 0, 0, 1, 0, 0),
            Race::ElfDrow => (0, 2, -1, 0, 0, 1),
            Race::DwarfHill => (0, 0, 1, 0, 0, -1),
            Race::DwarfMountain => (1, 0, 1, 0, 0, -1),
            Race::HalflingLightfoot => (-1, 1, 0, 0, 0, 0),
            Race::HalflingStout => (0, 1, 1, 0, 0, 0),
            Race::Gnome => (0, 0, 0, 1, 0, -1),
            Race::HalfElf => (0, 0, 0, 0, 0, 0),
            Race::HalfOrc => (1, 0, 1, 0, 0, -2),
        }
    }

    pub fn minimum_abilities(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (3, 3, 3, 3, 3, 3),
            Race::ElfHigh | Race::ElfWood => (3, 6, 7, 8, 3, 8),
            Race::ElfDrow => (3, 8, 7, 9, 3, 8),
            Race::DwarfHill | Race::DwarfMountain => (8, 3, 12, 3, 3, 3),
            Race::HalflingLightfoot | Race::HalflingStout => (3, 6, 10, 6, 3, 3),
            Race::Gnome => (6, 3, 8, 6, 3, 3),
            Race::HalfElf => (3, 6, 6, 4, 3, 3),
            Race::HalfOrc => (6, 3, 13, 3, 3, 3),
        }
    }

    pub fn maximum_abilities(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (18, 18, 18, 18, 18, 18),
            Race::ElfHigh => (18, 19, 16, 18, 18, 18),
            Race::ElfWood => (18, 18, 17, 18, 18, 18),
            Race::ElfDrow => (18, 20, 16, 19, 19, 19),
            Race::DwarfHill => (18, 17, 19, 18, 18, 16),
            Race::DwarfMountain => (19, 17, 19, 18, 18, 16),
            Race::HalflingLightfoot => (17, 19, 19, 18, 17, 18),
            Race::HalflingStout => (18, 19, 19, 18, 17, 18),
            Race::Gnome => (18, 18, 18, 19, 18, 15),
            Race::HalfElf => (18, 18, 18, 18, 18, 18),
            Race::HalfOrc => (18, 17, 19, 17, 16, 12),
        }
    }

    pub fn allowed_classes(&self) -> HashMap<&'static str, Option<u32>> {
        let mut map = HashMap::new();
        match self {
            Race::Human => {
                map.insert("Fighter", None);
                map.insert("Paladin", None);
                map.insert("Ranger", None);
                map.insert("Magic-User", None);
                map.insert("Illusionist", None);
                map.insert("Cleric", None);
                map.insert("Druid", None);
                map.insert("Thief", None);
                map.insert("Assassin", None);
                map.insert("Monk", None);
            }
            Race::ElfHigh | Race::ElfWood => {
                map.insert("Fighter", Some(7));
                map.insert("Ranger", Some(10));
                map.insert("Magic-User", Some(11));
                map.insert("Cleric", Some(7));
                map.insert("Thief", Some(12));
                map.insert("Assassin", Some(10));
            }
            Race::ElfDrow => {
                map.insert("Fighter", Some(7));
                map.insert("Magic-User", Some(11));
                map.insert("Cleric", Some(7));
                map.insert("Thief", Some(12));
                map.insert("Assassin", Some(10));
            }
            Race::DwarfHill | Race::DwarfMountain => {
                map.insert("Fighter", Some(9));
                map.insert("Cleric", Some(8));
                map.insert("Thief", Some(12));
                map.insert("Assassin", Some(9));
            }
            Race::Gnome => {
                map.insert("Fighter", Some(6));
                map.insert("Cleric", Some(7));
                map.insert("Illusionist", Some(7));
                map.insert("Thief", Some(13));
            }
            Race::HalfElf => {
                map.insert("Fighter", Some(8));
                map.insert("Ranger", Some(8));
                map.insert("Magic-User", Some(8));
                map.insert("Cleric", Some(5));
                map.insert("Druid", Some(9));
                map.insert("Thief", Some(12));
                map.insert("Assassin", Some(11));
            }
            Race::HalflingLightfoot | Race::HalflingStout => {
                map.insert("Fighter", Some(6));
                map.insert("Cleric", Some(8));
                map.insert("Thief", Some(15));
            }
            Race::HalfOrc => {
                map.insert("Fighter", Some(10));
                map.insert("Cleric", Some(4));
                map.insert("Thief", Some(8));
                map.insert("Assassin", Some(9));
            }
        }
        map
    }

    pub fn special_abilities(&self) -> Vec<&'static str> {
        match self {
            Race::Human => vec!["No level limits", "Can be any class"],
            Race::ElfHigh => vec![
                "+1 DEX, -1 CON",
                "Infravision 60ft",
                "90% sleep/charm resist",
                "Secret door sense",
                "+1 hit with bows/swords",
            ],
            Race::ElfWood => vec![
                "+1 INT",
                "Infravision 60ft",
                "90% sleep/charm resist",
                "Secret door sense",
                "+1 hit with bows",
            ],
            Race::ElfDrow => vec![
                "+2 INT, -1 CON, +1 CHA",
                "Infravision 120ft",
                "90% sleep/charm resist",
                "Secret door sense",
                "+1 hit with bows/swords",
                "Innate spell-like abilities",
            ],
            Race::DwarfHill => vec![
                "+1 CON, -1 CHA",
                "Infravision 60ft",
                "Detect stonework",
                "+1 save vs magic",
                "-4 AC vs giants",
            ],
            Race::DwarfMountain => vec![
                "+1 STR, +1 CON, -1 CHA",
                "Infravision 60ft",
                "Detect stonework",
                "+1 save vs magic",
                "-4 AC vs giants",
            ],
            Race::HalflingLightfoot => vec![
                "-1 STR, +1 DEX",
                "+1 hit with slings/thrown",
                "-4 AC vs giants",
                "+1 save vs magic",
                "Hide in shadows",
            ],
            Race::HalflingStout => vec![
                "+1 DEX, +1 CON",
                "+1 hit with slings/thrown",
                "-4 AC vs giants",
                "+1 save vs magic",
                "Hide in shadows",
            ],
            Race::Gnome => vec![
                "+1 INT, -1 CHA",
                "Infravision 60ft",
                "Detect unsafe walls",
                "+1 save vs magic",
                "-4 AC vs giants",
            ],
            Race::HalfElf => vec![
                "Infravision 60ft",
                "30% sleep/charm resist",
                "Secret door sense",
            ],
            Race::HalfOrc => vec![
                "+1 STR, +1 CON, -2 CHA",
                "Infravision 60ft",
            ],
        }
    }
}

impl std::str::FromStr for Race {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" => Ok(Race::Human),
            "elf" | "high elf" | "highelf" => Ok(Race::ElfHigh),
            "wood elf" | "woodelf" => Ok(Race::ElfWood),
            "drow" => Ok(Race::ElfDrow),
            "dwarf" | "hill dwarf" | "hilldwarf" => Ok(Race::DwarfHill),
            "mountain dwarf" | "mountaindwarf" => Ok(Race::DwarfMountain),
            "halfling" | "lightfoot halfling" | "lightfoothalfling" => Ok(Race::HalflingLightfoot),
            "stout halfling" | "stouthalfling" => Ok(Race::HalflingStout),
            "gnome" => Ok(Race::Gnome),
            "half-elf" | "halfelf" => Ok(Race::HalfElf),
            "half-orc" | "halforc" => Ok(Race::HalfOrc),
            _ => Err(format!("Unknown race: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Class {
    Fighter,
    Paladin,
    Ranger,
    MagicUser,
    Illusionist,
    Cleric,
    Druid,
    Thief,
    Assassin,
    Monk,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Fighter => "Fighter",
            Class::Paladin => "Paladin",
            Class::Ranger => "Ranger",
            Class::MagicUser => "Magic-User",
            Class::Illusionist => "Illusionist",
            Class::Cleric => "Cleric",
            Class::Druid => "Druid",
            Class::Thief => "Thief",
            Class::Assassin => "Assassin",
            Class::Monk => "Monk",
        }
    }

    pub fn group(&self) -> ClassGroup {
        match self {
            Class::Fighter | Class::Paladin | Class::Ranger => ClassGroup::Warrior,
            Class::MagicUser | Class::Illusionist => ClassGroup::Wizard,
            Class::Cleric | Class::Druid => ClassGroup::Priest,
            Class::Thief | Class::Assassin => ClassGroup::Rogue,
            Class::Monk => ClassGroup::Monk,
        }
    }

    pub fn hit_die(&self) -> u32 {
        match self {
            Class::Fighter | Class::Paladin => 10,
            Class::Ranger => 8,
            Class::MagicUser | Class::Illusionist | Class::Monk => 4,
            Class::Cleric | Class::Druid => 8,
            Class::Thief | Class::Assassin => 6,
        }
    }

    pub fn ability_requirements(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Class::Fighter => (9, 3, 3, 3, 3, 3),
            Class::Paladin => (12, 9, 9, 3, 13, 17),
            Class::Ranger => (13, 13, 14, 3, 14, 3),
            Class::MagicUser => (3, 9, 3, 3, 3, 3),
            Class::Illusionist => (3, 9, 3, 3, 3, 3),
            Class::Cleric => (3, 3, 3, 3, 9, 3),
            Class::Druid => (3, 3, 3, 3, 12, 15),
            Class::Thief => (3, 9, 3, 3, 3, 3),
            Class::Assassin => (12, 11, 3, 3, 3, 3),
            Class::Monk => (15, 3, 3, 3, 15, 3),
        }
    }

    pub fn prime_requisites(&self) -> Vec<&'static str> {
        match self {
            Class::Fighter => vec!["str"],
            Class::Paladin => vec!["str", "wis"],
            Class::Ranger => vec!["str", "dex", "wis"],
            Class::MagicUser => vec!["int"],
            Class::Illusionist => vec!["int"],
            Class::Cleric => vec!["wis"],
            Class::Druid => vec!["wis"],
            Class::Thief => vec!["dex"],
            Class::Assassin => vec!["dex"],
            Class::Monk => vec!["str", "wis"],
        }
    }

    pub fn initial_weapon_proficiencies(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 4,
            ClassGroup::Wizard | ClassGroup::Monk => 1,
            ClassGroup::Priest => 2,
            ClassGroup::Rogue => 2,
        }
    }

    pub fn weapon_proficiency_rate(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 1,
            ClassGroup::Wizard | ClassGroup::Priest | ClassGroup::Rogue | ClassGroup::Monk => 1,
        }
    }

    pub fn non_proficiency_penalty(&self) -> i32 {
        match self.group() {
            ClassGroup::Warrior => -2,
            ClassGroup::Wizard => -5,
            ClassGroup::Priest | ClassGroup::Rogue => -3,
            ClassGroup::Monk => -3,
        }
    }

    pub fn casts_spells(&self) -> bool {
        matches!(
            self,
            Class::MagicUser | Class::Illusionist | Class::Cleric | Class::Druid
        )
    }

    pub fn spell_type(&self) -> Option<SpellType> {
        match self {
            Class::MagicUser | Class::Illusionist => Some(SpellType::Arcane),
            Class::Cleric | Class::Druid => Some(SpellType::Divine),
            _ => None,
        }
    }

    pub fn allowed_alignments(&self) -> Vec<Alignment> {
        match self {
            Class::Paladin => vec![Alignment::LawfulGood],
            Class::Ranger => vec![
                Alignment::LawfulGood,
                Alignment::NeutralGood,
                Alignment::ChaoticGood,
            ],
            Class::Monk => vec![
                Alignment::LawfulGood,
                Alignment::LawfulNeutral,
                Alignment::LawfulEvil,
            ],
            Class::Assassin => vec![
                Alignment::LawfulEvil,
                Alignment::NeutralEvil,
                Alignment::ChaoticEvil,
            ],
            Class::Druid => vec![Alignment::TrueNeutral],
            _ => Alignment::all(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fighter" => Some(Class::Fighter),
            "paladin" => Some(Class::Paladin),
            "ranger" => Some(Class::Ranger),
            "magic-user" | "magicuser" | "magic user" | "mage" | "wizard" => Some(Class::MagicUser),
            "illusionist" => Some(Class::Illusionist),
            "cleric" => Some(Class::Cleric),
            "druid" => Some(Class::Druid),
            "thief" => Some(Class::Thief),
            "assassin" => Some(Class::Assassin),
            "monk" => Some(Class::Monk),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassGroup {
    Warrior,
    Wizard,
    Priest,
    Rogue,
    Monk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellType {
    Arcane,
    Divine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

impl Alignment {
    pub fn name(&self) -> &'static str {
        match self {
            Alignment::LawfulGood => "Lawful Good",
            Alignment::LawfulNeutral => "Lawful Neutral",
            Alignment::LawfulEvil => "Lawful Evil",
            Alignment::NeutralGood => "Neutral Good",
            Alignment::TrueNeutral => "True Neutral",
            Alignment::NeutralEvil => "Neutral Evil",
            Alignment::ChaoticGood => "Chaotic Good",
            Alignment::ChaoticNeutral => "Chaotic Neutral",
            Alignment::ChaoticEvil => "Chaotic Evil",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Alignment::LawfulGood,
            Alignment::LawfulNeutral,
            Alignment::LawfulEvil,
            Alignment::NeutralGood,
            Alignment::TrueNeutral,
            Alignment::NeutralEvil,
            Alignment::ChaoticGood,
            Alignment::ChaoticNeutral,
            Alignment::ChaoticEvil,
        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lawful good" | "lg" => Some(Alignment::LawfulGood),
            "lawful neutral" | "ln" => Some(Alignment::LawfulNeutral),
            "lawful evil" | "le" => Some(Alignment::LawfulEvil),
            "neutral good" | "ng" => Some(Alignment::NeutralGood),
            "true neutral" | "neutral" | "tn" | "n" => Some(Alignment::TrueNeutral),
            "neutral evil" | "ne" => Some(Alignment::NeutralEvil),
            "chaotic good" | "cg" => Some(Alignment::ChaoticGood),
            "chaotic neutral" | "cn" => Some(Alignment::ChaoticNeutral),
            "chaotic evil" | "ce" => Some(Alignment::ChaoticEvil),
            _ => None,
        }
    }
}

pub fn thac0(group: ClassGroup, level: u32) -> i32 {
    let lvl = level.min(20) as usize;
    let table = match group {
        ClassGroup::Warrior => &[
            20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ],
        ClassGroup::Priest => &[
            20, 20, 20, 18, 18, 18, 16, 16, 16, 14,
            14, 14, 12, 12, 12, 10, 10, 10, 8, 8,
        ],
        ClassGroup::Rogue => &[
            20, 20, 20, 20, 19, 19, 19, 19, 18, 18,
            18, 18, 17, 17, 17, 17, 16, 16, 16, 16,
        ],
        ClassGroup::Wizard => &[
            20, 20, 20, 19, 19, 19, 18, 18, 18, 17,
            17, 17, 16, 16, 16, 15, 15, 15, 14, 14,
        ],
        ClassGroup::Monk => &[
            20, 20, 20, 19, 19, 19, 18, 18, 18, 17,
            17, 17, 16, 16, 16, 15, 15, 15, 14, 14,
        ],
    };
    table[lvl.saturating_sub(1)]
}

pub fn saving_throws(group: ClassGroup, level: u32) -> (i32, i32, i32, i32, i32) {
    let lvl = level.min(20) as usize;
    let table = match group {
        ClassGroup::Warrior => &[
            (14, 16, 15, 17, 17), (14, 16, 15, 17, 17), (13, 15, 14, 16, 16),
            (13, 15, 14, 16, 16), (11, 13, 12, 13, 14), (11, 13, 12, 13, 14),
            (10, 12, 11, 12, 13), (10, 12, 11, 12, 13), (8, 10, 9, 9, 11),
            (8, 10, 9, 9, 11), (7, 9, 8, 8, 10), (7, 9, 8, 8, 10),
            (5, 7, 6, 5, 8), (5, 7, 6, 5, 8), (4, 6, 5, 4, 7),
            (4, 6, 5, 4, 7), (3, 5, 4, 4, 6), (3, 5, 4, 4, 6),
            (3, 5, 4, 4, 6), (3, 5, 4, 4, 6),
        ],
        ClassGroup::Priest => &[
            (10, 14, 13, 16, 15), (10, 14, 13, 16, 15), (10, 14, 13, 16, 15),
            (9, 13, 12, 15, 14), (9, 13, 12, 15, 14), (9, 13, 12, 15, 14),
            (7, 11, 10, 13, 12), (7, 11, 10, 13, 12), (7, 11, 10, 13, 12),
            (6, 10, 9, 12, 11), (6, 10, 9, 12, 11), (6, 10, 9, 12, 11),
            (5, 9, 8, 11, 10), (5, 9, 8, 11, 10), (5, 9, 8, 11, 10),
            (4, 8, 7, 10, 9), (4, 8, 7, 10, 9), (4, 8, 7, 10, 9),
            (2, 6, 5, 8, 7), (2, 6, 5, 8, 7),
        ],
        ClassGroup::Rogue => &[
            (13, 14, 12, 16, 15), (13, 14, 12, 16, 15), (13, 14, 12, 16, 15),
            (13, 14, 12, 16, 15), (12, 12, 11, 15, 13), (12, 12, 11, 15, 13),
            (12, 12, 11, 15, 13), (10, 10, 9, 13, 11), (10, 10, 9, 13, 11),
            (10, 10, 9, 13, 11), (8, 8, 7, 11, 9), (8, 8, 7, 11, 9),
            (8, 8, 7, 11, 9), (6, 6, 5, 9, 7), (6, 6, 5, 9, 7),
            (6, 6, 5, 9, 7), (4, 4, 3, 7, 5), (4, 4, 3, 7, 5),
            (4, 4, 3, 7, 5), (4, 4, 3, 7, 5),
        ],
        ClassGroup::Wizard => &[
            (14, 11, 13, 15, 12), (14, 11, 13, 15, 12), (14, 11, 13, 15, 12),
            (13, 9, 11, 13, 10), (13, 9, 11, 13, 10), (13, 9, 11, 13, 10),
            (11, 7, 9, 11, 8), (11, 7, 9, 11, 8), (11, 7, 9, 11, 8),
            (10, 5, 7, 9, 6), (10, 5, 7, 9, 6), (10, 5, 7, 9, 6),
            (8, 3, 5, 7, 4), (8, 3, 5, 7, 4), (8, 3, 5, 7, 4),
            (7, 2, 3, 5, 3), (7, 2, 3, 5, 3), (7, 2, 3, 5, 3),
            (5, 2, 3, 5, 3), (5, 2, 3, 5, 3),
        ],
        ClassGroup::Monk => &[
            (13, 14, 12, 16, 15), (13, 14, 12, 16, 15), (13, 14, 12, 16, 15),
            (13, 14, 12, 16, 15), (12, 12, 11, 15, 13), (12, 12, 11, 15, 13),
            (12, 12, 11, 15, 13), (10, 10, 9, 13, 11), (10, 10, 9, 13, 11),
            (10, 10, 9, 13, 11), (8, 8, 7, 11, 9), (8, 8, 7, 11, 9),
            (8, 8, 7, 11, 9), (6, 6, 5, 9, 7), (6, 6, 5, 9, 7),
            (6, 6, 5, 9, 7), (4, 4, 3, 7, 5), (4, 4, 3, 7, 5),
            (4, 4, 3, 7, 5), (4, 4, 3, 7, 5),
        ],
    };
    table[lvl.saturating_sub(1)]
}

pub fn xp_for_level(class: &Class, level: u32) -> u32 {
    if level <= 1 {
        return 0;
    }
    let lvl = level.min(20) as usize;
    let table = match class {
        Class::Fighter => &[
            0, 2000, 4000, 8000, 16000, 32000, 64000, 125000, 250000, 500000,
            750000, 1000000, 1250000, 1500000, 1750000, 2000000, 2250000, 2500000,
            2750000, 3000000,
        ],
        Class::Paladin => &[
            0, 2750, 5500, 11000, 22500, 45000, 95000, 175000, 350000, 700000,
            1050000, 1400000, 1750000, 2100000, 2450000, 2800000, 3150000, 3500000,
            3850000, 4200000,
        ],
        Class::Ranger => &[
            0, 2250, 4500, 10000, 20000, 40000, 90000, 150000, 300000, 600000,
            900000, 1200000, 1500000, 1800000, 2100000, 2400000, 2700000, 3000000,
            3300000, 3600000,
        ],
        Class::MagicUser => &[
            0, 2500, 5000, 10000, 20000, 40000, 60000, 90000, 135000, 250000,
            375000, 750000, 1125000, 1500000, 1875000, 2250000, 2625000, 3000000,
            3375000, 3750000,
        ],
        Class::Illusionist => &[
            0, 2250, 4500, 9000, 18000, 35000, 60000, 95000, 145000, 250000,
            375000, 750000, 1125000, 1500000, 1875000, 2250000, 2625000, 3000000,
            3375000, 3750000,
        ],
        Class::Cleric => &[
            0, 1500, 3000, 6000, 13000, 27500, 55000, 110000, 225000, 450000,
            675000, 900000, 1125000, 1350000, 1575000, 1800000, 2025000, 2250000,
            2475000, 2700000,
        ],
        Class::Druid => &[
            0, 2000, 4000, 7500, 12500, 20000, 35000, 60000, 90000, 125000,
            200000, 300000, 750000, 1500000, 3000000, 3500000, 4000000, 4500000,
            5000000, 5500000,
        ],
        Class::Thief => &[
            0, 1250, 2500, 5000, 10000, 20000, 40000, 70000, 110000, 160000,
            220000, 440000, 660000, 880000, 1100000, 1320000, 1540000, 1760000,
            1980000, 2200000,
        ],
        Class::Assassin => &[
            0, 1500, 3000, 6000, 12000, 25000, 50000, 100000, 200000, 300000,
            425000, 575000, 750000, 1000000, 1500000, 2000000, 2500000, 3000000,
            3500000, 4000000,
        ],
        Class::Monk => &[
            0, 2000, 4000, 7500, 12500, 20000, 35000, 60000, 100000, 150000,
            225000, 325000, 475000, 700000, 1000000, 1500000, 2000000, 2500000,
            3000000, 3500000,
        ],
    };
    table[lvl.saturating_sub(1)]
}

pub fn arcane_spell_slots(caster_level: u32) -> Vec<u32> {
    let table = [
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![2, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![3, 2, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 2, 1, 0, 0, 0, 0, 0, 0],
        vec![4, 2, 2, 0, 0, 0, 0, 0, 0],
        vec![4, 3, 2, 1, 0, 0, 0, 0, 0],
        vec![4, 3, 3, 2, 0, 0, 0, 0, 0],
        vec![4, 3, 3, 2, 1, 0, 0, 0, 0],
        vec![4, 4, 3, 2, 2, 0, 0, 0, 0],
        vec![4, 4, 4, 3, 3, 0, 0, 0, 0],
        vec![4, 4, 4, 4, 4, 1, 0, 0, 0],
        vec![5, 5, 5, 4, 4, 2, 0, 0, 0],
        vec![5, 5, 5, 4, 4, 3, 1, 0, 0],
        vec![5, 5, 5, 5, 5, 3, 2, 0, 0],
        vec![5, 5, 5, 5, 5, 4, 3, 1, 0],
        vec![5, 5, 5, 5, 5, 5, 4, 2, 0],
        vec![5, 5, 5, 5, 5, 5, 5, 3, 1],
        vec![5, 5, 5, 5, 5, 5, 5, 4, 2],
        vec![5, 5, 5, 5, 5, 5, 5, 5, 3],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn divine_spell_slots(caster_level: u32) -> Vec<u32> {
    let table = [
        vec![1, 0, 0, 0, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0, 0],
        vec![2, 1, 0, 0, 0, 0, 0],
        vec![3, 2, 0, 0, 0, 0, 0],
        vec![3, 3, 1, 0, 0, 0, 0],
        vec![3, 3, 2, 0, 0, 0, 0],
        vec![3, 3, 2, 1, 0, 0, 0],
        vec![3, 3, 3, 2, 0, 0, 0],
        vec![4, 4, 3, 2, 1, 0, 0],
        vec![4, 4, 3, 3, 2, 0, 0],
        vec![5, 4, 4, 3, 2, 1, 0],
        vec![6, 5, 5, 3, 2, 2, 0],
        vec![6, 6, 6, 4, 2, 2, 0],
        vec![6, 6, 6, 5, 3, 2, 1],
        vec![6, 6, 6, 6, 4, 2, 1],
        vec![7, 7, 7, 6, 4, 3, 1],
        vec![7, 7, 7, 7, 5, 3, 2],
        vec![8, 8, 8, 8, 6, 4, 2],
        vec![9, 9, 8, 8, 6, 4, 2],
        vec![9, 9, 9, 8, 7, 5, 2],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn strength_table(score: i32) -> (i32, i32, i32, i32, i32, i32) {
    match score {
        1 => (-5, -4, 1, 3, 1, 0),
        2 => (-3, -2, 1, 5, 1, 0),
        3 => (-3, -1, 5, 10, 2, 0),
        4 => (-2, -1, 10, 25, 3, 0),
        5 => (-2, -1, 10, 25, 3, 0),
        6 => (-1, 0, 20, 55, 4, 0),
        7 => (-1, 0, 20, 55, 4, 0),
        8 => (0, 0, 35, 90, 5, 1),
        9 => (0, 0, 35, 90, 5, 1),
        10 => (0, 0, 40, 115, 6, 2),
        11 => (0, 0, 40, 115, 6, 2),
        12 => (0, 0, 45, 140, 7, 4),
        13 => (0, 0, 45, 140, 7, 4),
        14 => (0, 0, 55, 170, 8, 7),
        15 => (0, 0, 55, 170, 8, 7),
        16 => (0, 1, 70, 195, 9, 10),
        17 => (1, 1, 85, 220, 10, 13),
        18 => (1, 2, 110, 255, 11, 16),
        19..=25 => (3, 7, 485, 640, 16, 50),
        _ => (0, 0, 0, 0, 0, 0),
    }
}

pub fn exceptional_strength(percentile: i32) -> (i32, i32, i32, i32, i32, i32) {
    match percentile {
        1..=50 => (1, 3, 135, 280, 12, 20),
        51..=75 => (2, 3, 160, 305, 13, 25),
        76..=90 => (2, 4, 185, 330, 14, 30),
        91..=99 => (2, 5, 235, 380, 15, 35),
        0 | 100 => (3, 6, 335, 480, 16, 40),
        _ => (1, 2, 110, 255, 11, 16),
    }
}

pub fn dexterity_table(score: i32) -> (i32, i32, i32) {
    match score {
        1 => (-6, -6, 5),
        2 => (-4, -4, 5),
        3 => (-3, -3, 4),
        4 => (-2, -2, 3),
        5 => (-1, -1, 2),
        6 => (0, 0, 1),
        7 => (0, 0, 0),
        8..=14 => (0, 0, 0),
        15 => (0, 0, -1),
        16 => (1, 1, -2),
        17 => (2, 2, -3),
        18 => (2, 2, -4),
        19..=25 => (3, 3, -4),
        _ => (0, 0, 0),
    }
}

pub fn constitution_table(score: i32) -> (i32, i32, i32) {
    match score {
        1 => (-3, 25, 30),
        2 => (-2, 30, 35),
        3 => (-2, 35, 40),
        4 => (-1, 40, 45),
        5 => (-1, 45, 50),
        6 => (-1, 50, 55),
        7 => (0, 55, 60),
        8..=12 => (0, 65, 70),
        13..=14 => (0, 75, 80),
        15 => (1, 80, 85),
        16 => (2, 85, 90),
        17 => (2, 95, 96),
        18 => (2, 99, 100),
        19..=25 => (2, 99, 100),
        _ => (0, 0, 0),
    }
}

pub fn warrior_hp_bonus(con: i32) -> i32 {
    match con {
        17 => 3,
        18..=25 => 4,
        _ => 0,
    }
}

pub fn intelligence_table(score: i32) -> (i32, i32, i32, i32) {
    match score {
        1 => (0, 0, 0, 0),
        2..=7 => (1, 0, 0, 0),
        8 => (1, 0, 0, 0),
        9 => (2, 4, 35, 6),
        10 => (2, 5, 40, 7),
        11 => (2, 5, 45, 7),
        12 => (3, 6, 50, 7),
        13 => (3, 6, 55, 9),
        14 => (4, 7, 60, 9),
        15 => (4, 7, 65, 11),
        16 => (5, 8, 70, 11),
        17 => (6, 8, 75, 14),
        18 => (7, 9, 85, 18),
        19..=25 => (8, 9, 95, 99),
        _ => (0, 0, 0, 0),
    }
}

pub fn wisdom_table(score: i32) -> (i32, Vec<(u32, u32)>, i32) {
    let bonus = match score {
        1..=11 => vec![],
        12 => vec![(1, 1)],
        13 => vec![(1, 1), (2, 1)],
        14 => vec![(1, 1), (2, 1)],
        15 => vec![(1, 2), (2, 1)],
        16 => vec![(1, 2), (2, 1), (3, 1)],
        17 => vec![(1, 3), (2, 2), (3, 1), (4, 1)],
        18 => vec![(1, 4), (2, 3), (3, 2), (4, 2)],
        19..=25 => vec![(1, 5), (2, 4), (3, 3), (4, 3), (5, 2)],
        _ => vec![],
    };
    let defense = match score {
        1..=12 => 0,
        13..=14 => 1,
        15 => 1,
        16 => 2,
        17 => 3,
        18..=25 => 4,
        _ => 0,
    };
    let failure = match score {
        1..=11 => 100 - (score - 1) * 5,
        12 => 25,
        13 => 20,
        14 => 15,
        15 => 10,
        16 => 5,
        17..=25 => 0,
        _ => 100,
    };
    (defense, bonus, failure)
}

pub fn charisma_table(score: i32) -> (i32, i32, i32) {
    match score {
        1 => (0, -8, -7),
        2 => (1, -7, -6),
        3 => (1, -6, -5),
        4 => (1, -5, -4),
        5 => (2, -4, -3),
        6 => (2, -3, -2),
        7 => (3, -2, -1),
        8..=12 => (4, 0, 0),
        13 => (5, 0, 1),
        14 => (6, 1, 2),
        15 => (7, 3, 3),
        16 => (8, 4, 5),
        17 => (10, 6, 6),
        18 => (15, 8, 7),
        19..=25 => (20, 10, 8),
        _ => (0, 0, 0),
    }
}

pub fn starting_gold(class: &Class) -> (u32, u32, u32) {
    match class {
        Class::Fighter | Class::Paladin | Class::Ranger => (5, 10, 1),
        Class::MagicUser | Class::Illusionist => (2, 4, 10),
        Class::Cleric | Class::Druid => (3, 6, 10),
        Class::Thief | Class::Assassin | Class::Monk => (2, 6, 10),
    }
}

pub fn thief_skill_bases() -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    (15, 10, 5, 10, 5, 15, 60, 0)
}

pub fn dexterity_thief_adjustment(dex: i32) -> (i32, i32, i32, i32, i32) {
    match dex {
        9 => (-15, -10, -10, -20, -10),
        10 => (-10, -5, -10, -15, -5),
        11 => (-5, 0, -5, -10, 0),
        12 => (0, 0, 0, -5, 0),
        13..=15 => (0, 0, 0, 0, 0),
        16 => (0, 5, 0, 0, 0),
        17 => (5, 10, 0, 5, 5),
        18 => (10, 15, 5, 10, 10),
        19..=25 => (15, 20, 10, 15, 15),
        _ => (0, 0, 0, 0, 0),
    }
}

pub fn thief_skill_advancement(level: u32) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let lvl = level as i32;
    (
        lvl * 5,
        lvl * 4,
        lvl * 4,
        lvl * 5,
        lvl * 4,
        lvl * 3,
        lvl * 2,
        if level >= 4 { (lvl - 3) * 5 } else { 0 },
    )
}

pub const BASE_AC: i32 = 10;

pub fn armor_types() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("none", 10);
    map.insert("shield_only", 9);
    map.insert("leather", 8);
    map.insert("padded", 8);
    map.insert("studded_leather", 7);
    map.insert("ring_mail", 7);
    map.insert("scale_mail", 6);
    map.insert("chain_mail", 5);
    map.insert("splint_mail", 4);
    map.insert("banded_mail", 4);
    map.insert("plate_mail", 3);
    map.insert("field_plate", 2);
    map.insert("full_plate", 1);
    map
}

pub fn xp_bonus(prime_scores: &[i32]) -> i32 {
    let all_16_or_higher = prime_scores.iter().all(|&s| s >= 16);
    let all_13_or_higher = prime_scores.iter().all(|&s| s >= 13);

    if all_16_or_higher {
        10
    } else if all_13_or_higher {
        5
    } else if prime_scores.iter().any(|&s| s < 9) && prime_scores.iter().any(|&s| s < 6) {
        -20
    } else if prime_scores.iter().any(|&s| s < 9) {
        -10
    } else {
        0
    }
}
