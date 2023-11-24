use crate::{
    api::api_extensions::FeatureType, character_model::*, components::*,
    get_current_features, get_prof_bonus,
};
use leptos::{expect_context, html::*, prelude::*};

pub fn ProfPanel() -> HtmlElement<Div> {
    let features = get_current_features();
    let saves = Signal::derive(move || {
        features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::SavingThrow(ability) = &f.feature_type {
                    Some(ability.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Ability>>()
    });
    let skills = Signal::derive(move || {
        features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::SkillProficency(prof) = &f.feature_type {
                    Some(prof)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<String>>()
    });
    let other_profs = Signal::derive(move || {
        features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::OtherProficency(prof) = &f.feature_type {
                    Some(prof)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<String>>()
    });
    BoxedColumn()
        .child(h1().child("Proficencies:"))
        .child(
            ul().classes("nav nav-tabs mb-3")
                .id("proficencyTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab("saves-tab", true, "Saves"),
                    Tab("skills-tab", false, "Skills"),
                    Tab("other-tab", false, "Other"),
                ]),
        )
        .child(
            ul().style("padding-left", "0rem").child(
                div()
                    .classes("tab-content")
                    .id("proficencyTabsContent")
                    .child(vec![
                        TabPanel("saves-tab", true, SavesDisplay(saves)),
                        TabPanel("skills-tab", false, SkillsTab(skills)),
                        TabPanel(
                            "other-tab",
                            false,
                            OtherProfsTab(other_profs),
                        ),
                    ]),
            ),
        )
}

pub fn SavesDisplay(saves: Signal<Vec<Ability>>) -> HtmlDiv {
    let ability_scores = expect_context::<AbilityScoresReactive>();
    div().child(
        ul().classes("list-group").child(
            [
                Ability::Strength,
                Ability::Dexterity,
                Ability::Constitution,
                Ability::Wisdom,
                Ability::Intelligence,
                Ability::Charisma,
            ]
            .iter()
            .map(|ability| {
                li().classes("list-group-item").child(
                    div()
                        .classes("d-flex justify-content-between")
                        .child(div().child(ability.to_string().to_string()))
                        .child(div().child(move || {
                            calc_save(
                                ability_scores,
                                saves,
                                ability.clone(),
                                get_prof_bonus(),
                            )
                        })),
                )
            })
            .collect::<Vec<HtmlElement<Li>>>(),
        ),
    )
}

fn calc_save(
    ability_scores: AbilityScoresReactive,
    saves: Signal<Vec<Ability>>,
    ability: Ability,
    proficiency_bonus: Signal<i32>,
) -> i32 {
    let bonus = if saves().contains(&ability) {
        proficiency_bonus()
    } else {
        0
    };
    ability_scores.get_ability_mod(&ability) + bonus
}

pub fn SkillsTab(skills: Signal<Vec<String>>) -> HtmlDiv {
    div().child(ul().classes("list-group").child(move || {
        skills()
            .iter()
            .map(|skill| {
                li().classes("list-group-item")
                    .child(div().child(skill.to_string()))
            })
            .collect::<Vec<HtmlElement<Li>>>()
    }))
}

pub fn OtherProfsTab(other_profs: Signal<Vec<String>>) -> HtmlDiv {
    div().child(ul().classes("list-group").child(move || {
        other_profs()
            .iter()
            .map(|prof| {
                li().classes("list-group-item")
                    .child(div().child(prof.to_string()))
            })
            .collect::<Vec<HtmlElement<Li>>>()
    }))
}
