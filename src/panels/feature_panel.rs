#![allow(non_snake_case)]

use crate::api::api_extensions::*;
use crate::DivList;
use crate::OptionList;

use crate::api::api_model::Species;
use crate::api::api_model::Subspecies;
use crate::character_model::Ability;
use crate::character_model::CharacterAsi;
use crate::character_model::CharacterDetails;
use crate::components::*;
use crate::get_current_features;
use crate::get_current_species;
use crate::get_subspecies;
use crate::markdown::*;

use crate::api::api_extensions::FeatureType;
use crate::set_subspecies;

use leptos::{ev, html::*, *};
use leptos::{IntoView, Scope};

pub fn FeaturePanel(
    cx: Scope,
    class_tab: HtmlDiv,
    species_tab: HtmlDiv,
    background_tab: HtmlDiv,
) -> HtmlDiv {
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

/// Tab of the feature menu that renders the
/// features from the character's class
pub fn ClassTab(cx: Scope) -> HtmlDiv {
    let features = get_current_features(cx);
    let filter = |f: &&Feature| {
        f.source_slug.split(':').next() == Some("class") && !f.hidden
    };
    let feature_list = move || {
        features()
            .iter()
            .filter(filter)
            .cloned()
            .map(|f| FeatureDiv(f, cx))
            .collect::<DivList>()
    };
    div(cx).child(
        div(cx)
            .classes("accordion")
            .id("featuresAccordion")
            .child(feature_list),
    )
}

fn FeatureDiv(f: Feature, cx: Scope) -> HtmlDiv {
    let f_desc = f.desc.to_string();
    let feature_display = match f.feature_type.clone() {
        FeatureType::Option(feature_op) => {
            RenderOptionFeature(cx, feature_op, f_desc, &f.feature_slug())
        }
        _ => div(cx).inner_html(parse_markdown_table(&f.desc)),
    };
    AccordionItem(
        cx,
        div(cx).child(format!("{} (Level {})", f.name.clone(), f.level)),
        div(cx).child(feature_display),
    )
}

fn RenderOptionFeature(
    cx: Scope,
    feature_op: FeatureOptions,
    f_desc: String,
    f_slug: &String,
) -> HtmlDiv {
    let num_choices = feature_op.num_choices;
    let options = feature_op.options;
    let generate_dropdown = |i: i32| {
        // We need this slug to represent BOTH
        // the feature this selection came from,
        // as well as WHICH option box it was
        // selected in.
        let slug = format!("{}:{}", f_slug, i);
        FeatureOptionDropdown(cx, slug, &options)
    };
    let dropdowns = (0..num_choices)
        .map(generate_dropdown)
        .collect::<Vec<HtmlElement<Select>>>();
    div(cx).child(f_desc).child(dropdowns)
}

fn FeatureOptionDropdown(
    cx: Scope,
    slug: String,
    options: &[Feature],
) -> HtmlElement<Select> {
    let matches_slug = |f: &&FeatureOptionsSelection| f.slug == slug;
    // Hack because I don't think any feature option will ever reach
    // 99 options lol.
    let mut selected_index = 99;
    let selected_index_ptr = &mut selected_index;
    // Don't track here, because we don't want this
    // element to refresh when we change our
    // selection.
    let selected_optional_features =
        expect_context::<RwSignal<Vec<FeatureOptionsSelection>>>(cx);
    selected_optional_features.with_untracked(move |selected| {
        if let Some(thing) = selected.iter().find(matches_slug) {
            *selected_index_ptr = thing.selection;
        }
    });
    CustomSelect(cx)
        .child(option(cx).child("Select...").attr("value", 99))
        .child(
            options
                .clone()
                .iter()
                // Enumerate so we can get the index
                // of each item.
                .enumerate()
                .map(|i| SelectFeatureOption(cx, i, selected_index))
                .collect::<OptionList>(),
        )
        .on(ev::change, move |event| {
            change_selected_feature(&slug, event, selected_optional_features);
        })
}

fn change_selected_feature(
    slug: &str,
    e: web_sys::Event,
    selected_optional_features: RwSignal<Vec<FeatureOptionsSelection>>,
) {
    let feature_option_slug = slug.clone();
    let val = event_target_value(&e);
    if let Ok(index) = str::parse::<usize>(&val) {
        selected_optional_features.update(|selected| {
            selected.retain(|s| s.slug != feature_option_slug);
            selected.push(FeatureOptionsSelection {
                slug: feature_option_slug.to_string(),
                selection: index,
            })
        });
    }
}

fn SelectFeatureOption(
    cx: Scope,
    (i, op): (usize, &Feature),
    selected_index: usize,
) -> HtmlElement<Option_> {
    let out = match &op.feature_type {
        FeatureType::Asi(asi) => SelectFeatureOptionAsi(cx, asi),
        FeatureType::SkillProficency(prof) => {
            SelectFeatureOptionProficiency(cx, prof)
        }
        FeatureType::SavingThrow(ab) => SelectFeatureOptionSave(cx, ab),
        _ => option(cx),
    };
    out.prop("value", i).prop("selected", i == selected_index)
}

fn SelectFeatureOptionAsi(
    cx: Scope,
    asi: &CharacterAsi,
) -> HtmlElement<Option_> {
    let asi_name = asi.score.to_string();
    option(cx).child(asi_name.to_string())
}
fn SelectFeatureOptionProficiency(
    cx: Scope,
    prof: &str,
) -> HtmlElement<Option_> {
    option(cx).child(prof.to_owned())
}
fn SelectFeatureOptionSave(
    cx: Scope,
    ability: &Ability,
) -> HtmlElement<Option_> {
    let ability_name = ability.to_string().to_owned();
    option(cx).child(ability_name)
}

/* pub fn DisplayClassFeatures(
    cx: Scope,
    class: &Class,
    level: Signal<i32>,
) -> HtmlDiv {
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
} */

/// Tab of the feature menu that renders the
/// features from the character's species and subspecies
pub fn SpeciesTab(cx: Scope) -> HtmlDiv {
    div(cx).child(move || {
        if let Some(s) = get_current_species(cx)() {
            SpeciesDisplay(cx, s)
        } else {
            div(cx)
        }
    })
}

fn SpeciesDisplay(cx: Scope, species: Species) -> HtmlDiv {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    let get_subspecies =
        create_read_slice(cx, character, |c| c.subspecies.to_string());
    let subspecies_list = species.subraces.clone();
    let mut features = species.features();
    let my_subspecies =
        subspecies_list.iter().find(|s| s.slug == get_subspecies());
    if let Some(subspecies) = my_subspecies {
        features.append(&mut subspecies.features());
    }
    let dropdown_maybe = if !subspecies_list.is_empty() {
        div(cx).child(SubspeciesDropdown(cx, subspecies_list))
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

fn SubspeciesDropdown(cx: Scope, subspecies: Vec<Subspecies>) -> impl IntoView {
    let options = subspecies
        .iter()
        .map(|ss| {
            // TODO: Look at whether this can be refactored
            OptionWithDocTitle(
                cx,
                &get_subspecies(cx)(),
                &ss.slug,
                &ss.name,
                &ss.document_title,
            )
        })
        .collect::<OptionList>();
    CustomSelect(cx)
        .classes("form-select mb-3")
        .prop("value", get_subspecies(cx))
        .on(ev::change, move |e| {
            set_subspecies(cx)(event_target_value(&e))
        })
        .child(option(cx).prop("value", "").child("Select a subspecies..."))
        .child(options)
}

/// Tab of the feature menu that renders the
/// features from the character's background.
pub fn BackgroundTab(cx: Scope) -> HtmlDiv {
    let features = get_current_features(cx);
    div(cx).classes("accordion").child(move || {
        features()
            .iter()
            .filter(|f| {
                !f.hidden
                    && f.source_slug.split(':').next() == Some("background")
            })
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

/*
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

pub fn FeatureItem(cx: Scope, f: Feature) -> HtmlDiv {
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
 */
