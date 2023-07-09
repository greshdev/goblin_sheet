use regex_static::{static_regex, Regex};
use serde::{Deserialize, Serialize};

use crate::character_model::CharacterAsi;

use super::api_model::*;

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
                });
            }
        }

        let desc_parts = self.traits.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature::default();
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let re = Regex::new(r"\*\*\*(.+)\*\*\*(.+)");
            if let Ok(re) = re {
                match re.captures(&line) {
                    Some(captures) => {
                        if current_feature.name != "" {
                            features.push(current_feature);
                            current_feature = Feature::default();
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
        if current_feature.name != "" {
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
                });
            }
        }

        let mut current_feature = Feature::default();
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let re = Regex::new(r"\*\*\*(.+)\*\*\*(.+)");
            if let Ok(re) = re {
                match re.captures(&line) {
                    Some(captures) => {
                        if current_feature.name != "" {
                            features.push(current_feature);
                            current_feature = Feature::default();
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
        if current_feature.name != "" {
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

        let mut current_feature = Feature::default();
        for line in desc_parts {
            if line.len() > 3 && line[0..4].to_string() == "### ".to_string() {
                if current_feature.name != "" {
                    features.push(current_feature);
                }
                current_feature = Feature::default();
                //current_feature.level = 1;
                let title = line.replace("### ", "");
                current_feature.name = title.trim().to_string();
            } else {
                for pattern in &patterns {
                    let matches = pattern.captures(line);

                    if let Some(captures) = matches {
                        if let Some(group) = captures.get(1) {
                            let string = group.as_str();
                            let level =
                                str::parse::<i32>(string).expect(&format!(
                                    "Parsed a non-numeric level: {}",
                                    string
                                ));

                            if current_feature.level != 0 {
                                let new_feature = Feature {
                                    level: level,
                                    name: current_feature.name.clone(),
                                    desc: String::new(),
                                    feature_type: FeatureType::None,
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
                if current_feature.level == 0 {
                    current_feature.level = 1;
                }
            }
        }
        features.push(current_feature);
        features.sort_by(|a, b| a.level.cmp(&b.level));
        features
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub desc: String,
    pub level: i32,
    pub feature_type: FeatureType,
}

#[derive(Serialize, Deserialize, Default)]
pub enum FeatureType {
    Asi(CharacterAsi),
    Proficiency,
    #[default]
    None,
}
