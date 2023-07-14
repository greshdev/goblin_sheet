#![allow(non_snake_case)]

use crate::api::api_extensions::Feature;
use crate::api::api_model::Class;
use crate::DivList;
use crate::OptionList;

use crate::api::api_model::Species;
use crate::api::api_model::Subspecies;
use crate::components::*;
use crate::markdown::*;

use crate::api::api_extensions::FeatureType;
use leptos::{ev, html::*, *};
use leptos::{IntoView, Scope};

/// Tab of the feature menu that renders the
/// features from the character's class
pub fn ClassTab(cx: Scope, features: Signal<Vec<Feature>>) -> HtmlElement<Div> {
    div(cx).child(div(cx).classes("accordion").id("featuresAccordion").child(
        move || {
            features()
                .iter()
                .filter(|f| {
                    f.source_slug.split(':').next() == Some("class")
                        && !f.hidden
                })
                .cloned()
                .map(|f| FeatureItem(cx, &f))
                .collect::<DivList>()
        },
    ))
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
                .collect::<DivList>()
        })
}

/// Tab of the feature menu that renders the
/// features from the character's species and subspecies
pub fn SpeciesTab(
    cx: Scope,
    // TODO: subspecies_features closure
    (subspecies, set_subspecies): (Signal<String>, SignalSetter<String>),
    current_species: Signal<Option<Species>>,
) -> HtmlElement<Div> {
    div(cx).child(move || {
        if let Some(s) = current_species() {
            SpeciesDisplay(cx, s, subspecies, set_subspecies)
        } else {
            div(cx)
        }
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
    let dropdown_maybe = if !subspecies_list.is_empty() {
        div(cx).child(SubspeciesDropdown(
            cx,
            subspecies_list,
            get_subspecies,
            set_subspecies,
        ))
    } else {
        div(cx)
    };

    let features_div = if !features.is_empty() {
        div(cx).classes("accordion").child(
            features
                .iter()
                .filter(|f| !f.hidden)
                .map(|f| {
                    AccordionItem(
                        cx,
                        div(cx).child(f.name.clone()),
                        div(cx).inner_html(parse_markdown_table(&f.desc)),
                    )
                })
                .collect::<DivList>(),
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
                    // TODO: Look at whether this can be refactored
                    OptionWithDocTitle(
                        cx,
                        &get_subspecies(),
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
    features: Signal<Vec<Feature>>,
) -> HtmlElement<Div> {
    div(cx).classes("accordion").child(move || {
        features()
            .iter()
            .filter(|f| f.source_slug.split(':').next() == Some("background"))
            .map(|f| {
                AccordionItem(
                    cx,
                    div(cx).child(&f.name),
                    div(cx).inner_html(parse_markdown_table(&f.desc)),
                )
            })
            .collect::<DivList>()
    })
}

pub fn FeaturePanel(
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
    let feature_display = match f.feature_type {
        FeatureType::Option(_) => todo!(),
        _ => div(cx).inner_html(parse_markdown_table(&f.desc)),
    };
    AccordionItem(
        cx,
        div(cx).child(format!("{} (Level {})", f.name.clone(), f.level)),
        feature_display,
    )
}
