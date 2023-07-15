use leptos::{Signal, SignalGet};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharacterDetails {
    pub name: String,

    pub class: String,
    //pub level: i32,
    pub background: String,

    pub species: String,
    pub subspecies: String,
    pub xp: i32,

    // HP without con mod factored in.
    base_hp: i32,
    pub ability_scores: AbilityScores,
}

impl CharacterDetails {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            species: String::new(),
            class: String::new(),
            subspecies: String::new(),
            background: String::new(),
            //level: 1,
            xp: 0,
            base_hp: 0,
            ability_scores: AbilityScores::new(),
        }
    }
    pub fn prof_bonus(&self) -> i32 {
        ((self.level() - 1) / 4) + 2
    }
    pub fn set_base_hp(&mut self, hp: i32) {
        self.base_hp = hp;
    }
    pub fn base_hp(&self) -> i32 {
        self.base_hp
    }
    //pub fn max_hp(&self) -> i32 {
    //    self.base_hp + (self.ability_scores.con_mod() * self.level())
    //}
    pub fn change_species(&mut self, new: String) {
        self.species = new;
        self.subspecies = String::new();
    }
    pub fn set_level(&mut self, level: i32) {
        self.xp = level_to_xp(level)
    }
    pub fn level(&self) -> i32 {
        xp_to_level(self.xp)
    }
}

impl Default for CharacterDetails {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CharacterAsi {
    pub score: Ability,
    pub source_slug: String,
    pub amount: i32,
}
impl CharacterAsi {
    pub fn new(slug: String, ability: Ability, amount: i32) -> Self {
        Self {
            source_slug: slug,
            score: ability,
            amount,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AbilityScores {
    pub base_str: i32,
    pub base_dex: i32,
    pub base_con: i32,
    pub base_wis: i32,
    pub base_int: i32,
    pub base_cha: i32,
}
impl AbilityScores {
    pub fn score_to_mod(score: i32) -> i32 {
        (score - 10) / 2
    }
    pub fn new() -> Self {
        /*
        let test = CharacterAsi {
           score: AbilityScore::Charisma,
           source_slug: String::default(),
           amount: 4,
        };
        */
        Self {
            base_str: 10,
            base_dex: 10,
            base_con: 10,
            base_wis: 10,
            base_int: 10,
            base_cha: 10,
        }
    }
}
impl Default for AbilityScores {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct AbilityScoresReactive {
    pub ability_scores: Signal<AbilityScores>,
    pub asis: Signal<Vec<CharacterAsi>>,
}

impl AbilityScoresReactive {
    pub fn all_asis(&self) -> Vec<CharacterAsi> {
        self.asis.get()
    }
    fn asis_for_score(&self, score: Ability) -> Vec<CharacterAsi> {
        self.all_asis()
            .iter()
            .filter(|a| a.score == score)
            .cloned()
            .collect::<Vec<CharacterAsi>>()
    }
    pub fn str_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Strength)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_str + asi_boost
    }
    pub fn dex_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Dexterity)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_dex + asi_boost
    }
    pub fn con_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Constitution)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_con + asi_boost
    }
    pub fn wis_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Wisdom)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_wis + asi_boost
    }
    pub fn int_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Intelligence)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_int + asi_boost
    }
    pub fn cha_score(&self) -> i32 {
        let asi_boost: i32 = self
            .asis_for_score(Ability::Charisma)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.ability_scores.get().base_cha + asi_boost
    }
    pub fn str_mod(&self) -> i32 {
        Self::score_to_mod(self.str_score())
    }
    pub fn dex_mod(&self) -> i32 {
        Self::score_to_mod(self.dex_score())
    }
    pub fn con_mod(&self) -> i32 {
        Self::score_to_mod(self.con_score())
    }
    pub fn wis_mod(&self) -> i32 {
        Self::score_to_mod(self.wis_score())
    }
    pub fn int_mod(&self) -> i32 {
        Self::score_to_mod(self.int_score())
    }
    pub fn cha_mod(&self) -> i32 {
        Self::score_to_mod(self.cha_score())
    }
    pub fn score_to_mod(score: i32) -> i32 {
        (score - 10) / 2
    }
    pub fn get_ability_mod(&self, ability: &Ability) -> i32 {
        match ability {
            Ability::Strength => self.str_mod(),
            Ability::Dexterity => self.dex_mod(),
            Ability::Constitution => self.con_mod(),
            Ability::Wisdom => self.wis_mod(),
            Ability::Intelligence => self.int_mod(),
            Ability::Charisma => self.cha_mod(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}
impl Ability {
    pub fn to_string(&self) -> &str {
        match self {
            Ability::Strength => "Strength",
            Ability::Dexterity => "Dexterity",
            Ability::Constitution => "Constitution",
            Ability::Wisdom => "Wisdom",
            Ability::Intelligence => "Intelligence",
            Ability::Charisma => "Charisma",
        }
    }
    pub fn from_string(string: &str) -> Option<Ability> {
        match string.to_uppercase().as_str() {
            "STRENGTH" => Some(Ability::Strength),
            "DEXTERITY" => Some(Ability::Dexterity),
            "CONSTITUTION" => Some(Ability::Constitution),
            "WISDOM" => Some(Ability::Wisdom),
            "INTELLIGENCE" => Some(Ability::Intelligence),
            "CHARISMA" => Some(Ability::Charisma),
            _ => None,
        }
    }
}

fn xp_to_level(xp: i32) -> i32 {
    match xp {
        i32::MIN..=299 => 1,
        300..=899 => 2,
        900..=2699 => 3,
        2700..=6499 => 4,
        6500..=13999 => 5,
        14000..=22999 => 6,
        23000..=33999 => 7,
        34000..=47999 => 8,
        48000..=63999 => 9,
        64000..=84999 => 10,
        85000..=99999 => 11,
        100000..=119999 => 12,
        120000..=139999 => 13,
        140000..=164999 => 14,
        165000..=194999 => 15,
        195000..=224999 => 16,
        225000..=264999 => 17,
        265000..=304999 => 18,
        305000..=354999 => 19,
        355000..=i32::MAX => 20,
    }
}

fn level_to_xp(level: i32) -> i32 {
    match level {
        i32::MIN..=1 => 0,
        2 => 300,
        3 => 900,
        4 => 2700,
        5 => 6500,
        6 => 14000,
        7 => 23000,
        8 => 34000,
        9 => 48000,
        10 => 64000,
        11 => 85000,
        12 => 100000,
        13 => 120000,
        14 => 140000,
        15 => 165000,
        16 => 195000,
        17 => 225000,
        18 => 265000,
        19 => 305000,
        20..=i32::MAX => 355000,
    }
}
