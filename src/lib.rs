#![allow(non_snake_case)]

mod api;
mod character_model;
mod components;
mod header_panel;
mod markdown;

use crate::api::api_model::Class;
use crate::api::FuturesWrapper;
use crate::character_model::CharacterDetails;
use crate::components::*;
use crate::markdown::*;
use api::api_extensions::*;

use api::api_model::Background;
use api::api_model::Species;
use api::api_model::Subspecies;
use header_panel::Header;
use leptos::{component, IntoView, Scope};
use leptos::{ev, html::*, *};

const CHAR_STORAGE_KEY: &str = "character_sheet_character";

fn get_character() -> CharacterDetails {
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
    return CharacterDetails::new();
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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create reactive signal to store character state
    let character = create_rw_signal(cx, get_character());

    // Update local storage whenever the character details change
    create_effect(cx, move |_| write_character_to_local_storage(character));

    // Create wrapper for async access to data from Open5e
    let api_data = FuturesWrapper::new(cx);

    let species_slice =
        create_read_slice(cx, character, |c| c.species.to_string());
    let subspecies_slice =
        create_read_slice(cx, character, |c| c.subspecies.to_string());

    let current_species = move || {
        species_slice.with(|species_slug| {
            let slug = species_slug.clone();
            if let Some(species_list) = api_data.species.read(cx) {
                species_list.iter().find(|s| s.slug == slug).cloned()
            } else {
                None
            }
        })
    };

    //let computed_asis = move || {
    //    let species = species_slice.get();
    //    let subspecies = species_slice.get();
    //    let species_def =
    //};

    let background =
        create_read_slice(cx, character, |c| c.background.to_string());

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
        Header(cx, character, api_data),
        // Stats row
        div(cx).classes("container").child(StatsRow(cx, character)),
        div(cx)
            .attr("class", "container")
            // Left column
            .child(
                GridRow(cx)
                    .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                    // Center column
                    .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                    // Right column
                    .child(RightColumn(
                        cx,
                        api_data,
                        //character,
                        create_read_slice(
                            cx,
                            character,
                            CharacterDetails::level,
                        ),
                        create_read_slice(cx, character, |c| {
                            c.class.to_string()
                        }),
                        create_read_slice(cx, character, |c| {
                            c.species.to_string()
                        }),
                        create_read_slice(cx, character, |c| {
                            c.background.to_string()
                        }),
                        subspecies_signals,
                    )),
            ),
    ]
}

type OptionList = Vec<HtmlElement<Option_>>;

pub fn StatsRow(
    cx: Scope,
    character: RwSignal<CharacterDetails>,
) -> HtmlElement<Div> {
    HorizontalPanel(cx).child(
        GridRow(cx)
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Strength",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.str_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_str,
                    |c, v| c.ability_scores.base_str = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.str_mod()
                }),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Dexterity",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.dex_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_dex,
                    |c, v| c.ability_scores.base_dex = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.dex_mod()
                }),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Constitution",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.con_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_con,
                    |c, v| c.ability_scores.base_con = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.con_mod()
                }),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Wisdom",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.wis_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_wis,
                    |c, v| c.ability_scores.base_wis = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.wis_mod()
                }),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Intelligence",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.int_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_int,
                    |c, v| c.ability_scores.base_int = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.int_mod()
                }),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Charisma",
                create_read_slice(cx, character, |c| {
                    c.ability_scores.cha_score()
                }),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_cha,
                    |c, v| c.ability_scores.base_cha = v,
                ),
                create_read_slice(cx, character, |c| {
                    c.ability_scores.cha_mod()
                }),
            ))),
    )
}

