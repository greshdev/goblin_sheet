#![allow(non_snake_case)]

use crate::character_model::CharacterDetails;
use crate::components::*;
use crate::dnd_api::{ClassesAPI, SpeciesAPI};
use comrak::{markdown_to_html, ComrakOptions};
use dnd_api::{
    Background, BackgroundsAPI, Class, Feature, Species, Subspecies,
};

use leptos::{component, create_local_resource, IntoView, Scope};
use leptos::{ev, html::*, *};
mod character_model;
mod components;
mod dnd_api;
mod header_panel;
use header_panel::Header;

/// Fetch list of species options from Open5e
async fn fetch_species(_: ()) -> Vec<Species> {
    let res = reqwasm::http::Request::get("https://api.open5e.com/v1/races/")
        .send()
        .await;
    match res {
        Ok(response) => match response.json::<SpeciesAPI>().await {
            Ok(api) => api.results,
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the SpeciesAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching species data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}

/// Fetch list of class options from Open5e
async fn fetch_classes(_: ()) -> Vec<Class> {
    let res = reqwasm::http::Request::get("https://api.open5e.com/v1/classes/")
        .send()
        .await;
    match res {
        Ok(response) => match response.json::<ClassesAPI>().await {
            Ok(api) => api.results,
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the ClassAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching class data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}

/// Fetch list of background options from Open5e
async fn fetch_backgrounds(_: ()) -> Vec<Background> {
    let res =
        reqwasm::http::Request::get("https://api.open5e.com/v1/backgrounds/")
            .send()
            .await;
    match res {
        Ok(response) => match response.json::<BackgroundsAPI>().await {
            Ok(api) => api
                .results
                .iter()
                .filter(|b| b.document_slug != "a5e")
                .cloned()
                .collect::<Vec<Background>>(),
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the BackgroundAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching background data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}

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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    //let (name, set_name) = create_signal(cx, String::new());
    let character = create_rw_signal(cx, get_character());

    // Update local storage whenever the character details change
    create_effect(cx, move |_| {
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
    });

    let background =
        create_read_slice(cx, character, |c| c.background.to_string());

    let (species, _) = create_slice(
        cx,
        character,
        |c| c.species.to_string(),
        CharacterDetails::change_species,
    );
    let (subspecies, set_subspecies) = create_slice(
        cx,
        character,
        |c| c.subspecies.to_string(),
        |c, v| c.subspecies = v,
    );

    let species_future = create_local_resource(cx, || (), fetch_species);
    let class_future = create_local_resource(cx, || (), fetch_classes);
    let backgrounds_future =
        create_local_resource(cx, || (), fetch_backgrounds);

    let class_tab = ClassDisplay(
        cx,
        class_future,
        create_read_slice(cx, character, |c| c.class.clone()),
        create_read_slice(cx, character, CharacterDetails::level),
    );

    // These should probably be refactored to be cleaner and more properly reactive
    let species_tab = div(cx).child(move || {
        species_future.with(cx, |species_list| {
            species_list
                .iter()
                .find(|s| s.slug == species.get())
                .map(|s| {
                    SpeciesDisplay(cx, s.clone(), subspecies, set_subspecies)
                })
        })
    });
    let background_tab = div(cx).child(move || {
        backgrounds_future.with(cx, |backgrounds| {
            backgrounds
                .iter()
                .find(|b| b.slug == background())
                .map(|b| BackgroundDisplay(cx, b))
        })
    });

    let features_panel = ScrollableContainerBox(cx)
        .child(h1(cx).child("Features:"))
        .child(FeaturePanel(cx, class_tab, species_tab, background_tab));

    // Render the page
    vec![
        Header(
            cx,
            character,
            species_future,
            class_future,
            backgrounds_future,
        ),
        // Send row
        div(cx).classes("container").child(
            HorizontalPanel(cx).child(
                GridRow(cx)
                    .child(GridCol(cx).child(AbilityScoreBox(
                        cx,
                        "Strength",
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
            ),
        ),
        div(cx)
            .attr("class", "container")
            // Left column
            .child(
                GridRow(cx)
                    .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                    // Center column
                    .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                    // Right column
                    .child(GridCol(cx).child(features_panel)),
            ),
    ]
}

type OptionList = Vec<HtmlElement<Option_>>;

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
                    SubspeciesOption(cx, ss)
                        .prop("selected", ss.slug == get_subspecies())
                })
                .collect::<OptionList>(),
        )
}
pub fn SubspeciesOption(
    cx: Scope,
    subspecies: &Subspecies,
) -> HtmlElement<Option_> {
    option(cx)
        .prop("value", subspecies.slug.clone())
        .child(format!(
            "{} ({})",
            subspecies.name, subspecies.document_title
        ))
}

pub fn ClassDisplay(
    cx: Scope,
    class_future: Resource<(), Vec<Class>>,
    class: Signal<String>,
    level: Signal<i32>,
) -> HtmlElement<Div> {
    div(cx).child(move || {
        class_future.with(cx, |classes| {
            classes
                .iter()
                .find(|c| c.slug == class.get())
                .map(|s| DisplayClassFeatures(cx, s, level))
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

pub fn FeatureItem(cx: Scope, f: &Feature) -> HtmlElement<Div> {
    AccordionItem(
        cx,
        div(cx).child(format!("{} (Level {})", f.name.clone(), f.level)),
        div(cx).inner_html(parse_markdown_table(&f.desc)),
    )
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

fn AbilityScoreBox(
    cx: Scope,
    score_name: &str,
    (score, set_score): (Signal<i32>, SignalSetter<i32>),
    score_mod: Signal<i32>,
) -> HtmlElement<Div> {
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
                .classes("border rounded text-centered mx-auto")
                .style("width", "2.5vw")
                .style("height", "2.5vw")
                .style("text-align", "center")
                .style("margin-top", "-1.5vw")
                .prop("value", score)
                .prop("min", 0)
                .prop("max", 30)
                .on(ev::change, move |e| {
                    let val = event_target_value(&e);
                    if let Ok(num) = str::parse::<i32>(&val) {
                        set_score(num)
                    } else {
                        set_score(10)
                    }
                }),
        )
}

pub fn parse_markdown(markdown: &str) -> String {
    markdown_to_html(markdown, &ComrakOptions::default())
}
pub fn parse_markdown_table(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    markdown_to_html(markdown, &options)
}
