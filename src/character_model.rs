#![allow(dead_code)]

pub struct CharacterDetails {
    pub name: String,

    pub class: String,
    pub level: i64,
    pub background: String,

    pub species: String,
    pub subspecies: String,
    pub xp: i64,

    // HP without con mod factored in.
    base_hp: i64,
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
            level: 1,
            xp: 0,
            base_hp: 0,
            ability_scores: AbilityScores::new(),
        }
    }
    pub fn prof_bonus(&self) -> i64 {
        ((self.level - 1) / 4) + 2
    }
    pub fn set_base_hp(&mut self, hp: i64) {
        self.base_hp = hp;
    }
    pub fn max_hp(&self) -> i64 {
        self.base_hp + (self.ability_scores.con_mod() * self.level)
    }
}

#[derive(Clone)]
pub struct CharacterAsi {
    pub score: AbilityScore,
    pub source_slug: String,
    pub amount: i64,
}
impl CharacterAsi {
    pub fn new(slug: String, ability: AbilityScore, amount: i64) -> Self {
        Self {
            source_slug: slug,
            score: ability,
            amount,
        }
    }
}

pub struct AbilityScores {
    pub base_str: i64,
    pub base_dex: i64,
    pub base_con: i64,
    pub base_wis: i64,
    pub base_int: i64,
    pub base_cha: i64,
    pub level_1_asis: Vec<CharacterAsi>,
}

impl AbilityScores {
    pub fn all_asis(&self) -> Vec<CharacterAsi> {
        self.level_1_asis.clone()
    }
    fn asis_for_score(&self, score: AbilityScore) -> Vec<CharacterAsi> {
        self.all_asis()
            .iter()
            .filter(|a| a.score == score)
            .cloned()
            .collect::<Vec<CharacterAsi>>()
    }
    pub fn str_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Strength)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_str + asi_boost
    }
    pub fn dex_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Dexterity)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_dex + asi_boost
    }
    pub fn con_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Constitution)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_con + asi_boost
    }
    pub fn wis_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Wisdom)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_wis + asi_boost
    }
    pub fn int_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Intelligence)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_int + asi_boost
    }
    pub fn cha_score(&self) -> i64 {
        let asi_boost: i64 = self
            .asis_for_score(AbilityScore::Charisma)
            .iter()
            .map(|a| a.amount)
            .sum();
        self.base_cha + asi_boost
    }
    pub fn str_mod(&self) -> i64 {
        (self.str_score() - 10) / 2
    }
    pub fn dex_mod(&self) -> i64 {
        (self.dex_score() - 10) / 2
    }
    pub fn con_mod(&self) -> i64 {
        (self.con_score() - 10) / 2
    }
    pub fn wis_mod(&self) -> i64 {
        (self.wis_score() - 10) / 2
    }
    pub fn int_mod(&self) -> i64 {
        (self.int_score() - 10) / 2
    }
    pub fn cha_mod(&self) -> i64 {
        (self.cha_score() - 10) / 2
    }
    pub fn new() -> Self {
        Self {
            base_str: 10,
            base_dex: 10,
            base_con: 10,
            base_wis: 10,
            base_int: 10,
            base_cha: 10,
            level_1_asis: vec![],
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}
impl AbilityScore {
    pub fn to_string(&self) -> &str {
        match self {
            AbilityScore::Strength => "Strength",
            AbilityScore::Dexterity => "Dexterity",
            AbilityScore::Constitution => "Constitution",
            AbilityScore::Wisdom => "Wisdom",
            AbilityScore::Intelligence => "Intelligence",
            AbilityScore::Charisma => "Charisma",
        }
    }
    pub fn from_string(string: &String) -> Option<AbilityScore> {
        match string.to_uppercase().as_str() {
            "STRENGTH" => Some(AbilityScore::Strength),
            "DEXTERITY" => Some(AbilityScore::Dexterity),
            "CONSTITUTION" => Some(AbilityScore::Constitution),
            "WISDOM" => Some(AbilityScore::Wisdom),
            "INTELLIGENCE" => Some(AbilityScore::Intelligence),
            "CHARISMA" => Some(AbilityScore::Charisma),
            _ => None,
        }
    }
}

enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}
