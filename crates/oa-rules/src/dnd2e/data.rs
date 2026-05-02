use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Race {
    Human,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    Halfling,
    HalfOrc,
}

impl Race {
    pub fn name(&self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::Dwarf => "Dwarf",
            Race::Elf => "Elf",
            Race::Gnome => "Gnome",
            Race::HalfElf => "Half-Elf",
            Race::Halfling => "Halfling",
            Race::HalfOrc => "Half-Orc",
        }
    }

    pub fn ability_modifiers(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (0, 0, 0, 0, 0, 0),
            Race::Dwarf => (0, 0, 1, 0, 0, -1),
            Race::Elf => (0, 1, -1, 0, 0, 0),
            Race::Gnome => (0, 0, 0, 1, 0, -1),
            Race::HalfElf => (0, 0, 0, 0, 0, 0),
            Race::Halfling => (-1, 1, 0, 0, 0, 0),
            Race::HalfOrc => (1, 0, 0, -1, 0, -2),
        }
    }

    pub fn minimum_abilities(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (3, 3, 3, 3, 3, 3),
            Race::Dwarf => (8, 3, 12, 3, 3, 3),
            Race::Elf => (3, 6, 7, 8, 3, 8),
            Race::Gnome => (6, 3, 8, 6, 3, 3),
            Race::HalfElf => (3, 6, 6, 4, 3, 3),
            Race::Halfling => (3, 6, 10, 6, 3, 3),
            Race::HalfOrc => (6, 3, 13, 3, 3, 3),
        }
    }

    pub fn maximum_abilities(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (18, 18, 18, 18, 18, 18),
            Race::Dwarf => (18, 17, 19, 18, 18, 16),
            Race::Elf => (18, 19, 16, 18, 18, 18),
            Race::Gnome => (18, 18, 18, 19, 18, 15),
            Race::HalfElf => (18, 18, 18, 18, 18, 18),
            Race::Halfling => (17, 19, 19, 18, 17, 18),
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
                map.insert("Mage", None);
                map.insert("Abjurer", None);
                map.insert("Conjurer", None);
                map.insert("Diviner", None);
                map.insert("Enchanter", None);
                map.insert("Illusionist", None);
                map.insert("Invoker", None);
                map.insert("Necromancer", None);
                map.insert("Transmuter", None);
                map.insert("Cleric", None);
                map.insert("Druid", None);
                map.insert("Thief", None);
                map.insert("Bard", None);
            }
            Race::Dwarf => {
                map.insert("Fighter", Some(15));
                map.insert("Cleric", Some(10));
                map.insert("Thief", Some(12));
            }
            Race::Elf => {
                map.insert("Fighter", Some(12));
                map.insert("Ranger", Some(15));
                map.insert("Mage", Some(15));
                map.insert("Cleric", Some(12));
                map.insert("Thief", Some(12));
                map.insert("Bard", Some(12));
            }
            Race::Gnome => {
                map.insert("Fighter", Some(11));
                map.insert("Cleric", Some(9));
                map.insert("Illusionist", Some(15));
                map.insert("Thief", Some(13));
            }
            Race::HalfElf => {
                map.insert("Fighter", Some(14));
                map.insert("Ranger", Some(16));
                map.insert("Mage", Some(12));
                map.insert("Cleric", Some(14));
                map.insert("Druid", Some(9));
                map.insert("Thief", Some(12));
                map.insert("Bard", None);
            }
            Race::Halfling => {
                map.insert("Fighter", Some(9));
                map.insert("Cleric", Some(8));
                map.insert("Thief", Some(15));
            }
            Race::HalfOrc => {
                map.insert("Fighter", Some(10));
                map.insert("Cleric", Some(8));
                map.insert("Thief", Some(8));
            }
        }
        map
    }

    pub fn special_abilities(&self) -> Vec<&'static str> {
        match self {
            Race::Human => vec!["No level limits", "Can be any class"],
            Race::Dwarf => vec![
                "+1 CON, -1 CHA",
                "Infravision 60ft",
                "Detect stonework",
                "+1 save vs magic",
                "-4 AC vs giants",
            ],
            Race::Elf => vec![
                "+1 DEX, -1 CON",
                "Infravision 60ft",
                "90% sleep/charm resist",
                "Secret door sense",
                "+1 hit bows/swords",
            ],
            Race::Gnome => vec![
                "+1 INT, -1 WIS",
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
            Race::Halfling => vec![
                "-1 STR, +1 DEX",
                "+1 hit slings/thrown",
                "-4 AC vs giants",
                "+1 save vs magic",
                "Hide in shadows",
            ],
            Race::HalfOrc => vec![
                "+1 STR, -1 INT, -2 CHA",
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
            "dwarf" => Ok(Race::Dwarf),
            "elf" => Ok(Race::Elf),
            "gnome" => Ok(Race::Gnome),
            "half-elf" | "halfelf" => Ok(Race::HalfElf),
            "halfling" => Ok(Race::Halfling),
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
    Mage,
    Abjurer,
    Conjurer,
    Diviner,
    Enchanter,
    Illusionist,
    Invoker,
    Necromancer,
    Transmuter,
    Cleric,
    Druid,
    Thief,
    Bard,
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Fighter => "Fighter",
            Class::Paladin => "Paladin",
            Class::Ranger => "Ranger",
            Class::Mage => "Mage",
            Class::Abjurer => "Abjurer",
            Class::Conjurer => "Conjurer",
            Class::Diviner => "Diviner",
            Class::Enchanter => "Enchanter",
            Class::Illusionist => "Illusionist",
            Class::Invoker => "Invoker",
            Class::Necromancer => "Necromancer",
            Class::Transmuter => "Transmuter",
            Class::Cleric => "Cleric",
            Class::Druid => "Druid",
            Class::Thief => "Thief",
            Class::Bard => "Bard",
        }
    }

    pub fn group(&self) -> ClassGroup {
        match self {
            Class::Fighter | Class::Paladin | Class::Ranger => ClassGroup::Warrior,
            Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
            Class::Enchanter | Class::Illusionist | Class::Invoker |
            Class::Necromancer | Class::Transmuter => ClassGroup::Wizard,
            Class::Cleric | Class::Druid => ClassGroup::Priest,
            Class::Thief | Class::Bard => ClassGroup::Rogue,
        }
    }

    pub fn hit_die(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 10,
            ClassGroup::Wizard => 4,
            ClassGroup::Priest => 8,
            ClassGroup::Rogue => 6,
        }
    }

    pub fn ability_requirements(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Class::Fighter => (9, 3, 3, 3, 3, 3),
            Class::Paladin => (12, 3, 9, 3, 13, 17),
            Class::Ranger => (13, 13, 14, 3, 14, 3),
            Class::Mage => (3, 3, 3, 9, 3, 3),
            Class::Abjurer => (3, 3, 3, 9, 3, 3),
            Class::Conjurer => (3, 3, 3, 9, 3, 3),
            Class::Diviner => (3, 3, 3, 9, 3, 3),
            Class::Enchanter => (3, 3, 3, 9, 3, 3),
            Class::Illusionist => (3, 3, 3, 9, 3, 3),
            Class::Invoker => (3, 3, 3, 9, 3, 3),
            Class::Necromancer => (3, 3, 3, 9, 3, 3),
            Class::Transmuter => (3, 3, 3, 9, 3, 3),
            Class::Cleric => (3, 3, 3, 3, 9, 3),
            Class::Druid => (3, 3, 3, 3, 12, 15),
            Class::Thief => (3, 9, 3, 3, 3, 3),
            Class::Bard => (3, 12, 3, 13, 3, 15),
        }
    }

    pub fn prime_requisites(&self) -> Vec<&'static str> {
        match self {
            Class::Fighter => vec!["str"],
            Class::Paladin => vec!["str", "wis"],
            Class::Ranger => vec!["str", "dex", "wis"],
            Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
            Class::Enchanter | Class::Illusionist | Class::Invoker |
            Class::Necromancer | Class::Transmuter => vec!["int"],
            Class::Cleric | Class::Druid => vec!["wis"],
            Class::Thief => vec!["dex"],
            Class::Bard => vec!["dex", "int", "cha"],
        }
    }

    pub fn initial_weapon_proficiencies(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 4,
            ClassGroup::Wizard => 1,
            ClassGroup::Priest => 2,
            ClassGroup::Rogue => 2,
        }
    }

    pub fn weapon_proficiency_rate(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 1,
            ClassGroup::Wizard | ClassGroup::Priest | ClassGroup::Rogue => 1,
        }
    }

    pub fn non_proficiency_penalty(&self) -> i32 {
        match self.group() {
            ClassGroup::Warrior => -2,
            ClassGroup::Wizard => -5,
            ClassGroup::Priest | ClassGroup::Rogue => -3,
        }
    }

    pub fn initial_nonweapon_proficiencies(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 3,
            ClassGroup::Wizard => 4,
            ClassGroup::Priest => 4,
            ClassGroup::Rogue => 3,
        }
    }

    pub fn nonweapon_proficiency_rate(&self) -> u32 {
        match self.group() {
            ClassGroup::Warrior => 1,
            ClassGroup::Wizard | ClassGroup::Priest | ClassGroup::Rogue => 1,
        }
    }

    pub fn casts_spells(&self) -> bool {
        matches!(
            self,
            Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
            Class::Enchanter | Class::Illusionist | Class::Invoker |
            Class::Necromancer | Class::Transmuter |
            Class::Cleric | Class::Druid | Class::Bard
        )
    }

    pub fn spell_type(&self) -> Option<SpellType> {
        match self {
            Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
            Class::Enchanter | Class::Illusionist | Class::Invoker |
            Class::Necromancer | Class::Transmuter => Some(SpellType::Arcane),
            Class::Cleric | Class::Druid => Some(SpellType::Divine),
            Class::Bard => Some(SpellType::Bardic),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fighter" => Some(Class::Fighter),
            "paladin" => Some(Class::Paladin),
            "ranger" => Some(Class::Ranger),
            "mage" | "wizard" => Some(Class::Mage),
            "abjurer" => Some(Class::Abjurer),
            "conjurer" => Some(Class::Conjurer),
            "diviner" => Some(Class::Diviner),
            "enchanter" => Some(Class::Enchanter),
            "illusionist" => Some(Class::Illusionist),
            "invoker" => Some(Class::Invoker),
            "necromancer" => Some(Class::Necromancer),
            "transmuter" => Some(Class::Transmuter),
            "cleric" => Some(Class::Cleric),
            "druid" => Some(Class::Druid),
            "thief" => Some(Class::Thief),
            "bard" => Some(Class::Bard),
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellType {
    Arcane,
    Divine,
    Bardic,
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
            20, 20, 19, 19, 18, 18, 17, 17, 16, 16,
            15, 15, 14, 14, 13, 13, 12, 12, 11, 11,
        ],
        ClassGroup::Wizard => &[
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
        Class::Paladin | Class::Ranger => &[
            0, 2250, 4500, 9000, 18000, 36000, 75000, 150000, 300000, 600000,
            900000, 1200000, 1500000, 1800000, 2100000, 2400000, 2700000, 3000000,
            3300000, 3600000,
        ],
        Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
        Class::Enchanter | Class::Illusionist | Class::Invoker |
        Class::Necromancer | Class::Transmuter => &[
            0, 2500, 5000, 10000, 20000, 40000, 60000, 90000, 135000, 250000,
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
            200000, 300000, 750000, 1500000, 2250000, 3000000, 3750000, 4500000,
            5250000, 6000000,
        ],
        Class::Thief => &[
            0, 1250, 2500, 5000, 10000, 20000, 40000, 70000, 110000, 160000,
            220000, 440000, 660000, 880000, 1100000, 1320000, 1540000, 1760000,
            1980000, 2200000,
        ],
        Class::Bard => &[
            0, 2000, 4000, 8000, 16000, 32000, 64000, 130000, 260000, 520000,
            780000, 1040000, 1300000, 1560000, 1820000, 2080000, 2340000, 2600000,
            2860000, 3120000,
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

pub fn bard_spell_slots(caster_level: u32) -> Vec<u32> {
    let table = [
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0],
        vec![2, 1, 0, 0, 0, 0],
        vec![3, 1, 0, 0, 0, 0],
        vec![3, 2, 0, 0, 0, 0],
        vec![3, 2, 1, 0, 0, 0],
        vec![3, 3, 1, 0, 0, 0],
        vec![3, 3, 2, 0, 0, 0],
        vec![3, 3, 2, 1, 0, 0],
        vec![3, 3, 3, 1, 0, 0],
        vec![3, 3, 3, 2, 0, 0],
        vec![3, 3, 3, 2, 1, 0],
        vec![3, 3, 3, 3, 1, 0],
        vec![3, 3, 3, 3, 2, 0],
        vec![4, 3, 3, 3, 2, 1],
        vec![4, 4, 3, 3, 3, 1],
        vec![4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 3, 2],
        vec![4, 4, 4, 4, 4, 3],
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
        Class::Mage | Class::Abjurer | Class::Conjurer | Class::Diviner |
        Class::Enchanter | Class::Illusionist | Class::Invoker |
        Class::Necromancer | Class::Transmuter => (2, 4, 10),
        Class::Cleric | Class::Druid => (3, 6, 10),
        Class::Thief | Class::Bard => (2, 6, 10),
    }
}

pub fn thief_skill_bases() -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    (15, 10, 5, 10, 5, 15, 60, 0)
}

pub fn bard_skill_bases() -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    (10, 5, 0, 5, 0, 10, 50, 0)
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

pub fn can_multi_class(race: &Race) -> bool {
    !matches!(race, Race::Human)
}

pub fn can_dual_class(race: &Race) -> bool {
    matches!(race, Race::Human)
}

pub fn multi_class_combinations(race: &Race) -> Vec<Vec<&'static str>> {
    match race {
        Race::Dwarf => vec![
            vec!["Fighter", "Cleric"],
            vec!["Fighter", "Thief"],
        ],
        Race::Elf => vec![
            vec!["Fighter", "Mage"],
            vec!["Fighter", "Mage", "Thief"],
            vec!["Ranger", "Mage"],
            vec!["Mage", "Thief"],
        ],
        Race::Gnome => vec![
            vec!["Fighter", "Illusionist"],
            vec!["Fighter", "Thief"],
            vec!["Fighter", "Illusionist", "Thief"],
            vec!["Illusionist", "Thief"],
        ],
        Race::HalfElf => vec![
            vec!["Fighter", "Cleric"],
            vec!["Fighter", "Mage"],
            vec!["Fighter", "Druid"],
            vec!["Cleric", "Ranger"],
            vec!["Cleric", "Mage"],
            vec!["Cleric", "Druid", "Mage"],
            vec!["Druid", "Mage"],
            vec!["Fighter", "Cleric", "Mage"],
            vec!["Fighter", "Mage", "Thief"],
            vec!["Ranger", "Mage"],
            vec!["Cleric", "Mage", "Thief"],
            vec!["Cleric", "Thief"],
            vec!["Fighter", "Druid", "Mage"],
            vec!["Fighter", "Cleric", "Druid"],
            vec!["Fighter", "Cleric", "Mage", "Thief"],
            vec!["Fighter", "Druid", "Mage", "Thief"],
            vec!["Fighter", "Thief"],
            vec!["Mage", "Thief"],
        ],
        Race::Halfling => vec![
            vec!["Fighter", "Thief"],
        ],
        Race::HalfOrc => vec![
            vec!["Fighter", "Cleric"],
            vec!["Fighter", "Thief"],
            vec!["Cleric", "Thief"],
        ],
        Race::Human => vec![],
    }
}