fn AbilityScoreBox(
    cx: Scope,
    score_name: &str,
    score: Signal<i32>,
    (score_base, set_score_base): (Signal<i32>, SignalSetter<i32>),
    score_mod: Signal<i32>,
) -> HtmlElement<Div> {
    let (edit_mode, set_edit_mode) = create_signal(cx, false);
    let display_score =
        move || if edit_mode() { score_base() } else { score() };
    div(cx)
        .classes("d-flex flex-column")
        .child(score_name.to_string())
        .child(
            div(cx)
                .classes("border rounded text-centered mx-auto")
                .style("width", "5vw")
                .style("height", "5vw")
                .style("text-align", "center")
                .child(h2(cx).child(score_mod).classes("mt-1")),
        )
        .child(
            input(cx)
                //div(cx)
                .classes("border rounded mx-auto")
                .style("width", "2.5vw")
                .style("height", "2.5vw")
                .style("margin-top", "-1.5vw")
                .style("text-align", "center")
                //.classes("p-1")
                .style("background", "#212529")
                .prop("value", display_score)
                //.child(display_score)
                // When we "focus" on the input, switch to edit mode
                // and to show the "base" score
                .on(ev::focusin, move |_| set_edit_mode(true))
                // When we lose focus, switch back
                .on(ev::focusout, move |_| set_edit_mode(false))
                .on(ev::change, move |e| {
                    let val = event_target_value(&e);
                    if let Ok(num) = str::parse::<i32>(&val) {
                        set_score_base(num)
                    } else {
                        set_score_base(10)
                    }
                }),
        )
}

/*====================================
 *
 *  RIGHT COLUMN
 *
 *===================================*/

pub fn RightColumn(
    cx: Scope,
    api_data: FuturesWrapper,
    level: Signal<i32>,
    class: Signal<String>,
    species: Signal<String>,
    background: Signal<String>,
    subspecies_signals: (Signal<String>, SignalSetter<String>),
) -> HtmlElement<Div> {
    GridCol(cx).child(
        ScrollableContainerBox(cx)
            .child(h1(cx).child("Features:"))
            .child(FeaturePanel(
                cx,
                ClassTab(cx, api_data.classes, class, level),
                SpeciesTab(cx, species, subspecies_signals, api_data.species),
                BackgroundTab(cx, background, api_data.backgrounds),
            )),
    )
}

/// Tab of the feature menu that renders the
/// features from the character's class
pub fn ClassTab(
    cx: Scope,
    class_future: Resource<(), Vec<Class>>,
    class: Signal<String>,
    level: Signal<i32>,
) -> HtmlElement<Div> {
    div(cx).child(move || {
        class_future.with(cx, |classes| {
            let class_op = classes.iter().find(|c| c.slug == class.get());
            if let Some(class) = class_op {
                DisplayClassFeatures(cx, class, level)
            } else {
                div(cx)
            }
        })
    })
}

pub fn DisplayClassFeatures(
    cx: Scope,
    class: &Class,
    level: Signal<i32>,
) -> HtmlElement<Div> {
    let class = class.clone();
    div(cx)
        .classes("accordion")
        .id("featuresAccordion")
        .child(move || {
            class
                .features()
                .iter()
                .filter(|f| f.level <= level())
                .map(|f| FeatureItem(cx, f))
                .collect::<Vec<HtmlElement<Div>>>()
        })
}

/// Tab of the feature menu that renders the
/// features from the character's species and subspecies
pub fn SpeciesTab(
    cx: Scope,
    species: Signal<String>,
    (subspecies, set_subspecies): (Signal<String>, SignalSetter<String>),
    species_future: Resource<(), Vec<Species>>,
) -> HtmlElement<Div> {
    div(cx).child(move || {
        species_future.with(cx, |species_list| {
            species_list
                .iter()
                .find(|s| s.slug == species.get())
                .map(|s| {
                    SpeciesDisplay(cx, s.clone(), subspecies, set_subspecies)
                })
        })
    })
}

