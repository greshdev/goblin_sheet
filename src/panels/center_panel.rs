#![allow(non_snake_case)]

use crate::api::FuturesWrapper;
use crate::character_model::Ability;
use crate::character_model::AbilityScoresReactive;
use crate::character_model::AttackAction;
use crate::character_model::AttackType;
use crate::components::*;
use crate::get_prof_bonus;
use crate::markdown::parse_markdown;
use leptos::create_signal;
use leptos::ev;
use leptos::event_target_value;
use leptos::expect_context;
use leptos::html::*;
use leptos::leptos_dom::log;
use leptos::RwSignal;
use leptos::SignalUpdate;
use leptos::SignalWith;
use web_sys::SubmitEvent;

pub fn CenterPanel() -> HtmlDiv {
    div()
        .child(
            ul().classes("nav nav-tabs mb-3")
                .id("centerTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab("action-tab", true, "Actions"),
                    Tab("spell-tab", false, "Spells"),
                    Tab("equipment-tab", false, "Equipment"),
                ]),
        )
        .child(
            ul().style("padding-left", "0rem").child(
                div()
                    .style("overflow-y", "auto")
                    .classes("tab-content")
                    .id("featuresTabsContent")
                    .child(vec![
                        TabPanel("action-tab", true, ActionsTab()),
                        //TabPanel("spell-tab", true, SpellsTab()),
                        //TabPanel("equipment-tab", true, EquipmentTab()),
                    ]),
            ),
        )
}

fn ActionsTab() -> HtmlDiv {
    let attack_list = expect_context::<RwSignal<Vec<AttackAction>>>();
    div()
        .child(
            div()
                .classes("d-flex justify-content-between align-items-center")
                .child(h4().child("Attacks:"))
                .child(
                    h1().child("+")
                        .classes("mt-n1")
                        .attr("role", "button")
                        .attr("data-bs-toggle", "modal")
                        .attr("data-bs-target", "#attackCreateModal"),
                ),
        )
        .child(
            div()
                .classes("accordion mb-2")
                .id("featuresAccordion")
                .child(move || {
                    attack_list.with(|list| {
                        list.iter()
                            .map(AttackActionDisplay)
                            .collect::<DivList>()
                    })
                }),
        )
        .child(AttackCreationModal())
}

fn AttackActionDisplay(attack: &AttackAction) -> HtmlDiv {
    let attack_name = attack.name.to_string();
    let attack_slug = attack.slug.to_string();
    AccordionItem(
        div().child(attack_name),
        div()
            .inner_html(parse_markdown(&attack.generate_description(
                get_prof_bonus(),
                expect_context::<AbilityScoresReactive>(),
            )))
            .child(div().style("cursor", "pointer").child("[Remove]").on(
                ev::click,
                move |_| {
                    let attack_list =
                        expect_context::<RwSignal<Vec<AttackAction>>>();
                    attack_list.update(|current| {
                        let idx =
                            current.iter().position(|e| e.slug == attack_slug);
                        if let Some(idx) = idx {
                            current.remove(idx);
                        }
                    });
                },
            )),
    )
}

fn AttackCreationModal() -> HtmlElement<Div> {
    div()
        .classes("modal fade")
        .id("attackCreateModal")
        .attr("tabindex", "-1")
        .attr("aria-labelledby", "attackCreateModalLabel")
        .attr("aria-hidden", "true")
        .child(
            div()
                .classes(
                    "modal-dialog modal-dialog-centered container container-lg",
                )
                .child(
                    div()
                        .classes("modal-content")
                        .child(
                            div()
                                .classes("modal-header")
                                .child(
                                    h1().classes("modal-title fs-5")
                                        .id("attackCreateModalLabel")
                                        .child("Options"),
                                )
                                .child(
                                    button()
                                        .attr("type", "button")
                                        .classes("btn-close")
                                        .attr("data-bs-dismiss", "modal")
                                        .attr("aria-label", "Close"),
                                ),
                        )
                        .child(
                            div()
                                .classes("modal-body")
                                .child(CreateAttackForm()),
                        ),
                ),
        )
}

fn CreateAttackForm() -> HtmlElement<Form> {
    let attack_list = expect_context::<RwSignal<Vec<AttackAction>>>();
    // Declare the values to be used in the form below
    let (name, set_name) = create_signal(String::new());
    let (damage_base, set_damage_base) = create_signal(String::new());
    let (reach, set_reach) = create_signal(0);
    let (damage_type, set_damage_type) = create_signal(String::new());
    let on_submit = move |e: SubmitEvent| {
        log!("Submit event called!");
        let mut new_action = AttackAction {
            name: name(),
            slug: name(),
            ability: Ability::Strength,
            damage_base: damage_base(),
            proficient: true,
            attack_type: AttackType::Melee,
            reach: reach(),
            damage_type: damage_type(),
        };
        attack_list.update(|list| {
            // If an attack sharing the same slug as this attack
            // already exists in the attack list, add a unique
            // itendifier to the slug of this attack. Repeat
            // until there are no duplicates.
            while list.iter().any(|a| a.slug == new_action.slug) {
                new_action.slug =
                    format!("{}-{}", new_action.slug, uuid::Uuid::new_v4())
            }
            log!("Hello from within attack list update!");
            list.push(new_action);
            log!("Attack list now contains {} items!", list.len());
        });
        // Prevent the form from "submitting" and reloading the page
        e.prevent_default();
    };
    let weapons = expect_context::<FuturesWrapper>().weapons;
    form()
        .on(ev::submit, on_submit)
        .child(
            select()
                .classes("form-select")
                .child(option().child("Use a template..."))
                .child(move || {
                    weapons.with(|weapons| {
                        if let Some(weapons) = weapons {
                            weapons
                                .iter()
                                .map(|weapon| {
                                    option()
                                        .prop("value", weapon.slug.to_string())
                                        .child(weapon.name.to_string())
                                })
                                .collect::<OptionList>()
                        } else {
                            vec![]
                        }
                    })
                })
                .on(ev::change, move |ev| {
                    let val = event_target_value(&ev);
                    weapons.with(|weapons| {
                        if let Some(weapons) = weapons {
                            let weapon = weapons.iter().find(|w| w.slug == val);
                            if let Some(weapon) = weapon {
                                set_name(weapon.name.to_string());
                                set_damage_base(weapon.damage_dice.to_string());
                                set_reach(if weapon.is_reach() {
                                    10
                                } else {
                                    5
                                });
                                set_damage_type(weapon.damage_type.to_string())
                            }
                        }
                    });
                }),
        )
        .child(label().child("Name:"))
        .child(input().classes("form-control mb-2").prop("value", name).on(
            ev::input,
            move |e| {
                let val = event_target_value(&e);
                set_name(val.to_string())
            },
        ))
        .child(label().child("Damage:"))
        .child(
            input()
                .classes("form-control mb-2")
                .prop("value", damage_base)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    set_damage_base(val.to_string())
                }),
        )
        .child(label().child("Reach (ft.):"))
        .child(
            input()
                .classes("form-control mb-2")
                .prop("value", reach)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    if let Ok(num) = str::parse::<i32>(&val) {
                        set_reach(num)
                    }
                }),
        )
        .child(label().child("Damage Type:"))
        .child(
            input()
                .classes("form-control mb-2")
                .prop("value", damage_type)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    set_damage_type(val.to_string())
                }),
        )
        .child(
            button()
                .attr("type", "submit")
                .classes("btn btn-primary")
                .attr("data-bs-dismiss", "modal")
                .attr("aria-label", "Add")
                .child("Add"),
        )
}
