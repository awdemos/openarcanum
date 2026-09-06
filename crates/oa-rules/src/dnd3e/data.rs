use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ability {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl Ability {
    pub fn name(&self) -> &'static str {
        match self {
            Ability::Str => "Strength",
            Ability::Dex => "Dexterity",
            Ability::Con => "Constitution",
            Ability::Int => "Intelligence",
            Ability::Wis => "Wisdom",
            Ability::Cha => "Charisma",
        }
    }

    pub fn abbreviation(&self) -> &'static str {
        match self {
            Ability::Str => "STR",
            Ability::Dex => "DEX",
            Ability::Con => "CON",
            Ability::Int => "INT",
            Ability::Wis => "WIS",
            Ability::Cha => "CHA",
        }
    }
}

pub fn ability_modifier(score: i32) -> i32 {
    (score - 10) / 2
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Race {
    Human,
    Elf(ElfSubrace),
    Dwarf(DwarfSubrace),
    Halfling(HalflingSubrace),
    Gnome(GnomeSubrace),
    HalfElf,
    HalfOrc,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElfSubrace {
    High,
    Wood,
    Drow,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DwarfSubrace {
    Hill,
    Mountain,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HalflingSubrace {
    Lightfoot,
    Tallfellow,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GnomeSubrace {
    Rock,
    Forest,
}

impl Race {
    pub fn name(&self) -> String {
        match self {
            Race::Human => "Human".to_string(),
            Race::Elf(sub) => format!("{} Elf", sub.name()),
            Race::Dwarf(sub) => format!("{} Dwarf", sub.name()),
            Race::Halfling(sub) => format!("{} Halfling", sub.name()),
            Race::Gnome(sub) => format!("{} Gnome", sub.name()),
            Race::HalfElf => "Half-Elf".to_string(),
            Race::HalfOrc => "Half-Orc".to_string(),
        }
    }

    pub fn ability_modifiers(&self) -> (i32, i32, i32, i32, i32, i32) {
        match self {
            Race::Human => (0, 0, 0, 0, 0, 0),
            Race::Elf(sub) => match sub {
                ElfSubrace::High => (0, 2, -2, 0, 0, 0),
                ElfSubrace::Wood => (0, 2, -2, 0, 0, 0),
                ElfSubrace::Drow => (0, 2, -2, 2, 0, 2),
            },
            Race::Dwarf(sub) => match sub {
                DwarfSubrace::Hill => (0, 0, 2, 0, 0, -2),
                DwarfSubrace::Mountain => (0, 0, 2, 0, 0, -2),
            },
            Race::Halfling(sub) => match sub {
                HalflingSubrace::Lightfoot => (-2, 2, 0, 0, 0, 0),
                HalflingSubrace::Tallfellow => (-2, 2, 0, 0, 0, 0),
            },
            Race::Gnome(sub) => match sub {
                GnomeSubrace::Rock => (-2, 0, 2, 0, 0, 0),
                GnomeSubrace::Forest => (-2, 0, 2, 0, 0, 0),
            },
            Race::HalfElf => (0, 0, 0, 0, 0, 0),
            Race::HalfOrc => (2, 0, 0, -2, 0, -2),
        }
    }

    pub fn size(&self) -> SizeCategory {
        match self {
            Race::Human | Race::Elf(_) | Race::Dwarf(_) | Race::HalfElf | Race::HalfOrc => SizeCategory::Medium,
            Race::Halfling(_) | Race::Gnome(_) => SizeCategory::Small,
        }
    }

    pub fn base_speed(&self) -> u32 {
        match self {
            Race::Human | Race::HalfElf | Race::HalfOrc => 30,
            Race::Elf(ElfSubrace::High) | Race::Elf(ElfSubrace::Drow) => 30,
            Race::Elf(ElfSubrace::Wood) => 30,
            Race::Dwarf(_) => 20,
            Race::Halfling(_) => 20,
            Race::Gnome(_) => 20,
        }
    }

    pub fn favored_class(&self) -> &'static str {
        match self {
            Race::Human => "Any",
            Race::Elf(_) => "Wizard",
            Race::Dwarf(_) => "Fighter",
            Race::Halfling(_) => "Rogue",
            Race::Gnome(_) => "Bard",
            Race::HalfElf => "Any",
            Race::HalfOrc => "Barbarian",
        }
    }

    pub fn special_traits(&self) -> Vec<&'static str> {
        match self {
            Race::Human => vec!["Bonus feat at 1st level", "Extra skill points (4 at 1st, 1 per level)"],
            Race::Elf(ElfSubrace::High) => vec!["Low-light vision", "Immune to magic sleep", "+2 save vs enchantment", "Proficient with longsword, rapier, longbow, shortbow"],
            Race::Elf(ElfSubrace::Wood) => vec!["Low-light vision", "Immune to magic sleep", "+2 save vs enchantment", "Proficient with longsword, rapier, longbow, shortbow"],
            Race::Elf(ElfSubrace::Drow) => vec!["Darkvision 120ft", "Daylight sensitivity", "Spell resistance 11+level", "+2 save vs enchantment", "Immune to magic sleep"],
            Race::Dwarf(DwarfSubrace::Hill) => vec!["Darkvision 60ft", "Stonecunning", "+2 save vs poison", "+2 save vs spells", "Stability", "Proficient with dwarven weapons"],
            Race::Dwarf(DwarfSubrace::Mountain) => vec!["Darkvision 60ft", "Stonecunning", "+2 save vs poison", "+2 save vs spells", "Stability", "Proficient with dwarven weapons"],
            Race::Halfling(HalflingSubrace::Lightfoot) => vec!["+1 to all saves", "+1 vs fear (stacks)", "+2 Hide"],
            Race::Halfling(HalflingSubrace::Tallfellow) => vec!["+1 to all saves", "+1 vs fear (stacks)", "+2 Search, Spot, Listen"],
            Race::Gnome(GnomeSubrace::Rock) => vec!["Low-light vision", "+2 save vs illusions", "+1 DC to illusion spells", "+1 attack vs kobolds/goblins", "+4 AC vs giants", "Speak with burrowing mammals"],
            Race::Gnome(GnomeSubrace::Forest) => vec!["Low-light vision", "+2 save vs illusions", "+1 DC to illusion spells", "+1 attack vs kobolds/goblins", "+4 AC vs giants", "Pass without trace in natural settings"],
            Race::HalfElf => vec!["Low-light vision", "Immune to magic sleep", "+2 save vs enchantment", "Elven blood"],
            Race::HalfOrc => vec!["Darkvision 60ft", "Orc blood"],
        }
    }
}

impl ElfSubrace {
    pub fn name(&self) -> &'static str {
        match self {
            ElfSubrace::High => "High",
            ElfSubrace::Wood => "Wood",
            ElfSubrace::Drow => "Drow",
        }
    }
}

impl DwarfSubrace {
    pub fn name(&self) -> &'static str {
        match self {
            DwarfSubrace::Hill => "Hill",
            DwarfSubrace::Mountain => "Mountain",
        }
    }
}

impl HalflingSubrace {
    pub fn name(&self) -> &'static str {
        match self {
            HalflingSubrace::Lightfoot => "Lightfoot",
            HalflingSubrace::Tallfellow => "Tallfellow",
        }
    }
}

