#![allow(non_snake_case)]

use crate::character_model::Ability;
use crate::character_model::AbilityScoresReactive;
use crate::components::*;
use crate::get_prof_bonus;
use crate::markdown::parse_markdown;
use leptos::create_rw_signal;
use leptos::expect_context;
use leptos::html::*;
use leptos::RwSignal;
use leptos::Scope;
use leptos::Signal;
use leptos::SignalWith;

pub fn CenterPanel(
    cx: Scope
) -> HtmlDiv {
    div(cx)
        .child(
            ul(cx)
                .classes("nav nav-tabs mb-3")
                .id("centerTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab(cx, "action-tab", true, "Actions"),
                    Tab(cx, "spell-tab", false, "Spells"),
                    Tab(cx, "equipment-tab", false, "Equipment"),
                ]),
        )
        .child(
            ul(cx).style("padding-left", "0rem").child(
                div(cx)
                    .style("overflow-y", "auto")
                    .classes("tab-content")
                    .id("featuresTabsContent")
                    .child(vec![
                        div(cx),
                        TabPanel(
                            cx,
                            "action-tab",
                            true,
                            ActionsTab(cx,),
                        ),
                        //TabPanel(cx, "species-tab", false, species_tab),
                        //TabPanel(cx, "background-tab", false, background_tab),
                    ]),
            ),
        )
}

fn ActionsTab(cx: Scope) -> HtmlDiv {
    let attack_list: RwSignal<Vec<AttackAction>> = create_rw_signal(
        cx,
        vec![AttackAction {
            name: "Longsword".to_string(),
            ability: Ability::Strength,
            damage_base: "1d8".to_string(),
            proficient: true,
            attack_type: AttackType::Melee,
            reach: 5,
            damage_type: "slashing".to_string(),
        }],
    );
    div(cx).child(
        button(cx).classes("btn btn-primary mb-2").child("Add Action")
    ).child(div(cx).classes("accordion mb-2").id("featuresAccordion").child(
        move || {
            attack_list.with(|list| {
                list.iter()
                    .map(|attack| {
                        AttackActionDisplay(cx,attack)
                    })
                    .collect::<DivList>()
            })
        },
    ))
}

fn AttackActionDisplay(cx: Scope, attack: &AttackAction) -> HtmlDiv {
    AccordionItem(
        cx,
        div(cx).child(attack.name.to_string()),
        div(cx).inner_html(parse_markdown(
            &attack.generate_description(
                get_prof_bonus(cx), 
                expect_context::<AbilityScoresReactive>(cx)
            ),
        )),
    )
}

struct AttackAction {
    pub name: String,
    pub ability: Ability,
    pub damage_base: String,
    pub proficient: bool,
    pub attack_type: AttackType,
    pub reach: i32,
    pub damage_type: String,
}


enum AttackType {
    Melee,
    Ranged,
}
impl AttackType {
    pub fn to_string(&self) -> String {
        match self {
            AttackType::Melee => "Melee",
            AttackType::Ranged => "Ranged",
        }
        .to_string()
    }
}
impl AttackAction {
    pub fn generate_description(
        &self,
        proficency_bonus: Signal<i32>,
        ability_scores: AbilityScoresReactive,
    ) -> String {
        let ab_mod = ability_scores.get_ability_mod(&self.ability);
        let to_hit = ab_mod
            + if self.proficient {
                proficency_bonus()
            } else {
                0
            };
        format!(
            "_{} Weapon Attack:_ +{} to hit, reach {} ft., one target. _Hit:_ {} + {} {} damage.",
            self.attack_type.to_string(),
            to_hit,
            self.reach,
            self.damage_base, 
            ab_mod,
            self.damage_type
        )
    }
}
