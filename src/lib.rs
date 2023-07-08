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
                leptos::log!("Could not deserialize data from Open5e to the SpeciesAPI struct!");
                leptos::log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            leptos::log!("Error fetching species data from Open5e!");
            leptos::log!("{}", e);
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
                leptos::log!("Could not deserialize data from Open5e to the ClassAPI struct!");
                leptos::log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            leptos::log!("Error fetching class data from Open5e!");
            leptos::log!("{}", e);
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
            Ok(api) => api.results,
            // Handle deserialization error condition
            Err(e) => {
                leptos::log!("Could not deserialize data from Open5e to the BackgroundAPI struct!");
                leptos::log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            leptos::log!("Error fetching background data from Open5e!");
            leptos::log!("{}", e);
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

    let features_panel = ScrollableContainerBox(cx)
        .child(h1(cx).child("Features:"))
        .child(FeaturePanel(cx, class_tab, species_tab));

    // Render the page
    vec![
        Header(
            cx,
            character,
            species_future,
            class_future,
            backgrounds_future,
        ),
        div(cx)
            .attr("class", "container")
            // Left column
            .child(
                GridRow(cx)
                    .child(GridCol(cx).child("Column One"))
                    // Center column
                    .child(GridCol(cx).child("Column Two"))
                    // Right column
                    .child(GridCol(cx).child(features_panel)),
            ),
    ]
}

fn Header(
    cx: Scope,
    character: RwSignal<CharacterDetails>,
    species_future: Resource<(), Vec<Species>>,
    class_future: Resource<(), Vec<Class>>,
    backgrounds_future: Resource<(), Vec<Background>>,
) -> HtmlElement<Div> {
    let (species, set_species) = create_slice(
        cx,
        character,
        |c| c.species.to_string(),
        |c, n| {
            c.species = n;
            // Clear the subspecies when the species changes
            c.subspecies = String::new();
        },
    );
    let (class, set_class) = create_slice(
        cx,
        character,
        |c| c.class.to_string(),
        |c, v| c.class = v,
    );
    let (background, set_background) = create_slice(
        cx,
        character,
        |c| c.background.to_string(),
        |c, v| c.background = v,
    );

    let (name, set_name) =
        create_slice(cx, character, |c| c.name.to_string(), |c, n| c.name = n);

    let (level, set_level) = create_slice(
        cx,
        character,
        CharacterDetails::level,
        CharacterDetails::set_level,
    );

    div(cx)
        .classes("container border m-3 pt-2 text-center")
        .child(
        GridRow(cx)
            .classes("row container gx-5")
            .child(
                div(cx).classes("col d-flex align-items-center").child(
                    input(cx)
                        .classes("form-control")
                        .attr("placeholder", "Character Name")
                        .on(ev::input, move |e| {
                            set_name(event_target_value(&e))
                        })
                        .prop("value", name),
                ),
            )
            .child(
                GridCol(cx)
                    .child(GridRow(cx).child(ClassDropdown(
                        cx,
                        class_future,
                        class,
                        set_class,
                    )))
                    .child(GridRow(cx).child(SpeciesDropdown(
                        cx,
                        species_future,
                        species,
                        set_species,
                    ))),
            )
            .child(
                GridCol(cx)
                    //.attr("class", "col-sm-3")
                    .child(
                        GridRow(cx).child(
                            div(cx)
                                .classes("input-group")
                                .child(
                                    div(cx)
                                        .classes("input-group-text")
                                        .child("Level:"),
                                )
                                .child(
                                    CustomSelect(cx)
                                        .prop("value", level)
                                        .on(ev::input, move |e| {
                                            let event_val =
                                                event_target_value(&e);
                                            if let Ok(num) =
                                                str::parse::<i32>(&event_val)
                                            {
                                                set_level(num);
                                            }
                                        })
                                        .child(
                                            (1..=20)
                                                .into_iter()
                                                .map(|i| {
                                                    option(cx)
                                                        .prop("value", i)
                                                        .prop(
                                                            "selected",
                                                            i == level.get(),
                                                        )
                                                        .child(i.to_string())
                                                })
                                                .collect::<OptionList>(),
                                        ),
                                ),
                        ),
                    )
                    .child(GridRow(cx).child(div(cx).child(
                        BackgroundDropdown(
                            cx,
                            backgrounds_future,
                            background,
                            set_background,
                        ),
                    ))),
            ),
    )
}