impl GnomeSubrace {
    pub fn name(&self) -> &'static str {
        match self {
            GnomeSubrace::Rock => "Rock",
            GnomeSubrace::Forest => "Forest",
        }
    }
}

impl std::str::FromStr for Race {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "human" => Ok(Race::Human),
            "elf" | "high elf" => Ok(Race::Elf(ElfSubrace::High)),
            "wood elf" => Ok(Race::Elf(ElfSubrace::Wood)),
            "drow" | "dark elf" => Ok(Race::Elf(ElfSubrace::Drow)),
            "dwarf" | "hill dwarf" => Ok(Race::Dwarf(DwarfSubrace::Hill)),
            "mountain dwarf" => Ok(Race::Dwarf(DwarfSubrace::Mountain)),
            "halfling" | "lightfoot halfling" => Ok(Race::Halfling(HalflingSubrace::Lightfoot)),
            "tallfellow halfling" => Ok(Race::Halfling(HalflingSubrace::Tallfellow)),
            "gnome" | "rock gnome" => Ok(Race::Gnome(GnomeSubrace::Rock)),
            "forest gnome" => Ok(Race::Gnome(GnomeSubrace::Forest)),
            "half-elf" | "halfelf" => Ok(Race::HalfElf),
            "half-orc" | "halforc" => Ok(Race::HalfOrc),
            _ => Err(format!("Unknown race: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeCategory {
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

impl SizeCategory {
    pub fn name(&self) -> &'static str {
        match self {
            SizeCategory::Fine => "Fine",
            SizeCategory::Diminutive => "Diminutive",
            SizeCategory::Tiny => "Tiny",
            SizeCategory::Small => "Small",
            SizeCategory::Medium => "Medium",
            SizeCategory::Large => "Large",
            SizeCategory::Huge => "Huge",
            SizeCategory::Gargantuan => "Gargantuan",
            SizeCategory::Colossal => "Colossal",
        }
    }

    pub fn modifier(&self) -> i32 {
        match self {
            SizeCategory::Fine => 8,
            SizeCategory::Diminutive => 4,
            SizeCategory::Tiny => 2,
            SizeCategory::Small => 1,
            SizeCategory::Medium => 0,
            SizeCategory::Large => -1,
            SizeCategory::Huge => -2,
            SizeCategory::Gargantuan => -4,
            SizeCategory::Colossal => -8,
        }
    }

    pub fn grapple_modifier(&self) -> i32 {
        match self {
            SizeCategory::Fine => -16,
            SizeCategory::Diminutive => -12,
            SizeCategory::Tiny => -8,
            SizeCategory::Small => -4,
            SizeCategory::Medium => 0,
            SizeCategory::Large => 4,
            SizeCategory::Huge => 8,
            SizeCategory::Gargantuan => 12,
            SizeCategory::Colossal => 16,
        }
    }

    pub fn hide_modifier(&self) -> i32 {
        match self {
            SizeCategory::Fine => 16,
            SizeCategory::Diminutive => 12,
            SizeCategory::Tiny => 8,
            SizeCategory::Small => 4,
            SizeCategory::Medium => 0,
            SizeCategory::Large => -4,
            SizeCategory::Huge => -8,
            SizeCategory::Gargantuan => -12,
            SizeCategory::Colossal => -16,
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
    Wizard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrestigeClass {
    ArcaneArcher,
    Blackguard,
    Shadowdancer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BabProgression {
    Good,
    Average,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveProgression {
    Good,
    Poor,
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
            Class::Wizard => "Wizard",
        }
    }

    pub fn hit_die(&self) -> u32 {
        match self {
            Class::Barbarian => 12,
            Class::Fighter | Class::Paladin | Class::Ranger => 10,
            Class::Cleric | Class::Druid | Class::Monk | Class::Rogue | Class::Bard => 8,
            Class::Wizard | Class::Sorcerer => 6,
        }
    }

    pub fn bab_progression(&self) -> BabProgression {
        match self {
            Class::Barbarian | Class::Fighter | Class::Monk | Class::Paladin | Class::Ranger => BabProgression::Good,
            Class::Bard | Class::Cleric | Class::Druid | Class::Rogue => BabProgression::Average,
            Class::Sorcerer | Class::Wizard => BabProgression::Poor,
        }
    }

    pub fn fortitude_progression(&self) -> SaveProgression {
        match self {
            Class::Barbarian | Class::Fighter | Class::Monk | Class::Paladin | Class::Ranger | Class::Cleric | Class::Druid => SaveProgression::Good,
            Class::Bard | Class::Rogue | Class::Sorcerer | Class::Wizard => SaveProgression::Poor,
        }
    }

    pub fn reflex_progression(&self) -> SaveProgression {
        match self {
            Class::Monk | Class::Ranger | Class::Rogue | Class::Bard => SaveProgression::Good,
            Class::Barbarian | Class::Fighter | Class::Paladin | Class::Cleric | Class::Druid | Class::Sorcerer | Class::Wizard => SaveProgression::Poor,
        }
    }

    pub fn will_progression(&self) -> SaveProgression {
        match self {
            Class::Barbarian | Class::Cleric | Class::Druid | Class::Monk | Class::Paladin | Class::Sorcerer | Class::Wizard => SaveProgression::Good,
            Class::Fighter | Class::Ranger | Class::Rogue | Class::Bard => SaveProgression::Poor,
        }
    }

    pub fn skill_points_per_level(&self) -> i32 {
        match self {
            Class::Barbarian | Class::Fighter | Class::Paladin | Class::Cleric | Class::Sorcerer | Class::Wizard => 2,
            Class::Druid | Class::Monk => 4,
            Class::Bard | Class::Ranger => 6,
            Class::Rogue => 8,
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
            "wizard" => Some(Class::Wizard),
            _ => None,
        }
    }

    pub fn casts_spells(&self) -> bool {
        matches!(
            self,
            Class::Bard | Class::Cleric | Class::Druid | Class::Paladin |
            Class::Ranger | Class::Sorcerer | Class::Wizard
        )
    }

    pub fn spellcasting_type(&self) -> Option<SpellcastingType> {
        match self {
            Class::Wizard => Some(SpellcastingType::ArcanePrepared),
            Class::Sorcerer | Class::Bard => Some(SpellcastingType::ArcaneSpontaneous),
            Class::Cleric | Class::Druid | Class::Paladin | Class::Ranger => Some(SpellcastingType::Divine),
            _ => None,
        }
    }

    /// Key spellcasting ability
    pub fn spellcasting_ability(&self) -> Option<Ability> {
        match self {
            Class::Wizard => Some(Ability::Int),
            Class::Sorcerer | Class::Bard => Some(Ability::Cha),
            Class::Cleric | Class::Druid | Class::Paladin | Class::Ranger => Some(Ability::Wis),
            _ => None,
        }
    }

    pub fn required_alignment(&self) -> Option<Alignment> {
        match self {
            Class::Barbarian => Some(Alignment::NonLawful),
            Class::Monk => Some(Alignment::Lawful),
            Class::Paladin => Some(Alignment::LawfulGood),
            Class::Druid => Some(Alignment::NonLawful),
            Class::Bard => Some(Alignment::NonLawful),
            _ => None,
        }
    }

    pub fn bonus_feats(&self) -> bool {
        matches!(self, Class::Fighter)
    }

    pub fn class_skills(&self) -> Vec<&'static str> {
        match self {
            Class::Barbarian => vec!["Climb", "Craft", "Handle Animal", "Intimidate", "Jump", "Listen", "Ride", "Survival", "Swim"],
            Class::Bard => vec!["Appraise", "Balance", "Bluff", "Climb", "Concentration", "Craft", "Decipher Script", "Diplomacy", "Disguise", "Escape Artist", "Gather Information", "Hide", "Jump", "Knowledge (arcana)", "Knowledge (history)", "Knowledge (local)", "Knowledge (nobility)", "Listen", "Move Silently", "Perform", "Profession", "Sense Motive", "Sleight of Hand", "Speak Language", "Spellcraft", "Swim", "Tumble", "Use Magic Device"],
            Class::Cleric => vec!["Concentration", "Craft", "Diplomacy", "Heal", "Knowledge (arcana)", "Knowledge (history)", "Knowledge (religion)", "Knowledge (the planes)", "Profession", "Spellcraft"],
            Class::Druid => vec!["Concentration", "Craft", "Diplomacy", "Handle Animal", "Heal", "Knowledge (nature)", "Listen", "Profession", "Ride", "Spellcraft", "Spot", "Survival", "Swim"],
            Class::Fighter => vec!["Climb", "Craft", "Handle Animal", "Intimidate", "Jump", "Ride", "Swim"],
            Class::Monk => vec!["Balance", "Climb", "Concentration", "Craft", "Diplomacy", "Escape Artist", "Hide", "Jump", "Knowledge (arcana)", "Knowledge (religion)", "Listen", "Move Silently", "Perform", "Profession", "Sense Motive", "Spot", "Swim", "Tumble"],
            Class::Paladin => vec!["Concentration", "Craft", "Diplomacy", "Handle Animal", "Heal", "Knowledge (nobility)", "Knowledge (religion)", "Profession", "Ride", "Sense Motive"],
            Class::Ranger => vec!["Climb", "Concentration", "Craft", "Handle Animal", "Heal", "Hide", "Jump", "Knowledge (dungeoneering)", "Knowledge (geography)", "Knowledge (nature)", "Listen", "Move Silently", "Profession", "Ride", "Search", "Spot", "Survival", "Swim", "Use Rope"],
            Class::Rogue => vec!["Appraise", "Balance", "Bluff", "Climb", "Craft", "Decipher Script", "Diplomacy", "Disable Device", "Disguise", "Escape Artist", "Forgery", "Gather Information", "Hide", "Intimidate", "Jump", "Knowledge (local)", "Listen", "Move Silently", "Open Lock", "Perform", "Profession", "Search", "Sense Motive", "Sleight of Hand", "Spot", "Swim", "Tumble", "Use Magic Device", "Use Rope"],
            Class::Sorcerer => vec!["Bluff", "Concentration", "Craft", "Knowledge (arcana)", "Profession", "Spellcraft"],
            Class::Wizard => vec!["Concentration", "Craft", "Decipher Script", "Knowledge (arcana)", "Knowledge (architecture)", "Knowledge (dungeoneering)", "Knowledge (geography)", "Knowledge (history)", "Knowledge (local)", "Knowledge (nature)", "Knowledge (nobility)", "Knowledge (religion)", "Knowledge (the planes)", "Profession", "Spellcraft"],
        }
    }
}

impl PrestigeClass {
    pub fn name(&self) -> &'static str {
        match self {
            PrestigeClass::ArcaneArcher => "Arcane Archer",
            PrestigeClass::Blackguard => "Blackguard",
            PrestigeClass::Shadowdancer => "Shadowdancer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellcastingType {
    ArcanePrepared,
    ArcaneSpontaneous,
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
    Lawful,
    Good,
    Evil,
    Chaotic,
    NonLawful,
    NonGood,
    NonEvil,
    NonChaotic,
    LawfulGoodOnly,
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
            Alignment::Lawful => "Any Lawful",
            Alignment::Good => "Any Good",
            Alignment::Evil => "Any Evil",
            Alignment::Chaotic => "Any Chaotic",
            Alignment::NonLawful => "Any Non-Lawful",
            Alignment::NonGood => "Any Non-Good",
            Alignment::NonEvil => "Any Non-Evil",
            Alignment::NonChaotic => "Any Non-Chaotic",
            Alignment::LawfulGoodOnly => "Lawful Good",
        }
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
            "lawful" => Some(Alignment::Lawful),
            "good" => Some(Alignment::Good),
            "evil" => Some(Alignment::Evil),
            "chaotic" => Some(Alignment::Chaotic),
            _ => None,
        }
    }

    pub fn satisfies(&self, required: Alignment) -> bool {
        match required {
            Alignment::Lawful => matches!(self, Alignment::LawfulGood | Alignment::LawfulNeutral | Alignment::LawfulEvil),
            Alignment::Good => matches!(self, Alignment::LawfulGood | Alignment::NeutralGood | Alignment::ChaoticGood),
            Alignment::Evil => matches!(self, Alignment::LawfulEvil | Alignment::NeutralEvil | Alignment::ChaoticEvil),
            Alignment::Chaotic => matches!(self, Alignment::ChaoticGood | Alignment::ChaoticNeutral | Alignment::ChaoticEvil),
            Alignment::NonLawful => !matches!(self, Alignment::LawfulGood | Alignment::LawfulNeutral | Alignment::LawfulEvil),
            Alignment::NonGood => !matches!(self, Alignment::LawfulGood | Alignment::NeutralGood | Alignment::ChaoticGood),
            Alignment::NonEvil => !matches!(self, Alignment::LawfulEvil | Alignment::NeutralEvil | Alignment::ChaoticEvil),
            Alignment::NonChaotic => !matches!(self, Alignment::ChaoticGood | Alignment::ChaoticNeutral | Alignment::ChaoticEvil),
            Alignment::LawfulGoodOnly => matches!(self, Alignment::LawfulGood),
            _ => *self == required,
        }
    }
}

pub fn base_attack_bonus(progression: BabProgression, level: u32) -> i32 {
    match progression {
        BabProgression::Good => level as i32,
        BabProgression::Average => ((level as i32) * 3) / 4,
        BabProgression::Poor => (level as i32) / 2,
    }
}

pub fn save_bonus(progression: SaveProgression, level: u32) -> i32 {
    match progression {
        SaveProgression::Good => 2 + (level as i32) / 2,
        SaveProgression::Poor => (level as i32) / 3,
    }
}

pub fn num_attacks(bab: i32) -> u32 {
    if bab <= 0 {
        1
    } else {
        ((bab as u32 - 1) / 5) + 1
    }
}

pub fn attack_bonuses(bab: i32) -> Vec<i32> {
    let count = num_attacks(bab);
    (0..count).map(|i| bab - (i as i32 * 5)).collect()
}

pub fn first_level_hp(hit_die: u32, con_mod: i32) -> i32 {
    (hit_die as i32 + con_mod).max(1)
}

pub fn level_hp(_hit_die: u32, con_mod: i32) -> i32 {
    (1 + con_mod).max(1)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Skill {
    Appraise,
    Balance,
    Bluff,
    Climb,
    Concentration,
    Craft,
    DecipherScript,
    Diplomacy,
    DisableDevice,
    Disguise,
    EscapeArtist,
    Forgery,
    GatherInformation,
    HandleAnimal,
    Heal,
    Hide,
    Intimidate,
    Jump,
    KnowledgeArcana,
    KnowledgeArchitecture,
    KnowledgeDungeoneering,
    KnowledgeGeography,
    KnowledgeHistory,
    KnowledgeLocal,
    KnowledgeNature,
    KnowledgeNobility,
    KnowledgeReligion,
    KnowledgePlanes,
    Listen,
    MoveSilently,
    OpenLock,
    Perform,
    Profession,
    Ride,
    Search,
    SenseMotive,
    SleightOfHand,
    SpeakLanguage,
    Spellcraft,
    Spot,
    Survival,
    Swim,
    Tumble,
    UseMagicDevice,
    UseRope,
}

impl Skill {
    pub fn name(&self) -> &'static str {
        match self {
            Skill::Appraise => "Appraise",
            Skill::Balance => "Balance",
            Skill::Bluff => "Bluff",
            Skill::Climb => "Climb",
            Skill::Concentration => "Concentration",
            Skill::Craft => "Craft",
            Skill::DecipherScript => "Decipher Script",
            Skill::Diplomacy => "Diplomacy",
            Skill::DisableDevice => "Disable Device",
            Skill::Disguise => "Disguise",
            Skill::EscapeArtist => "Escape Artist",
            Skill::Forgery => "Forgery",
            Skill::GatherInformation => "Gather Information",
            Skill::HandleAnimal => "Handle Animal",
            Skill::Heal => "Heal",
            Skill::Hide => "Hide",
            Skill::Intimidate => "Intimidate",
            Skill::Jump => "Jump",
            Skill::KnowledgeArcana => "Knowledge (arcana)",
            Skill::KnowledgeArchitecture => "Knowledge (architecture)",
            Skill::KnowledgeDungeoneering => "Knowledge (dungeoneering)",
            Skill::KnowledgeGeography => "Knowledge (geography)",
            Skill::KnowledgeHistory => "Knowledge (history)",
            Skill::KnowledgeLocal => "Knowledge (local)",
            Skill::KnowledgeNature => "Knowledge (nature)",
            Skill::KnowledgeNobility => "Knowledge (nobility)",
            Skill::KnowledgeReligion => "Knowledge (religion)",
            Skill::KnowledgePlanes => "Knowledge (the planes)",
            Skill::Listen => "Listen",
            Skill::MoveSilently => "Move Silently",
            Skill::OpenLock => "Open Lock",
            Skill::Perform => "Perform",
            Skill::Profession => "Profession",
            Skill::Ride => "Ride",
            Skill::Search => "Search",
            Skill::SenseMotive => "Sense Motive",
            Skill::SleightOfHand => "Sleight of Hand",
            Skill::SpeakLanguage => "Speak Language",
            Skill::Spellcraft => "Spellcraft",
            Skill::Spot => "Spot",
            Skill::Survival => "Survival",
            Skill::Swim => "Swim",
            Skill::Tumble => "Tumble",
            Skill::UseMagicDevice => "Use Magic Device",
            Skill::UseRope => "Use Rope",
        }
    }

    pub fn key_ability(&self) -> Ability {
        match self {
            Skill::Appraise => Ability::Int,
            Skill::Balance => Ability::Dex,
            Skill::Bluff => Ability::Cha,
            Skill::Climb => Ability::Str,
            Skill::Concentration => Ability::Con,
            Skill::Craft => Ability::Int,
            Skill::DecipherScript => Ability::Int,
            Skill::Diplomacy => Ability::Cha,
            Skill::DisableDevice => Ability::Int,
            Skill::Disguise => Ability::Cha,
            Skill::EscapeArtist => Ability::Dex,
            Skill::Forgery => Ability::Int,
            Skill::GatherInformation => Ability::Cha,
            Skill::HandleAnimal => Ability::Cha,
            Skill::Heal => Ability::Wis,
            Skill::Hide => Ability::Dex,
            Skill::Intimidate => Ability::Cha,
            Skill::Jump => Ability::Str,
            Skill::KnowledgeArcana |
            Skill::KnowledgeArchitecture |
            Skill::KnowledgeDungeoneering |
            Skill::KnowledgeGeography |
            Skill::KnowledgeHistory |
            Skill::KnowledgeLocal |
            Skill::KnowledgeNature |
            Skill::KnowledgeNobility |
            Skill::KnowledgeReligion |
            Skill::KnowledgePlanes => Ability::Int,
            Skill::Listen => Ability::Wis,
            Skill::MoveSilently => Ability::Dex,
            Skill::OpenLock => Ability::Dex,
            Skill::Perform => Ability::Cha,
            Skill::Profession => Ability::Wis,
            Skill::Ride => Ability::Dex,
            Skill::Search => Ability::Int,
            Skill::SenseMotive => Ability::Wis,
            Skill::SleightOfHand => Ability::Dex,
            Skill::SpeakLanguage => Ability::Int,
            Skill::Spellcraft => Ability::Int,
            Skill::Spot => Ability::Wis,
            Skill::Survival => Ability::Wis,
            Skill::Swim => Ability::Str,
            Skill::Tumble => Ability::Dex,
            Skill::UseMagicDevice => Ability::Cha,
            Skill::UseRope => Ability::Dex,
        }
    }

    pub fn is_trained_only(&self) -> bool {
        matches!(self,
            Skill::DecipherScript | Skill::DisableDevice | Skill::HandleAnimal |
            Skill::OpenLock | Skill::Profession | Skill::SleightOfHand |
            Skill::SpeakLanguage | Skill::Spellcraft | Skill::Tumble |
            Skill::UseMagicDevice
        )
    }

    pub fn armor_check_penalty(&self) -> bool {
        matches!(self,
            Skill::Balance | Skill::Climb | Skill::EscapeArtist | Skill::Hide |
            Skill::Jump | Skill::MoveSilently | Skill::Ride | Skill::SleightOfHand |
            Skill::Swim | Skill::Tumble
        )
    }
}

/// Maximum rank for a class skill = level + 3
pub fn max_class_skill_rank(level: u32) -> i32 {
    (level as i32) + 3
}

/// Maximum rank for a cross-class skill = (level + 3) / 2
pub fn max_cross_class_skill_rank(level: u32) -> i32 {
    ((level as i32) + 3) / 2
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Feat {
    CombatReflexes,
    Dodge,
    Mobility,
    SpringAttack,
    PowerAttack,
    Cleave,
    GreatCleave,
    WeaponFocus,
    WeaponSpecialization,
    Toughness,
    IronWill,
    LightningReflexes,
    GreatFortitude,
    ImprovedInitiative,
    PointBlankShot,
    PreciseShot,
    RapidShot,
    Manyshot,
    ShotOnTheRun,
    TwoWeaponFighting,
    ImprovedTwoWeaponFighting,
    ImprovedCritical,
    StunningFist,
    DeflectArrows,
    SnatchArrows,
    ImprovedGrapple,
    ImprovedTrip,
    ImprovedDisarm,
    ImprovedSunder,
    CombatExpertise,
    WhirlwindAttack,
    ImprovedFeint,
    MountedCombat,
    RideByAttack,
    SpiritedCharge,
    Trample,
    Leadership,
    SpellFocus,
    GreaterSpellFocus,
    SpellPenetration,
    GreaterSpellPenetration,
    BrewPotion,
    CraftWand,
    CraftWondrousItem,
    ScribeScroll,
    EmpowerSpell,
    EnlargeSpell,
    ExtendSpell,
    HeightenSpell,
    MaximizeSpell,
    QuickenSpell,
    SilentSpell,
    StillSpell,
    WidenSpell,
    Alertness,
    Athletic,
    Negotiator,
    Persuasive,
    SelfSufficient,
    Stealthy,
    Track,
}

impl Feat {
    pub fn name(&self) -> &'static str {
        match self {
            Feat::CombatReflexes => "Combat Reflexes",
            Feat::Dodge => "Dodge",
            Feat::Mobility => "Mobility",
            Feat::SpringAttack => "Spring Attack",
            Feat::PowerAttack => "Power Attack",
            Feat::Cleave => "Cleave",
            Feat::GreatCleave => "Great Cleave",
            Feat::WeaponFocus => "Weapon Focus",
            Feat::WeaponSpecialization => "Weapon Specialization",
            Feat::Toughness => "Toughness",
            Feat::IronWill => "Iron Will",
            Feat::LightningReflexes => "Lightning Reflexes",
            Feat::GreatFortitude => "Great Fortitude",
            Feat::ImprovedInitiative => "Improved Initiative",
            Feat::PointBlankShot => "Point Blank Shot",
            Feat::PreciseShot => "Precise Shot",
            Feat::RapidShot => "Rapid Shot",
            Feat::Manyshot => "Manyshot",
            Feat::ShotOnTheRun => "Shot on the Run",
            Feat::TwoWeaponFighting => "Two-Weapon Fighting",
            Feat::ImprovedTwoWeaponFighting => "Improved Two-Weapon Fighting",
            Feat::ImprovedCritical => "Improved Critical",
            Feat::StunningFist => "Stunning Fist",
            Feat::DeflectArrows => "Deflect Arrows",
            Feat::SnatchArrows => "Snatch Arrows",
            Feat::ImprovedGrapple => "Improved Grapple",
            Feat::ImprovedTrip => "Improved Trip",
            Feat::ImprovedDisarm => "Improved Disarm",
            Feat::ImprovedSunder => "Improved Sunder",
            Feat::CombatExpertise => "Combat Expertise",
            Feat::WhirlwindAttack => "Whirlwind Attack",
            Feat::ImprovedFeint => "Improved Feint",
            Feat::MountedCombat => "Mounted Combat",
            Feat::RideByAttack => "Ride-By Attack",
            Feat::SpiritedCharge => "Spirited Charge",
            Feat::Trample => "Trample",
            Feat::Leadership => "Leadership",
            Feat::SpellFocus => "Spell Focus",
            Feat::GreaterSpellFocus => "Greater Spell Focus",
            Feat::SpellPenetration => "Spell Penetration",
            Feat::GreaterSpellPenetration => "Greater Spell Penetration",
            Feat::BrewPotion => "Brew Potion",
            Feat::CraftWand => "Craft Wand",
            Feat::CraftWondrousItem => "Craft Wondrous Item",
            Feat::ScribeScroll => "Scribe Scroll",
            Feat::EmpowerSpell => "Empower Spell",
            Feat::EnlargeSpell => "Enlarge Spell",
            Feat::ExtendSpell => "Extend Spell",
            Feat::HeightenSpell => "Heighten Spell",
            Feat::MaximizeSpell => "Maximize Spell",
            Feat::QuickenSpell => "Quicken Spell",
            Feat::SilentSpell => "Silent Spell",
            Feat::StillSpell => "Still Spell",
            Feat::WidenSpell => "Widen Spell",
            Feat::Alertness => "Alertness",
            Feat::Athletic => "Athletic",
            Feat::Negotiator => "Negotiator",
            Feat::Persuasive => "Persuasive",
            Feat::SelfSufficient => "Self-Sufficient",
            Feat::Stealthy => "Stealthy",
            Feat::Track => "Track",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "combat reflexes" => Some(Feat::CombatReflexes),
            "dodge" => Some(Feat::Dodge),
            "mobility" => Some(Feat::Mobility),
            "spring attack" => Some(Feat::SpringAttack),
            "power attack" => Some(Feat::PowerAttack),
            "cleave" => Some(Feat::Cleave),
            "great cleave" => Some(Feat::GreatCleave),
            "weapon focus" => Some(Feat::WeaponFocus),
            "weapon specialization" => Some(Feat::WeaponSpecialization),
            "toughness" => Some(Feat::Toughness),
            "iron will" => Some(Feat::IronWill),
            "lightning reflexes" => Some(Feat::LightningReflexes),
            "great fortitude" => Some(Feat::GreatFortitude),
            "improved initiative" => Some(Feat::ImprovedInitiative),
            "point blank shot" => Some(Feat::PointBlankShot),
            "precise shot" => Some(Feat::PreciseShot),
            "rapid shot" => Some(Feat::RapidShot),
            "manyshot" => Some(Feat::Manyshot),
            "shot on the run" => Some(Feat::ShotOnTheRun),
            "two-weapon fighting" => Some(Feat::TwoWeaponFighting),
            "improved two-weapon fighting" => Some(Feat::ImprovedTwoWeaponFighting),
            "improved critical" => Some(Feat::ImprovedCritical),
            "stunning fist" => Some(Feat::StunningFist),
            "deflect arrows" => Some(Feat::DeflectArrows),
            "snatch arrows" => Some(Feat::SnatchArrows),
            "improved grapple" => Some(Feat::ImprovedGrapple),
            "improved trip" => Some(Feat::ImprovedTrip),
            "improved disarm" => Some(Feat::ImprovedDisarm),
            "improved sunder" => Some(Feat::ImprovedSunder),
            "combat expertise" => Some(Feat::CombatExpertise),
            "whirlwind attack" => Some(Feat::WhirlwindAttack),
            "improved feint" => Some(Feat::ImprovedFeint),
            "mounted combat" => Some(Feat::MountedCombat),
            "ride-by attack" => Some(Feat::RideByAttack),
            "spirited charge" => Some(Feat::SpiritedCharge),
            "trample" => Some(Feat::Trample),
            "leadership" => Some(Feat::Leadership),
            "spell focus" => Some(Feat::SpellFocus),
            "greater spell focus" => Some(Feat::GreaterSpellFocus),
            "spell penetration" => Some(Feat::SpellPenetration),
            "greater spell penetration" => Some(Feat::GreaterSpellPenetration),
            "brew potion" => Some(Feat::BrewPotion),
            "craft wand" => Some(Feat::CraftWand),
            "craft wondrous item" => Some(Feat::CraftWondrousItem),
            "scribe scroll" => Some(Feat::ScribeScroll),
            "empower spell" => Some(Feat::EmpowerSpell),
            "enlarge spell" => Some(Feat::EnlargeSpell),
            "extend spell" => Some(Feat::ExtendSpell),
            "heighten spell" => Some(Feat::HeightenSpell),
            "maximize spell" => Some(Feat::MaximizeSpell),
            "quicken spell" => Some(Feat::QuickenSpell),
            "silent spell" => Some(Feat::SilentSpell),
            "still spell" => Some(Feat::StillSpell),
            "widen spell" => Some(Feat::WidenSpell),
            "alertness" => Some(Feat::Alertness),
            "athletic" => Some(Feat::Athletic),
            "negotiator" => Some(Feat::Negotiator),
            "persuasive" => Some(Feat::Persuasive),
            "self-sufficient" => Some(Feat::SelfSufficient),
            "stealthy" => Some(Feat::Stealthy),
            "track" => Some(Feat::Track),
            _ => None,
        }
    }
}

pub fn normal_feat_count(level: u32) -> u32 {
    1 + ((level.saturating_sub(1)) / 3)
}

pub fn fighter_bonus_feat_count(level: u32) -> u32 {
    level.div_ceil(2)
}

pub fn full_caster_spell_slots(caster_level: u32) -> Vec<u32> {
    let table: Vec<Vec<u32>> = vec![
        vec![3, 1],
        vec![4, 2],
        vec![4, 2, 1],
        vec![4, 3, 2],
        vec![4, 3, 2, 1],
        vec![4, 3, 3, 2],
        vec![4, 4, 3, 2, 1],
        vec![4, 4, 3, 3, 2],
        vec![4, 4, 4, 3, 2, 1],
        vec![4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 3, 2, 1],
        vec![4, 4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 4, 3, 2, 1],
        vec![4, 4, 4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 4, 4, 3, 2, 1],
        vec![4, 4, 4, 4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 4, 4, 4, 3, 2, 1],
        vec![4, 4, 4, 4, 4, 4, 4, 3, 3, 2],
        vec![4, 4, 4, 4, 4, 4, 4, 4, 3, 3],
        vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn bard_spell_slots(caster_level: u32) -> Vec<u32> {
    let table: Vec<Vec<u32>> = vec![
        vec![],
        vec![2],
        vec![3, 0],
        vec![3, 1],
        vec![3, 2, 0],
        vec![3, 2, 1],
        vec![3, 3, 2, 0],
        vec![3, 3, 2, 1],
        vec![3, 3, 3, 2, 0],
        vec![3, 3, 3, 2, 1],
        vec![3, 3, 3, 3, 2, 0],
        vec![3, 3, 3, 3, 2, 1],
        vec![3, 3, 3, 3, 3, 2, 0],
        vec![3, 3, 3, 3, 3, 2, 1],
        vec![3, 3, 3, 3, 3, 3, 2, 0],
        vec![3, 3, 3, 3, 3, 3, 2, 1],
        vec![3, 3, 3, 3, 3, 3, 3, 2, 0],
        vec![3, 3, 3, 3, 3, 3, 3, 2, 1],
        vec![3, 3, 3, 3, 3, 3, 3, 3, 2, 0],
        vec![3, 3, 3, 3, 3, 3, 3, 3, 2, 1],
    ];
    let idx = (caster_level as usize).saturating_sub(1).min(19);
    table[idx].clone()
}

pub fn half_caster_spell_slots(caster_level: u32) -> Vec<u32> {
    let effective_level = caster_level.saturating_sub(3);
    if effective_level == 0 {
        return vec![];
    }
    let table: Vec<Vec<u32>> = vec![
        vec![0],
        vec![1],
        vec![1, 0],
        vec![1, 1],
        vec![1, 1, 0],
        vec![1, 1, 1],
        vec![2, 1, 1, 0],
        vec![2, 1, 1, 1],
        vec![2, 2, 1, 1, 0],
        vec![2, 2, 2, 1, 1],
    ];
    let idx = (effective_level as usize).saturating_sub(1).min(9);
    table[idx].clone()
}

pub const BASE_AC: i32 = 10;

pub fn calculate_ac(
    dex_mod: i32,
    armor_bonus: i32,
    shield_bonus: i32,
    size_mod: i32,
    natural_armor: i32,
    deflection: i32,
    misc: i32,
) -> i32 {
    BASE_AC + dex_mod + armor_bonus + shield_bonus + size_mod + natural_armor + deflection + misc
}

pub fn armor_types() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("Padded", 1);
    map.insert("Leather", 2);
    map.insert("Studded leather", 3);
    map.insert("Chain shirt", 4);
    map.insert("Hide", 3);
    map.insert("Scale mail", 4);
    map.insert("Chainmail", 5);
    map.insert("Breastplate", 5);
    map.insert("Splint mail", 6);
    map.insert("Banded mail", 6);
    map.insert("Half-plate", 7);
    map.insert("Full plate", 8);
    map
}

pub fn shield_types() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("Buckler", 1);
    map.insert("Shield, light wooden", 1);
    map.insert("Shield, light steel", 1);
    map.insert("Shield, heavy wooden", 2);
    map.insert("Shield, heavy steel", 2);
    map.insert("Tower shield", 4);
    map
}

pub fn xp_for_level(level: u32) -> u32 {
    match level {
        1 => 0,
        2 => 1000,
        3 => 3000,
        4 => 6000,
        5 => 10000,
        6 => 15000,
        7 => 21000,
        8 => 28000,
        9 => 36000,
        10 => 45000,
        11 => 55000,
        12 => 66000,
        13 => 78000,
        14 => 91000,
        15 => 105000,
        16 => 120000,
        17 => 136000,
        18 => 153000,
        19 => 171000,
        20 => 190000,
        _ => 190000 + ((level - 20) * 20000),
    }
}

pub fn starting_gold(class: &Class) -> (u32, u32, u32) {
    match class {
        Class::Barbarian => (3, 4, 10),
        Class::Bard => (3, 4, 10),
        Class::Cleric => (5, 4, 10),
        Class::Druid => (2, 4, 10),
        Class::Fighter => (6, 4, 10),
        Class::Monk => (5, 4, 1),
        Class::Paladin => (6, 4, 10),
        Class::Ranger => (6, 4, 10),
        Class::Rogue => (5, 4, 10),
        Class::Sorcerer => (2, 4, 10),
        Class::Wizard => (2, 4, 10),
    }
}

pub fn all_skills() -> Vec<Skill> {
    vec![
        Skill::Appraise,
        Skill::Balance,
        Skill::Bluff,
        Skill::Climb,
        Skill::Concentration,
        Skill::Craft,
        Skill::DecipherScript,
        Skill::Diplomacy,
        Skill::DisableDevice,
        Skill::Disguise,
        Skill::EscapeArtist,
        Skill::Forgery,
        Skill::GatherInformation,
        Skill::HandleAnimal,
        Skill::Heal,
        Skill::Hide,
        Skill::Intimidate,
        Skill::Jump,
        Skill::KnowledgeArcana,
        Skill::KnowledgeArchitecture,
        Skill::KnowledgeDungeoneering,
        Skill::KnowledgeGeography,
        Skill::KnowledgeHistory,
        Skill::KnowledgeLocal,
        Skill::KnowledgeNature,
        Skill::KnowledgeNobility,
        Skill::KnowledgeReligion,
        Skill::KnowledgePlanes,
        Skill::Listen,
        Skill::MoveSilently,
        Skill::OpenLock,
        Skill::Perform,
        Skill::Profession,
        Skill::Ride,
        Skill::Search,
        Skill::SenseMotive,
        Skill::SleightOfHand,
        Skill::SpeakLanguage,
        Skill::Spellcraft,
        Skill::Spot,
        Skill::Survival,
        Skill::Swim,
        Skill::Tumble,
        Skill::UseMagicDevice,
        Skill::UseRope,
    ]
}
