#![allow(non_snake_case)]

use crate::api::api_extensions::Feature;
use crate::api::api_extensions::FeatureOptionsSelection;
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
pub fn ClassTab(
    cx: Scope,
    selected_optional_features: RwSignal<Vec<FeatureOptionsSelection>>,
    features: Signal<Vec<Feature>>,
) -> HtmlElement<Div> {
    div(cx).child(div(cx).classes("accordion").id("featuresAccordion").child(
        move || {
            features()
                .iter()
                .filter(|f| {
                    f.source_slug.split(':').next() == Some("class")
                        && !f.hidden
                })
                .cloned()
                .map(|f| {
                    let f_desc = f.desc.to_string();
                    let feature_display = match f.feature_type.clone() {
                        FeatureType::Option(o) => {
                            let options = o.clone().options;
                            div(cx).child(f_desc).child(
                                (0..o.num_choices)
                                    .map(|index1| {
                                        // We need this slug to represent BOTH
                                        // the feature this selection came from,
                                        // as well as WHICH option box it was
                                        // selected in.
                                        let slug = format!("{}:{}", f.feature_slug(), index1) ;
                                        let slug_2 = slug.clone();
                                        let mut selected_index = 0;
                                        let selected_index_ptr = &mut selected_index;
                                        // Don't track here, because we don't want this
                                        // element to refresh when we change our
                                        // selection.
                                        selected_optional_features.with_untracked(
                                            move |selected| 
                                            if let Some(thing) = selected.iter().find(|f| f.slug == slug_2) {
                                                *selected_index_ptr = thing.selection;
                                            }
                                        );
                                        CustomSelect(cx)
                                            .child(
                                                options
                                                .clone()
                                                .iter()
                                                // Enumerate so we can get the index
                                                // of each item.
                                                .enumerate()
                                                .map(|(i, op)| match &op.feature_type {
                                                    FeatureType::Asi(_) => option(cx),
                                                    FeatureType::Proficiency(prof) => 
                                                        option(cx)
                                                            .prop("value", i)
                                                            .prop("selected", i == selected_index)
                                                            .child(prof.clone()),
                                                    FeatureType::SavingThrow(_) => option(cx),
                                                    _ => option(cx),
                                                })
                                                .collect::<OptionList>())
                                            .on(ev::change, move |e| {
                                                let feature_option_slug = slug.clone();
                                                let val =
                                                    event_target_value(&e);
                                                if let Ok(index) = str::parse::<usize>(&val) {
                                                    selected_optional_features.update(|selected| {
                                                        selected.retain(|s| s.slug != feature_option_slug);
                                                        selected.push(FeatureOptionsSelection { 
                                                            slug: feature_option_slug, 
                                                            selection: index }
                                                        )
                                                    });
                                                }
                                            })
                                    })
                                    .collect::<Vec<HtmlElement<Select>>>(),
                            )
                        }
                        _ => div(cx).inner_html(parse_markdown_table(&f.desc)),
                    };
                    AccordionItem(
                        cx,
                        div(cx).child(format!(
                            "{} (Level {})",
                            f.name.clone(),
                            f.level
                        )),
                        div(cx).child(feature_display),
                    )
                })
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
                .cloned()
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
            .filter(|f| 
                !f.hidden && 
                f.source_slug.split(':').next() == Some("background")
            )
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

fn FeatureOptionsList(cx: Scope, options: &[Feature]) -> OptionList {
    options
        .clone()
        .iter()
        .map(|op| match &op.feature_type {
            FeatureType::Asi(_) => option(cx),
            FeatureType::Proficiency(prof) => option(cx).child(prof.clone()),
            FeatureType::SavingThrow(_) => option(cx),
            _ => option(cx),
        })
        .collect::<OptionList>()
}

pub fn FeatureItem(cx: Scope, f: Feature) -> HtmlElement<Div> {
    let f_desc = f.desc.to_string();
    let feature_display = match f.feature_type.clone() {
        FeatureType::Option(o) => {
            let options = o.clone().options;
            div(cx).child(f_desc).child(
                (0..o.num_choices)
                    .map(|_| {
                        CustomSelect(cx).child(FeatureOptionsList(cx, &options))
                    })
                    .collect::<Vec<HtmlElement<Select>>>(),
            )
        }
        _ => div(cx).inner_html(parse_markdown_table(&f.desc)),
    };
    AccordionItem(
        cx,
        div(cx).child(format!("{} (Level {})", f.name.clone(), f.level)),
        div(cx).child(feature_display),
    )
}
