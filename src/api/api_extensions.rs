use super::api_model::*;
use crate::character_model::{Ability, AttackAction, AttackType, CharacterAsi};
use lazy_regex::{regex, regex_captures};
use leptos::leptos_dom::log;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Feature {
    pub name: String,
    pub desc: String,
    pub level: i32,
    pub feature_type: FeatureType,
    pub source_slug: String,
    pub hidden: bool,
}
impl Feature {
    pub fn feature_slug(&self) -> String {
        format!(
            "{}:{}",
            self.source_slug,
            self.name.to_lowercase().replace(' ', "_")
        )
    }
    pub fn new_skill(skill: &str, source_slug: &str) -> Self {
        Self {
            name: format!("Skill: {}", skill),
            desc: String::new(),
            level: 1,
            feature_type: FeatureType::SkillProficency(skill.to_string()),
            source_slug: source_slug.to_string(),
            hidden: true,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum FeatureType {
    Asi(CharacterAsi),
    SavingThrow(Ability),
    SkillProficency(String),
    OtherProficency(String),
    Option(FeatureOptions),
    Fluff,
    #[default]
    None,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct FeatureOptions {
    pub num_choices: i32,
    pub options: Vec<Feature>,
}
/// A selection of an item from within a FeatureOptions.
/// Since FeatureOptions can allow for multiple choices,
/// you can have multiple of these per FeatureOptions.
#[derive(Serialize, Deserialize, PartialEq)]
pub struct FeatureOptionsSelection {
    /// Each feature option belongs to a Feature, which has a feature slug
    /// This string represents that slug.
    pub slug: String,
    /// An index into the options array of the FeatureOption, representing
    /// the currently selected item
    pub selection: usize,
}

impl Species {
    pub fn features(&self) -> Vec<Feature> {
        let mut features: Vec<Feature> = vec![];

        // Add ASI as a feature
        for asi in &self.asi {
            for char_asi in asi.get_asis(&self.slug) {
                features.push(Feature {
                    level: 1,
                    name: "Ability Score Increase".to_string(),
                    desc: self.asi_desc.to_string(),
                    feature_type: FeatureType::Asi(char_asi),
                    source_slug: format!("species:{}", self.slug),
                    hidden: true,
                });
            }
        }

        let desc_parts = self.traits.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature {
            source_slug: format!("species:{}", self.slug),
            ..Default::default()
        };
        for line in desc_parts {
            // Open5e's data is somewhat inconsistently formatted currently.
            // Here we clean up the data a bit by standardizing all bold-itallics
            // formatting.
            let line = line.replace("**_", "***").replace("_**", "***");
            // The feature name will be in the first phrase in bold-italics.
            let captures = regex_captures!(r"\*\*\*(.+)\*\*\*(.+)", &line);
            if let Some((_, feature_name, rest)) = captures {
                if !current_feature.name.is_empty() {
                    features.push(current_feature);
                    current_feature = Feature {
                        source_slug: format!("species:{}", self.slug),
                        ..Default::default()
                    };
                }
                if !feature_name.is_empty() {
                    current_feature.name = feature_name.to_string();
                }
                if !rest.is_empty() {
                    current_feature.desc += rest;
                }
            } else {
                current_feature.desc += &line;
            }
        }
        if !current_feature.name.is_empty() {
            features.push(current_feature);
        }
        features
    }
}

impl Subspecies {
    pub fn features(&self) -> Vec<Feature> {
        let mut features: Vec<Feature> = vec![];

        let desc_parts = self.traits.split("\n\n").collect::<Vec<&str>>();

        // Add ASI as a feature
        for api_asi in &self.asi {
            for char_asi in api_asi.get_asis(&self.slug) {
                features.push(Feature {
                    level: 1,
                    name: "Ability Score Increase".to_string(),
                    desc: self.asi_desc.to_string(),
                    feature_type: FeatureType::Asi(char_asi),
                    source_slug: format!("subspecies:{}", self.slug),
                    hidden: true,
                });
            }
        }

        let mut current_feature = Feature {
            source_slug: format!("subspecies:{}", self.slug),
            ..Default::default()
        };
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let captures = regex_captures!(r"\*\*\*(.+)\*\*\*(.+)", &line);
            if let Some((_, feature_name, rest)) = captures {
                if !current_feature.name.is_empty() {
                    features.push(current_feature);
                    current_feature = Feature::default();
                    current_feature.source_slug =
                        format!("subspecies:{}", self.slug);
                }
                if !feature_name.is_empty() {
                    current_feature.name = feature_name.to_string();
                }
                if !rest.is_empty() {
                    current_feature.desc += rest;
                }
            } else {
                current_feature.desc += &line;
            }
        }
        if !current_feature.name.is_empty() {
            features.push(current_feature);
        }
        features
    }
}

impl Class {
    pub fn base_hp(&self) -> i32 {
        let mut split = self.hp_at_1st_level.split(' ');
        if let Some(word) = split.next() {
            if let Ok(num) = str::parse::<i32>(word) {
                num
            } else {
                0
            }
        } else {
            0
        }
    }
    pub fn features(&self) -> Vec<Feature> {
        let source_slug = format!("class:{}", self.slug);
        let mut features: Vec<Feature> = vec![];

        // Add class skills as a feature
        let level_pattern =
            regex!(r"Choose (two|three|four|two skills) from (.+)");
        let skill_choices = &self.prof_skills;
        let source_slug_2 = source_slug.clone();
        if skill_choices == "Choose any three" {
            // Bard!
            let mut feature = Feature {
                name: "Class Skills".to_string(),
                desc: skill_choices.to_string(),
                level: 1,
                feature_type: FeatureType::None,
                source_slug: source_slug_2,
                hidden: false,
            };
            feature.feature_type = FeatureType::Option(FeatureOptions {
                num_choices: 3,
                options: SKILL_LIST
                    .iter()
                    .map(|s| Feature::new_skill(s, &feature.feature_slug()))
                    .collect::<Vec<Feature>>(),
            });
            features.push(feature);
        } else if let Some(captures) = level_pattern.captures(skill_choices) {
            // Handle first match (number of skills to pick)
            let mut count = 0;
            if let Some(group) = captures.get(1) {
                count = match group.as_str() {
                    "two" => 2,
                    "two skills" => 2,
                    "three" => 3,
                    "four" => 4,
                    _ => 0,
                };
            }
            // Parse rest of string as list of skills
            let mut skills = vec![];
            if let Some(group) = captures.get(2) {
                let group_string = group.as_str();
                let new_string = group_string.replace(" and ", " ");
                for substring in new_string.split(',') {
                    skills.push(substring.trim().to_string());
                }
            }
            let mut feature = Feature {
                name: "Class Skills".to_string(),
                desc: skill_choices.to_string(),
                level: 1,
                feature_type: FeatureType::None,
                source_slug: source_slug_2,
                hidden: false,
            };
            let skills_as_features = skills
                .iter()
                .map(|s| Feature::new_skill(s, &feature.feature_slug()))
                .collect::<Vec<Feature>>();
            feature.feature_type = FeatureType::Option(FeatureOptions {
                num_choices: count,
                options: skills_as_features,
            });
            features.push(feature);
        }

        let level_patterns = [
            regex!(r"At ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            regex!(r"When you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            regex!(r"Starting at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            regex!(r"By ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            regex!(r"Beginning at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            regex!(r"Beginning when you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"),
        ];

        let desc = self.desc.replace("\n \n", "\n\n");
        let desc_parts = desc.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature {
            source_slug: source_slug.to_string(),
            ..Default::default()
        };
        for line in desc_parts {
            // In the Open5e dataset, features start with a level 3 header.
            if line.len() > 3 && line[0..4] == *"### " {
                if !current_feature.name.is_empty() {
                    // If we haven't encountered any mention of a level
                    // yet for this feature, assume it's level one.
                    if current_feature.level == 0 {
                        current_feature.level = 1;
                    }
                    features.push(current_feature);
                }
                current_feature = Feature {
                    name: line.replace("### ", "").trim().to_string(),
                    source_slug: source_slug.to_string(),
                    ..Default::default()
                };
            } else {
                // Check if this line of the feature description mentions
                // a level at which it applies.
                for pattern in level_patterns.iter() {
                    let matches = pattern.captures(line);

                    if let Some(captures) = matches {
                        if let Some(group) = captures.get(1) {
                            let string = group.as_str();
                            let level = str::parse::<i32>(string)
                                .unwrap_or_else(|_| {
                                    panic!(
                                        "Parsed a non-numeric level: {}",
                                        string
                                    )
                                });
                            if current_feature.level != 0 {
                                let new_feature = Feature {
                                    level,
                                    name: current_feature.name.clone(),
                                    desc: String::new(),
                                    feature_type: FeatureType::None,
                                    source_slug: source_slug.to_string(),
                                    hidden: false,
                                };
                                features.push(current_feature);
                                current_feature = new_feature;
                            } else {
                                current_feature.level = level;
                            }
                            break;
                        }
                    }
                }
                if current_feature.desc != String::default() {
                    current_feature.desc += "\n\n";
                }
                current_feature.desc += line.trim();
            }
        }
        // If we haven't encountered any mention of a level
        // yet for this feature, assume it's level one.
        if current_feature.level == 0 {
            current_feature.level = 1;
        }
        features.push(current_feature);

        // Add saving throw proficencies
        let saves = self.prof_saving_throws.split_whitespace();
        for save in saves {
            let string = save.replace(',', "");
            let attribute = Ability::from_string(&string);
            if let Some(attribute) = attribute {
                features.push(Feature {
                    name: format!("{string} Saving Throw"),
                    desc: format!(
                        "You have proficiency in {string} saving throws.",
                    ),
                    level: 1,
                    feature_type: FeatureType::SavingThrow(attribute),
                    source_slug: source_slug.to_string(),
                    hidden: true,
                })
            }
        }

        features.sort_by(|a, b| a.level.cmp(&b.level));
        features
    }
}
const SKILL_LIST: [&str; 18] = [
    "Athletics",
    "Acrobatics",
    "Sleight of Hand",
    "Stealth",
    "Arcana",
    "History",
    "Investigation",
    "Nature",
    "Religion",
    "Animal Handling",
    "Insight",
    "Medicine",
    "Perception",
    "Survival",
    "Deception",
    "Intimidation",
    "Performance",
    "Persuasion",
];
impl Background {
    pub fn features(&self) -> Vec<Feature> {
        let source_slug = format!("background:{}", self.slug);
        let mut features = vec![];

        // Description of background
        features.push(Feature {
            name: format!("{} Decription", self.name),
            desc: self.desc.to_string(),
            level: 1,
            feature_type: FeatureType::Fluff,
            source_slug: source_slug.clone(),
            hidden: false,
        });

        // Primary feature for background
        features.push(Feature {
            name: self.feature.to_string(),
            desc: self.feature_desc.to_string(),
            level: 1,
            feature_type: FeatureType::None,
            source_slug: source_slug.clone(),
            hidden: false,
        });

        // Suggested characteristics for background
        features.push(Feature {
            name: format!("{} Characteristics", self.name),
            desc: self.suggested_characteristics.to_string(),
            level: 1,
            feature_type: FeatureType::Fluff,
            source_slug: source_slug.clone(),
            hidden: false,
        });

        if let Some(skill_profs) = &self.skill_proficiencies {
            if self.document_slug != "a5e" && !skill_profs.contains(" or ") {
                let skills = &skill_profs;
                for word in skills.split(',') {
                    let word = word.trim();
                    features.push(Feature {
                        name: "Skill Proficiencies".to_string(),
                        desc: skill_profs.to_string(),
                        level: 1,
                        feature_type: FeatureType::SkillProficency(
                            word.to_string(),
                        ),
                        source_slug: source_slug.to_string(),
                        hidden: true,
                    });
                }
            }
        }

        if let Some(tool_prof_string) = &self.tool_proficiencies {
            let split = tool_prof_string.split(',');
            for substring in split {
                if substring == "Two of your choice" {
                    let feature = Feature {
                        name: "Tool Proficency".to_string(),
                        desc: substring.to_string(),
                        level: 1,
                        feature_type: FeatureType::OtherProficency(
                            "Two tools of your choice".to_string(),
                        ),
                        source_slug: source_slug.to_string(),
                        hidden: true,
                    };
                    features.push(feature);
                } else if substring != "No additional tool proficiencies" {
                    let feature = Feature {
                        name: "Tool Proficency".to_string(),
                        desc: substring.to_string(),
                        level: 1,
                        feature_type: FeatureType::OtherProficency(
                            substring.to_string(),
                        ),
                        source_slug: source_slug.to_string(),
                        hidden: true,
                    };
                    features.push(feature);
                }
            }
            log!("{}", tool_prof_string);
        }
        if let Some(languages_string) = &self.languages {
            if languages_string == "One language of your choice, typically your adopted parents' language (if any)"{
                let feature = Feature {
                    name: "Language".to_string(),
                    desc: languages_string.to_string(),
                    level: 1,
                    feature_type: FeatureType::OtherProficency(
                        languages_string.to_string(),
                    ),
                    source_slug: source_slug.to_string(),
                    hidden: true,
                };
                features.push(feature);
            } else if languages_string != "No additional languages" {
                let split = languages_string.split(',');
                for substring in split {
                    if substring == "One of your choice" 
                    || substring == "One language of your choice" {
                        let feature = Feature {
                            name: "Language".to_string(),
                            desc: substring.to_string(),
                            level: 1,
                            feature_type: FeatureType::OtherProficency(
                                "One language of your choice".to_string(),
                            ),
                            source_slug: source_slug.to_string(),
                            hidden: true,
                        };
                        features.push(feature);
                    } else if substring == "Two of your choice" {
                        let feature = Feature {
                            name: "Language".to_string(),
                            desc: substring.to_string(),
                            level: 1,
                            feature_type: FeatureType::OtherProficency(
                                "Two languages of your choice".to_string(),
                            ),
                            source_slug: source_slug.to_string(),
                            hidden: true,
                        };
                        features.push(feature);
                    } else {
                        let feature = Feature {
                            name: "Language".to_string(),
                            desc: substring.to_string(),
                            level: 1,
                            feature_type: FeatureType::OtherProficency(
                                format!("Language: {}", substring),
                            ),
                            source_slug: source_slug.to_string(),
                            hidden: true,
                        };
                        features.push(feature);
                    }
                }
            }
            //log!("{}", tool_prof_string);
        }

        features
    }
}
#[allow(dead_code)]
impl Weapon {
    pub fn is_finesse(&self) -> bool {
        match &self.properties {
            Some(p) => p.contains(&String::from("finesse")),
            None => false,
        }
    }
    pub fn is_light(&self) -> bool {
        match &self.properties {
            Some(p) => p.contains(&String::from("light")),
            None => false,
        }
    }
    pub fn is_reach(&self) -> bool {
        match &self.properties {
            Some(p) => p.contains(&String::from("reach")),
            None => false,
        }
    }
    pub fn is_ranged(&self) -> bool {
        self.category.contains(&String::from("Ranged"))
    }
    pub fn to_attack(&self) -> AttackAction {
        AttackAction {
            name: self.name.to_string(),
            ability: if self.is_finesse() {
                Ability::Dexterity
            } else {
                Ability::Strength
            },
            damage_base: self.damage_dice.to_string(),
            proficient: false,
            attack_type: if self.is_ranged() {
                AttackType::Ranged
            } else {
                AttackType::Melee
            },
            reach: if self.is_reach() { 10 } else { 5 },
            damage_type: self.damage_type.to_string(),
        }
    }
}
