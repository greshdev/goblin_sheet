#![allow(non_snake_case)]

mod api;
mod character_model;
mod components;
mod markdown;
mod panels;

use crate::api::FuturesWrapper;
use crate::character_model::AbilityScoresReactive;
use crate::character_model::CharacterAsi;
use crate::character_model::CharacterDetails;
use crate::components::*;
use api::api_extensions::*;
use api::api_model::Species;
use character_model::Ability;
use panels::feature_panel::BackgroundTab;
use panels::feature_panel::ClassTab;
use panels::feature_panel::FeaturePanel;
use panels::feature_panel::SpeciesTab;
use panels::header_panel::HeaderPanel;
use panels::stats_panel::StatsPanel;

use leptos::{component, IntoView, Scope};
use leptos::{html::*, *};

const CHAR_STORAGE_KEY: &str = "char_sheet_character";
const OPTIONS_STORAGE_KEY: &str = "char_sheet_selected_optional_features";

fn load_character() -> CharacterDetails {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(data)) = local_storage.get_item(CHAR_STORAGE_KEY) {
                if let Ok(character) =
                    serde_json::from_str::<CharacterDetails>(&data)
                {
                    return character;
                }
            }
        }
    }
    CharacterDetails::new()
}

fn write_character_to_local_storage(character: RwSignal<CharacterDetails>) {
    // Make sure we can actually correctly access local storage
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            character.with(|char| {
                // Serialize the character to json
                if let Ok(json) = serde_json::to_string(char) {
                    // Store the json
                    let _ = local_storage.set_item(CHAR_STORAGE_KEY, &json);
                }
            })
        }
    }
}

fn load_selected_optional_features() -> Vec<FeatureOptionsSelection> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(data)) = local_storage.get_item(OPTIONS_STORAGE_KEY)
            {
                if let Ok(options) =
                    serde_json::from_str::<Vec<FeatureOptionsSelection>>(&data)
                {
                    return options;
                }
            }
        }
    }
    vec![]
}

fn write_optional_features_to_local_storage(
    options: RwSignal<Vec<FeatureOptionsSelection>>,
) {
    // Make sure we can actually correctly access local storage
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            options.with(|options| {
                // Serialize the character to json
                if let Ok(json) = serde_json::to_string(options) {
                    // Store the json
                    let _ = local_storage.set_item(OPTIONS_STORAGE_KEY, &json);
                }
            })
        }
    }
}

#[derive(Clone, Copy)]
pub struct FeaturesWrapper {
    pub all: Signal<Vec<Feature>>,
    pub optional_features: RwSignal<Vec<Feature>>,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create reactive signal to store character state
    let character = create_rw_signal(cx, load_character());

    // Update local storage whenever the character details change
    create_effect(cx, move |_| write_character_to_local_storage(character));

    // Create wrapper for async access to data from Open5e
    let api_data = FuturesWrapper::new(cx);

    let species_slice =
        create_read_slice(cx, character, |c| c.species.to_string());
    let subspecies_slice =
        create_read_slice(cx, character, |c| c.subspecies.to_string());
    let class_slice = create_read_slice(cx, character, |c| c.class.to_string());
    let background_slice =
        create_read_slice(cx, character, |c| c.background.to_string());
    let level = create_read_slice(cx, character, CharacterDetails::level);

