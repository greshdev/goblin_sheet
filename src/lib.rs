#![allow(non_snake_case)]

use crate::character_model::CharacterDetails;
use crate::components::*;
use crate::dnd_api::{ClassesAPI, SpeciesAPI};
use comrak::{markdown_to_html, ComrakOptions};
use dnd_api::{Species, Subspecies};

use leptos::{component, create_local_resource, IntoView, Scope};
use leptos::{ev, html::*, *};
mod character_model;
mod components;
mod dnd_api;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    //let (name, set_name) = create_signal(cx, String::new());
    let character = create_rw_signal(cx, CharacterDetails::new());
    let (species, _) = create_slice(
        cx,
        character,
        |c| c.species.to_string(),
        |c, n| {
            c.species = n;
            // Clear the subspecies when the species changes
            c.subspecies = String::new();
        },
    );
    let (subspecies, set_subspecies) = create_slice(
        cx,
        character,
        |c| c.subspecies.to_string(),
        |c, v| c.subspecies = v,
    );

    let species_future = create_local_resource(
        cx,
        || (),
        |_| async move {
            let res = reqwasm::http::Request::get("https://api.open5e.com/v1/races/")
                .send()
                .await;
            match res {
                Ok(response) => match response.json::<SpeciesAPI>().await {
                    Ok(api) => api.results,
                    Err(_) => vec![],
                },
                Err(_) => vec![],
            }
        },
    );
    let class_future = create_local_resource(
        cx,
        || (),
        |_| async move {
            let res = reqwasm::http::Request::get("https://api.open5e.com/v1/classes/")
                .send()
                .await;
            match res {
                Ok(response) => match response.json::<ClassesAPI>().await {
                    Ok(api) => api.results,
                    Err(_) => vec![],
                },
                Err(_) => vec![],
            }
        },
    );

    // Render the page
    vec![
        Header(cx, character, species_future, class_future),
        div(cx)
            .attr("class", "container")
            // Left column
            .child(
                GridRow(cx)
                    .child(
                        GridCol(cx).child(
                            div(cx)
                                .style("height", "80vh")
                                .style("overflow-y", "auto")
                                .classes("container border p-2")
                                .child(h1(cx).child("Features:"))
                                .child(move || {
                                    species_future.with(cx, |species_list| {
                                        species_list
                                            .iter()
                                            .find(|s| s.slug == species.get())
                                            .map(|s| {
                                                SpeciesDisplay(
                                                    cx,
                                                    s.clone(),
                                                    subspecies,
                                                    set_subspecies,
                                                )
                                            })
                                            .unwrap_or(div(cx))
                                    })
                                }),
                        ),
                    )
                    // Center column
                    .child(GridCol(cx).child("Two"))
                    // Right column
                    .child(GridCol(cx).child("Three")),
            ),
    ]
}

fn Header(
    cx: Scope,
    character: RwSignal<CharacterDetails>,
    species_future: Resource<(), Vec<Species>>,
    class_future: Resource<(), Vec<crate::dnd_api::Class>>,
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
    let (class, set_class) =
        create_slice(cx, character, |c| c.class.to_string(), |c, v| c.class = v);

    let (name, set_name) = create_slice(cx, character, |c| c.name.to_string(), |c, n| c.name = n);
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
                            .on(ev::input, move |e| set_name(event_target_value(&e)))
                            .prop("value", name),
                    ),
                )
                .child(
                    GridCol(cx)
                        .child(GridRow(cx).child(ClassDropdown(cx, class_future, class, set_class)))
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
                        .child(GridRow(cx).child("Row 1"))
                        .child(GridRow(cx).child("Row 2")),
                )
                .child(
                    GridCol(cx)
                        //.attr("class", "col-sm-3")
                        .child(GridRow(cx).child("Row 1"))
                        .child(GridRow(cx).child("Row 2")),
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
        .classes("mb-3")
        .prop("value", species)
        .on(ev::change, move |e| set_species(event_target_value(&e)))
        .attr("placeholder", "Species")
        .child(option(cx).prop("value", "").child("Select a species..."))
        .child(move || {
            future
                .with(cx, |species| {
                    species
                        .iter()
                        .map(|s| SpeciesOption(cx, s))
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading....")])
        })
}

fn ClassDropdown(
    cx: Scope,
    future: Resource<(), Vec<crate::dnd_api::Class>>,
    class: Signal<String>,
    set_class: SignalSetter<String>,
) -> impl IntoView {
    CustomSelect(cx)
        .prop("value", class)
        .on(ev::change, move |e| set_class(event_target_value(&e)))
        .child(option(cx).child("Select a class..."))
        .child(move || {
            future
                .with(cx, |classes| {
                    classes
                        .iter()
                        .map(|c| {
                            option(cx)
                                .prop("value", c.slug.clone())
                                .child(c.name.clone())
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading...")])
        })
}

fn SpeciesOption(cx: Scope, species: &Species) -> HtmlElement<Option_> {
    option(cx)
        .prop("value", species.slug.clone())
        .child(format!("{} ({})", species.name, species.document_title))
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
        //.child(div(cx).inner_html(parse_markdown(&species.desc)))
        .child(
            species
                .features()
                .iter()
                .map(|f| {
                    div(cx)
                        .child(h3(cx).child(f.name.clone()))
                        .child(div(cx).inner_html(parse_markdown_table(&f.desc)))
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
                .map(|ss| SubspeciesOption(cx, ss))
                .collect::<OptionList>(),
        )
}
pub fn SubspeciesOption(cx: Scope, subspecies: &Subspecies) -> HtmlElement<Option_> {
    option(cx)
        .prop("value", subspecies.slug.clone())
        .child(format!(
            "{} ({})",
            subspecies.name, subspecies.document_title
        ))
}

pub fn parse_markdown(markdown: &str) -> String {
    markdown_to_html(markdown, &ComrakOptions::default())
}
pub fn parse_markdown_table(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    markdown_to_html(markdown, &options)
}
