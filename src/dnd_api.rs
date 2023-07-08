use regex_static::static_regex;
use regex_static::Regex;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesAPI {
    pub count: i64,
    pub next: Value,
    pub previous: Value,
    pub results: Vec<Species>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub name: String,
    pub slug: String,
    pub desc: String,
    #[serde(rename = "asi_desc")]
    pub asi_desc: String,
    pub asi: Vec<Asi>,
    pub age: String,
    pub alignment: String,
    pub size: String,
    pub speed: Speed,
    #[serde(rename = "speed_desc")]
    pub speed_desc: String,
    pub languages: String,
    pub vision: String,
    pub traits: String,
    pub subraces: Vec<Subspecies>,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__license_url")]
    pub document_license_url: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asi {
    pub attributes: Vec<String>,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speed {
    pub walk: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SpeciesFeature {
    pub name: String,
    pub desc: String,
}
impl Species {
    pub fn features(&self) -> Vec<SpeciesFeature> {
        let mut features: Vec<SpeciesFeature> = vec![];

        let desc_parts = self.traits.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = SpeciesFeature::default();
        for line in desc_parts {
            let line = line.replace("**_", "***").replace("_**", "***");
            let re = Regex::new(r"\*\*\*(.+)\*\*\*(.+)");
            if let Ok(re) = re {
                match re.captures(&line) {
                    Some(captures) => {
                        if current_feature.name != "" {
                            features.push(current_feature);
                            current_feature = SpeciesFeature::default();
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
        features.push(current_feature);
        features
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subspecies {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<Asi>,
    pub traits: String,
    #[serde(rename = "asi_desc")]
    pub asi_desc: String,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassesAPI {
    pub count: i64,
    pub next: Value,
    pub previous: Value,
    pub results: Vec<Class>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name: String,
    pub slug: String,
    pub desc: String,
    #[serde(rename = "hit_dice")]
    pub hit_dice: String,
    #[serde(rename = "hp_at_1st_level")]
    pub hp_at_1st_level: String,
    #[serde(rename = "hp_at_higher_levels")]
    pub hp_at_higher_levels: String,
    #[serde(rename = "prof_armor")]
    pub prof_armor: String,
    #[serde(rename = "prof_weapons")]
    pub prof_weapons: String,
    #[serde(rename = "prof_tools")]
    pub prof_tools: String,
    #[serde(rename = "prof_saving_throws")]
    pub prof_saving_throws: String,
    #[serde(rename = "prof_skills")]
    pub prof_skills: String,
    pub equipment: String,
    pub table: String,
    #[serde(rename = "spellcasting_ability")]
    pub spellcasting_ability: String,
    #[serde(rename = "subtypes_name")]
    pub subtypes_name: String,
    pub archetypes: Vec<Archetype>,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__license_url")]
    pub document_license_url: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}

impl Class {
    pub fn features(this: &Self) -> Vec<Feature> {
        let patterns = vec![
            static_regex!(r"At ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"When you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"Starting at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"By ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"Beginning at ([0-9]{1,2})[a-zA-Z]{1,2} level"),
            static_regex!(r"Beginning when you reach ([0-9]{1,2})[a-zA-Z]{1,2} level"),
        ];
        let mut features: Vec<Feature> = vec![];

        let desc = this.desc.replace("\n \n", "\n\n");
        let desc_parts = desc.split("\n\n").collect::<Vec<&str>>();

        let mut current_feature = Feature::default();
        for line in desc_parts {
            if line.len() > 3 && line[0..4].to_string() == "### ".to_string() {
                if current_feature.level != 0 {
                    features.push(current_feature)
                }
                current_feature = Feature::default();
                let title = line.replace("### ", "");
                current_feature.name = title.trim().to_string();
            } else {
                for pattern in &patterns {
                    let matches = pattern.captures(line);

                    if let Some(captures) = matches {
                        if let Some(group) = captures.get(1) {
                            let string = group.as_str();
                            let level = str::parse::<i32>(string)
                                .expect(&format!("Parsed a non-numeric level: {}", string));

                            if current_feature.level != 0 {
                                let new_feature = Feature {
                                    level,
                                    name: current_feature.name.clone(),
                                    desc: String::new(),
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
        features.sort_by(|a, b| a.level.cmp(&b.level));
        features
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    desc: String,
    level: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Archetype {
    pub name: String,
    pub slug: String,
    pub desc: String,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__license_url")]
    pub document_license_url: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundsAPI {
    pub count: i64,
    pub next: Value,
    pub previous: Value,
    pub results: Vec<Background>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    pub name: String,
    pub desc: String,
    pub slug: String,
    #[serde(rename = "skill_proficiencies")]
    pub skill_proficiencies: String,
    #[serde(rename = "tool_proficiencies")]
    pub tool_proficiencies: Option<String>,
    pub languages: Option<String>,
    pub equipment: String,
    pub feature: String,
    #[serde(rename = "feature_desc")]
    pub feature_desc: String,
    #[serde(rename = "suggested_characteristics")]
    pub suggested_characteristics: String,
    #[serde(rename = "document__slug")]
    pub document_slug: String,
    #[serde(rename = "document__title")]
    pub document_title: String,
    #[serde(rename = "document__license_url")]
    pub document_license_url: String,
    #[serde(rename = "document__url")]
    pub document_url: String,
}
