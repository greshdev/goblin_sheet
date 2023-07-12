use leptos::log;
use regex_static::{static_regex, Regex};
use serde::{Deserialize, Serialize};

use crate::character_model::CharacterAsi;

use super::api_model::*;

#[derive(Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct Feature {
    pub name: String,
    pub desc: String,
    pub level: i32,
    pub feature_type: FeatureType,
    pub source_slug: String,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum FeatureType {
    Asi(CharacterAsi),
    Proficiency,
    #[default]
    Fluff,
    None,
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
                });
            }
        }

        let desc_parts = self.traits.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature {
            source_slug: format!("species:{}", self.slug),
            ..Default::default()
        };
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let re = Regex::new(r"\*\*\*(.+)\*\*\*(.+)");
            if let Ok(re) = re {
                match re.captures(&line) {
                    Some(captures) => {
                        if !current_feature.name.is_empty() {
                            features.push(current_feature);
                            current_feature = Feature::default();
                            current_feature.source_slug =
                                format!("species:{}", self.slug);
                        }
                        if let Some(group) = captures.get(1) {
                            current_feature.name = group.as_str().to_string();
                        }
                        if let Some(group) = captures.get(2) {
                            current_feature.desc += group.as_str();
                        }
                    }
                    None => {
                        current_feature.desc += &line;
                    }
                }
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
                });
            }
        }

        let mut current_feature = Feature {
            source_slug: format!("subspecies:{}", self.slug),
            ..Default::default()
        };
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let re = Regex::new(r"\*\*\*(.+)\*\*\*(.+)");
            if let Ok(re) = re {
                match re.captures(&line) {
                    Some(captures) => {
                        if !current_feature.name.is_empty() {
                            features.push(current_feature);
                            current_feature = Feature::default();
                            current_feature.source_slug =
                                format!("subspecies:{}", self.slug);
                        }
                        if let Some(group) = captures.get(1) {
                            current_feature.name = group.as_str().to_string();
                        }
                        if let Some(group) = captures.get(2) {
                            current_feature.desc += group.as_str();
                        }
                    }
                    None => {
                        current_feature.desc += &line;
                    }
                }
            }
        }
        if !current_feature.name.is_empty() {
            features.push(current_feature);
        }
        features
    }
}

impl Class {
    pub fn features(&self) -> Vec<Feature> {
        let patterns = vec![
            static_regex!(r"At ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"When you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"Starting at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"By ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"Beginning at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(
                r"Beginning when you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"
            ),
        ];
        let mut features: Vec<Feature> = vec![];

        let desc = self.desc.replace("\n \n", "\n\n");
        let desc_parts = desc.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature {
            source_slug: format!("class:{}", self.slug),
            ..Default::default()
        };
        for line in desc_parts {
            if line.len() > 3 && line[0..4] == *"### " {
                if !current_feature.name.is_empty() {
                    if current_feature.level == 0 {
                        current_feature.level = 1;
                    }
                    features.push(current_feature);
                }
                current_feature = Feature::default();
                current_feature.source_slug = format!("class:{}", self.slug);
                //current_feature.level = 1;
                let title = line.replace("### ", "");
                current_feature.name = title.trim().to_string();
            } else {
                // Check if this line of the feature description mentions
                // a level at which it applies.
                for pattern in &patterns {
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
                                    source_slug: format!("class:{}", self.slug),
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
        if current_feature.level == 0 {
            current_feature.level = 1;
        }
        features.push(current_feature);
        features.sort_by(|a, b| a.level.cmp(&b.level));
        features
    }
}

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
        });

        // Primary feature for background
        features.push(Feature {
            name: self.feature.to_string(),
            desc: self.feature_desc.to_string(),
            level: 1,
            feature_type: FeatureType::None,
            source_slug: source_slug.clone(),
        });

        // Suggested characteristics for background
        features.push(Feature {
            name: format!("{} Characteristics", self.name),
            desc: self.suggested_characteristics.to_string(),
            level: 1,
            feature_type: FeatureType::Fluff,
            source_slug: source_slug.clone(),
        });

        features
    }
}