    // Closure to reactively get the API definition of the current species.
    let current_species = Signal::derive(cx, move || {
        let species = species_slice();
        if let Some(species_list) = api_data.species.read(cx) {
            species_list.iter().find(|s| s.slug == species).cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current
    // subspecies. Only returns a result if the current subspecies is
    // a subspecies of the current species.
    let current_subspecies = Signal::derive(cx, move || {
        let subspecies = subspecies_slice();
        if let Some(species) = current_species() {
            species
                .subraces
                .iter()
                .find(|s| s.slug == subspecies)
                .cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current class.
    let current_class = Signal::derive(cx, move || {
        let class = class_slice();
        if let Some(class_list) = api_data.classes.read(cx) {
            class_list.iter().find(|s| s.slug == class).cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current background.
    let current_background = Signal::derive(cx, move || {
        let background = background_slice();
        if let Some(background_list) = api_data.backgrounds.read(cx) {
            background_list
                .iter()
                .find(|s| s.slug == background)
                .cloned()
        } else {
            None
        }
    });

    let features_base = Signal::derive(cx, move || {
        let mut features_out: Vec<Feature> = vec![];

        // These are currently the only character properties
        // that can supply features, so they're the only ones
        // we need to listen too for now.
        // Subclass should be added later.

        // Species
        if let Some(species_def) = current_species() {
            features_out.append(&mut species_def.features());
        }

        // Subspecies
        if let Some(subspecies_def) = current_subspecies() {
            let f = &mut subspecies_def.features();
            features_out.append(f);
        }

        // Class
        if let Some(class) = current_class() {
            features_out.append(&mut class.features());
        }

        // Background
        if let Some(background) = current_background() {
            features_out.append(&mut background.features());
        }

        features_out
            .iter()
            .filter(|f| f.level <= level())
            .cloned()
            .collect::<Vec<Feature>>()
    });

    let optional_features = Signal::derive(cx, move || {
        features_base()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Option(op) = &f.feature_type {
                    Some((f.feature_slug(), op.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, FeatureOptions)>>()
    });

    // TODO: Fix bug where selected optional features for a class are
    // retained if you change classes?
    let selected_otional_features: RwSignal<Vec<FeatureOptionsSelection>> =
        create_rw_signal(cx, load_selected_optional_features());

    // Update local storage when the selected optional features change
    create_effect(cx, move |_| {
        write_optional_features_to_local_storage(selected_otional_features)
    });

    let current_features = Signal::derive(cx, move || {
        let mut features_out: Vec<Feature> = features_base();

        selected_otional_features.with(|selected| {
            for select in selected {
                let op_features = optional_features();
                let option = op_features.iter().find_map(|(slug, op_feat)| {
                    if select.slug.contains(&slug.to_string()) {
                        Some(op_feat)
                    } else {
                        None
                    }
                });
                // We found the option that corresponds to this selection
                if let Some(options) = option {
                    if let Some(selection) =
                        options.options.get(select.selection)
                    {
                        features_out.push(selection.to_owned());
                    }
                }
            }
        });
        features_out
    });

    let current_asis = Signal::derive(cx, move || {
        current_features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Asi(asi) = &f.feature_type {
                    Some(asi)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<CharacterAsi>>()
    });

    let ability_scores = AbilityScoresReactive {
        ability_scores: create_read_slice(cx, character, |c| {
            c.ability_scores.clone()
        }),
        asis: current_asis,
    };

    let _proficiencies = Signal::derive(cx, move || {
        current_features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Proficiency(prof) = &f.feature_type {
                    Some(prof)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<String>>()
    });

    let proficency_bonus =
        create_read_slice(cx, character, CharacterDetails::prof_bonus);

    let subspecies_signals = create_slice(
        cx,
        character,
        |c| c.subspecies.to_string(),
        |c, v| c.subspecies = v,
    );

    // ==============
    // RENDER
    // ==============
    vec![
        HeaderPanel(cx, character, api_data),
        // Stats row
        div(cx).classes("container").child(StatsPanel(
            cx,
            character,
            proficency_bonus,
            ability_scores,
        )),
        div(cx).attr("class", "container").child(
            GridRow(cx)
                // Left column
                .child(LeftColumn(
                    cx,
                    current_features,
                    proficency_bonus,
                    ability_scores,
                ))
                // Center column
                .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                // Right column
                .child(RightColumn(
                    cx,
                    selected_otional_features,
                    features_base,
                    current_species,
                    subspecies_signals,
                )),
        ),
        // OptionSelectionModal(cx),
    ]
}

type OptionList = Vec<HtmlElement<Option_>>;
type DivList = Vec<HtmlElement<Div>>;

/*====================================
 *
 *  LEFT COLUMN
 *
 *===================================*/

pub fn LeftColumn(
    cx: Scope,
    features: Signal<Vec<Feature>>,
    proficiency_bonus: Signal<i32>,
    ability_scores: AbilityScoresReactive,
) -> HtmlElement<Div> {
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
                if let FeatureType::Proficiency(prof) = &f.feature_type {
                    Some(prof)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<String>>()
    });
    GridCol(cx).child(
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
                                SavesDisplay(
                                    cx,
                                    saves,
                                    proficiency_bonus,
                                    ability_scores,
                                ),
                            ),
                            TabPanel(
                                cx,
                                "skills-tab",
                                false,
                                SkillsDisplay(cx, skills),
                            ),
                            //TabPanel(cx, "other-tab", false, background_tab),
                        ]),
                ),
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

pub fn SavesDisplay(
    cx: Scope,
    saves: Signal<Vec<Ability>>,
    proficiency_bonus: Signal<i32>,
    ability_scores: AbilityScoresReactive,
) -> HtmlElement<Div> {
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
                                proficiency_bonus,
                            )
                        })),
                )
            })
            .collect::<Vec<HtmlElement<Li>>>(),
        ),
    )
}

pub fn SkillsDisplay(
    cx: Scope,
    skills: Signal<Vec<String>>,
) -> HtmlElement<Div> {
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

/*====================================
 *
 *  RIGHT COLUMN
 *
 *===================================*/

pub fn RightColumn(
    cx: Scope,
    selected_optional_features: RwSignal<Vec<FeatureOptionsSelection>>,
    features: Signal<Vec<Feature>>,
    current_species: Signal<Option<Species>>,
    subspecies_signals: (Signal<String>, SignalSetter<String>),
) -> HtmlElement<Div> {
    GridCol(cx).child(
        ScrollableContainerBox(cx)
            .child(h1(cx).child("Features:"))
            .child(FeaturePanel(
                cx,
                ClassTab(cx, selected_optional_features, features),
                SpeciesTab(cx, subspecies_signals, current_species),
                BackgroundTab(cx, features),
            )),
    )
}