fn SpeciesDisplay(
    cx: Scope,
    species: Species,
    get_subspecies: Signal<String>,
    set_subspecies: SignalSetter<String>,
) -> HtmlElement<Div> {
    let subspecies_list = species.subraces.clone();
    let mut features = species.features();
    let my_subspecies =
        subspecies_list.iter().find(|s| s.slug == get_subspecies());
    if let Some(subspecies) = my_subspecies {
        features.append(&mut subspecies.features());
    }
    //let subspecies_list_2 = species.subraces.clone();
    let dropdown_maybe = if subspecies_list.len() > 0 {
        div(cx).child(SubspeciesDropdown(
            cx,
            subspecies_list,
            get_subspecies,
            set_subspecies,
        ))
    } else {
        div(cx)
    };

    let features_div = if features.len() > 0 {
        div(cx).classes("accordion").child(
            features
                .iter()
                .map(|f| {
                    AccordionItem(
                        cx,
                        div(cx).child(f.name.clone()),
                        div(cx).inner_html(parse_markdown_table(&f.desc)),
                    )
                })
                .collect::<Vec<HtmlElement<Div>>>(),
        )
    } else {
        div(cx)
    };

    div(cx).child(dropdown_maybe).child(features_div)
}

pub fn SubspeciesDropdown(
    cx: Scope,
    subspecies: Vec<Subspecies>,
    get_subspecies: Signal<String>,
    set_subspecies: SignalSetter<String>,
) -> impl IntoView {
    CustomSelect(cx)
        .classes("form-select mb-3")
        .prop("value", get_subspecies)
        .on(ev::change, move |e| set_subspecies(event_target_value(&e)))
        .child(option(cx).prop("value", "").child("Select a subspecies..."))
        .child(
            subspecies
                .iter()
                .map(|ss| {
                    OptionWithDocTitle(
                        cx,
                        &&get_subspecies(),
                        &ss.slug,
                        &ss.name,
                        &ss.document_title,
                    )
                })
                .collect::<OptionList>(),
        )
}

/// Tab of the feature menu that renders the
/// features from the character's background
pub fn BackgroundTab(
    cx: Scope,
    background: Signal<String>,
    backgrounds_future: Resource<(), Vec<Background>>,
) -> HtmlElement<Div> {
    div(cx).child(move || {
        backgrounds_future.with(cx, |backgrounds| {
            backgrounds
                .iter()
                .find(|b| b.slug == background())
                .map(|b| BackgroundDisplay(cx, b))
        })
    })
}

fn BackgroundDisplay(cx: Scope, background: &Background) -> HtmlElement<Div> {
    div(cx)
        .classes("accordion")
        .child(AccordionItem(
            cx,
            div(cx).child("Description"),
            div(cx).inner_html(parse_markdown_table(&background.desc)),
        ))
        .child(AccordionItem(
            cx,
            div(cx)
                .child(format!("Feature: {}", background.feature.to_string())),
            div(cx).child(background.feature_desc.to_string()),
        ))
        .child(AccordionItem(
            cx,
            div(cx).child(format!("Suggested Characteristics")),
            div(cx).child(div(cx).inner_html(parse_markdown_table(
                &background.suggested_characteristics,
            ))),
        ))
}

fn FeaturePanel(
    cx: Scope,
    class_tab: HtmlElement<Div>,
    species_tab: HtmlElement<Div>,
    background_tab: HtmlElement<Div>,
) -> HtmlElement<Div> {
    div(cx)
        .child(
            ul(cx)
                .classes("nav nav-tabs mb-3")
                .id("featuresTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab(cx, "class-tab", true, "Class"),
                    Tab(cx, "species-tab", false, "Species"),
                    Tab(cx, "background-tab", false, "Background"),
                ]),
        )
        .child(
            ul(cx).style("padding-left", "0rem").child(
                div(cx)
                    .classes("tab-content")
                    .id("featuresTabsContent")
                    .child(vec![
                        TabPanel(cx, "class-tab", true, class_tab),
                        TabPanel(cx, "species-tab", false, species_tab),
                        TabPanel(cx, "background-tab", false, background_tab),
                    ]),
            ),
        )
}

pub fn FeatureItem(cx: Scope, f: &Feature) -> HtmlElement<Div> {
    AccordionItem(
        cx,
        div(cx).child(format!("{} (Level {})", f.name.clone(), f.level)),
        div(cx).inner_html(parse_markdown_table(&f.desc)),
    )
}
