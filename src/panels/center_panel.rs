#![allow(non_snake_case)]

use crate::api::api_model::Weapon;
use crate::api::FuturesWrapper;
use crate::character_model::Ability;
use crate::character_model::AbilityScoresReactive;
use crate::components::*;
use crate::get_prof_bonus;
use crate::markdown::parse_markdown;
use leptos::create_rw_signal;
use leptos::create_signal;
use leptos::ev;
use leptos::event_target_value;
use leptos::expect_context;
use leptos::html::*;
use leptos::log;
use leptos::provide_context;
use leptos::RwSignal;
use leptos::Scope;
use leptos::Signal;
use leptos::SignalUpdate;
use leptos::SignalWith;
use web_sys::SubmitEvent;

pub fn CenterPanel(cx: Scope) -> HtmlDiv {
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
                        TabPanel(cx, "action-tab", true, ActionsTab(cx)),
                        //TabPanel(cx, "species-tab", false, species_tab),
                        //TabPanel(cx, "background-tab", false, background_tab),
                    ]),
            ),
        )
}

fn ActionsTab(cx: Scope) -> HtmlDiv {
    let attack_list: RwSignal<Vec<AttackAction>> = create_rw_signal(cx, vec![]);
    provide_context(cx, attack_list);
    let create_button = button(cx)
        .attr("type", "button")
        .classes("btn btn-primary mb-2")
        .attr("data-bs-toggle", "modal")
        .attr("data-bs-target", "#attackCreateModal")
        .child("Add Attack");
    div(cx)
        .child(create_button)
        .child(
            div(cx)
                .classes("accordion mb-2")
                .id("featuresAccordion")
                .child(move || {
                    attack_list.with(|list| {
                        list.iter()
                            .map(|attack| AttackActionDisplay(cx, attack))
                            .collect::<DivList>()
                    })
                }),
        )
        .child(AttackCreationModal(cx))
}

fn AttackActionDisplay(cx: Scope, attack: &AttackAction) -> HtmlDiv {
    AccordionItem(
        cx,
        div(cx).child(attack.name.to_string()),
        div(cx).inner_html(parse_markdown(&attack.generate_description(
            get_prof_bonus(cx),
            expect_context::<AbilityScoresReactive>(cx),
        ))),
    )
}
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

fn AttackCreationModal(cx: Scope) -> HtmlElement<Div> {
    div(cx)
        .classes("modal fade")
        .id("attackCreateModal")
        .attr("tabindex", "-1")
        .attr("aria-labelledby", "attackCreateModalLabel")
        .attr("aria-hidden", "true")
        .child(
            div(cx)
                .classes(
                    "modal-dialog modal-dialog-centered container container-lg",
                )
                .child(
                    div(cx)
                        .classes("modal-content")
                        .child(
                            div(cx)
                                .classes("modal-header")
                                .child(
                                    h1(cx)
                                        .classes("modal-title fs-5")
                                        .id("attackCreateModalLabel")
                                        .child("Options"),
                                )
                                .child(
                                    button(cx)
                                        .attr("type", "button")
                                        .classes("btn-close")
                                        .attr("data-bs-dismiss", "modal")
                                        .attr("aria-label", "Close"),
                                ),
                        )
                        .child(
                            div(cx)
                                .classes("modal-body")
                                .child(CreateAttackForm(cx)),
                        ),
                ),
        )
}

fn CreateAttackForm(cx: Scope) -> HtmlElement<Form> {
    let attack_list = expect_context::<RwSignal<Vec<AttackAction>>>(cx);
    // Declare the values to be used in the form below
    let (name, set_name) = create_signal(cx, String::new());
    let (damage_base, set_damage_base) = create_signal(cx, String::new());
    let (reach, set_reach) = create_signal(cx, 0);
    let (damage_type, set_damage_type) = create_signal(cx, String::new());
    let on_submit = move |e: SubmitEvent| {
        log!("Submit event called!");
        attack_list.update(|list| {
            log!("Hello from within attack list update!");
            list.push(AttackAction {
                name: name(),
                ability: Ability::Strength,
                damage_base: damage_base(),
                proficient: true,
                attack_type: AttackType::Melee,
                reach: reach(),
                damage_type: damage_type(),
            });
            log!("Attack list now contains {} items!", list.len());
        });
        // Prevent the form from "submitting" and reloading the page
        e.prevent_default();
    };
    let weapons = expect_context::<FuturesWrapper>(cx).weapons;
    form(cx)
        .on(ev::submit, on_submit)
        .child(
            select(cx)
                .classes("form-select")
                .child(option(cx).child("Use a template..."))
                .child(move || {
                    weapons.with(cx, |weapons| {
                        weapons
                            .iter()
                            .map(|weapon| {
                                option(cx)
                                    .prop("value", weapon.slug.to_string())
                                    .child(weapon.name.to_string())
                            })
                            .collect::<OptionList>()
                    })
                })
                .on(ev::change, move |ev| {
                    let val = event_target_value(&ev);
                    weapons.with(cx, |weapons| {
                        let weapon = weapons.iter().find(|w| w.slug == val);
                        if let Some(weapon) = weapon {
                            set_name(weapon.name.to_string());
                            set_damage_base(weapon.damage_dice.to_string());
                            set_reach(if weapon.is_reach() { 10 } else { 5 });
                            set_damage_type(weapon.damage_type.to_string())
                        }
                    });
                }),
        )
        .child(label(cx).child("Name:"))
        .child(
            input(cx)
                .classes("form-control mb-2")
                .prop("value", name)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    set_name(val.to_string())
                }),
        )
        .child(label(cx).child("Damage:"))
        .child(
            input(cx)
                .classes("form-control mb-2")
                .prop("value", damage_base)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    set_damage_base(val.to_string())
                }),
        )
        .child(label(cx).child("Reach (ft.):"))
        .child(
            input(cx)
                .classes("form-control mb-2")
                .prop("value", reach)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    if let Ok(num) = str::parse::<i32>(&val) {
                        set_reach(num)
                    }
                }),
        )
        .child(label(cx).child("Damage Type:"))
        .child(
            input(cx)
                .classes("form-control mb-2")
                .prop("value", damage_type)
                .on(ev::input, move |e| {
                    let val = event_target_value(&e);
                    set_damage_type(val.to_string())
                }),
        )
        .child(
            button(cx)
                .attr("type", "submit")
                .classes("btn btn-primary")
                .attr("data-bs-dismiss", "modal")
                .attr("aria-label", "Add")
                .child("Add"),
        )
}