type OptionList = Vec<HtmlElement<Option_>>;

fn SpeciesDropdown(
    cx: Scope,
    future: Resource<(), Vec<Species>>,
    species: Signal<String>,
    set_species: SignalSetter<String>,
) -> impl IntoView {
    CustomSelect(cx)
        //.classes("mb-3")
        .prop("value", species)
        .on(ev::change, move |e| set_species(event_target_value(&e)))
        .attr("placeholder", "Species")
        .child(option(cx).prop("value", "").child("Select a species..."))
        .child(move || {
            future
                .with(cx, |species_list| {
                    species_list
                        .iter()
                        .map(|s| {
                            SpeciesOption(cx, s)
                                .prop("selected", s.slug == species())
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading....")])
        })
}

fn SpeciesOption(cx: Scope, species: &Species) -> HtmlElement<Option_> {
    option(cx)
        .prop("value", species.slug.clone())
        .child(format!("{} ({})", species.name, species.document_title))
}

fn ClassDropdown(
    cx: Scope,
    future: Resource<(), Vec<Class>>,
    class: Signal<String>,
    set_class: SignalSetter<String>,
) -> impl IntoView {
    CustomSelect(cx)
        .prop("value", class)
        .on(ev::change, move |e| set_class(event_target_value(&e)))
        .child(option(cx).child("Select a class...").prop("value", ""))
        .child(move || {
            future
                .with(cx, |classes| {
                    classes
                        .iter()
                        .map(|c| {
                            option(cx)
                                .prop("value", c.slug.clone())
                                .prop("selected", c.slug == class.get())
                                .child(c.name.clone())
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading...")])
        })
}

fn BackgroundDropdown(
    cx: Scope,
    future: Resource<(), Vec<Background>>,
    background: Signal<String>,
    set_background: SignalSetter<String>,
) -> HtmlElement<Select> {
    CustomSelect(cx)
        .prop("value", background)
        .on(ev::change, move |e| set_background(event_target_value(&e)))
        .child(option(cx).child("Select a background...").prop("value", ""))
        .child(move || {
            future
                .with(cx, |bg| {
                    bg.iter()
                        .map(|c| {
                            option(cx)
                                .prop("value", c.slug.clone())
                                .prop("selected", c.slug == background.get())
                                .child(format!(
                                    "{} ({})",
                                    c.name, c.document_title
                                ))
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading...")])
        })
}

fn FeaturePanel(
    cx: Scope,
    class_tab: HtmlElement<Div>,
    species_tab: HtmlElement<Div>,
) -> HtmlElement<Div> {
    div(cx)
        .child(
            ul(cx)
                .classes("nav nav-tabs mb-3")
                .id("featuresTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab(cx, "class-tab", true, "Class"),
                    Tab(cx, "species-tab", false, "Class"),
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
    //let mut desc = format!(
    //    "{}\n{}",
    //    parse_markdown(&species.desc),
    //    parse_markdown_table(&species.traits)
    //);
    //for feat in species.features() {
    //    desc += parse_markdown_table(feat.)
    //}
    let subspecies = species.subraces.clone();
    let subspecies_2 = species.subraces.clone();
    let dropdown_maybe = if subspecies.len() > 0 {
        div(cx)
            .child(SubspeciesDropdown(
                cx,
                subspecies,
                get_subspecies,
                set_subspecies,
            ))
            .child(move || {
                if get_subspecies.get() != "" {
                    subspecies_2
                        .iter()
                        .find(|s| s.slug == get_subspecies.get())
                        .map(|s: &Subspecies| {
                            div(cx).inner_html(format!(
                                "{}\n{}",
                                parse_markdown(&s.desc),
                                parse_markdown(&s.traits)
                            ))
                        })
                        .unwrap_or(div(cx))
                } else {
                    div(cx)
                }
            })
    } else {
        div(cx)
    };
    div(cx)
        .classes("accordion")
        .child(
            species
                .features()
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
        .child(dropdown_maybe)
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

pub fn parse_markdown(markdown: &str) -> String {
    markdown_to_html(markdown, &ComrakOptions::default())
}
pub fn parse_markdown_table(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    markdown_to_html(markdown, &options)
}
