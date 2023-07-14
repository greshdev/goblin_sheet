use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::character_model::Ability;
use crate::character_model::CharacterAsi;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesAPI {
    pub count: i32,
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
    pub asi: Vec<ApiAsi>,
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
pub struct ApiAsi {
    pub attributes: Vec<String>,
    pub value: i32,
}
impl ApiAsi {
    pub fn get_asis(&self, source: &str) -> Vec<CharacterAsi> {
        let mut out = vec![];
        for att in &self.attributes {
            if let Some(ability) = Ability::from_string(att) {
                out.push(CharacterAsi {
                    score: ability,
                    source_slug: source.to_string(),
                    amount: self.value,
                })
            }
        }
        out
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speed {
    pub walk: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subspecies {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<ApiAsi>,
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
    pub count: i32,
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
    pub count: i32,
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
