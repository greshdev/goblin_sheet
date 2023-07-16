use crate::{
    api::api_extensions::FeatureType, character_model::*, components::*,
    get_current_features, get_prof_bonus,
};
use leptos::{expect_context, html::*, prelude::*, Scope};

pub fn ProfPanel(cx: Scope) -> HtmlElement<Div> {
    let features = get_current_features(cx);
    let saves = Signal::derive(cx, move || {
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
    let skills = Signal::derive(cx, move || {
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
    let other_profs = Signal::derive(cx, move || {
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
    BoxedColumn(cx)
        .child(h1(cx).child("Proficencies:"))
        .child(
            ul(cx)
                .classes("nav nav-tabs mb-3")
                .id("proficencyTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab(cx, "saves-tab", true, "Saves"),
                    Tab(cx, "skills-tab", false, "Skills"),
                    Tab(cx, "other-tab", false, "Other"),
                ]),
        )
        .child(
            ul(cx).style("padding-left", "0rem").child(
                div(cx)
                    .classes("tab-content")
                    .id("proficencyTabsContent")
                    .child(vec![
                        TabPanel(
                            cx,
                            "saves-tab",
                            true,
                            SavesDisplay(cx, saves),
                        ),
                        TabPanel(
                            cx,
                            "skills-tab",
                            false,
                            SkillsTab(cx, skills),
                        ),
                        TabPanel(
                            cx,
                            "other-tab",
                            false,
                            OtherProfsTab(cx, other_profs),
                        ),
                    ]),
            ),
        )
}

pub fn SavesDisplay(cx: Scope, saves: Signal<Vec<Ability>>) -> HtmlDiv {
    let ability_scores = expect_context::<AbilityScoresReactive>(cx);
    div(cx).child(
        ul(cx).classes("list-group").child(
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
                li(cx).classes("list-group-item").child(
                    div(cx)
                        .classes("d-flex justify-content-between")
                        .child(div(cx).child(ability.to_string().to_string()))
                        .child(div(cx).child(move || {
                            calc_save(
                                ability_scores,
                                saves,
                                ability.clone(),
                                get_prof_bonus(cx),
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

pub fn SkillsTab(cx: Scope, skills: Signal<Vec<String>>) -> HtmlDiv {
    div(cx).child(ul(cx).classes("list-group").child(move || {
        skills()
            .iter()
            .map(|skill| {
                li(cx)
                    .classes("list-group-item")
                    .child(div(cx).child(skill.to_string()))
            })
            .collect::<Vec<HtmlElement<Li>>>()
    }))
}

pub fn OtherProfsTab(cx: Scope, other_profs: Signal<Vec<String>>) -> HtmlDiv {
    div(cx).child(ul(cx).classes("list-group").child(move || {
        other_profs()
            .iter()
            .map(|prof| {
                li(cx)
                    .classes("list-group-item")
                    .child(div(cx).child(prof.to_string()))
            })
            .collect::<Vec<HtmlElement<Li>>>()
    }))
}
