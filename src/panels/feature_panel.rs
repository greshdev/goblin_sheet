#![allow(non_snake_case)]

use crate::api::api_extensions::*;

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

use crate::set_subspecies;

use leptos::{html::*, *};

pub fn FeaturePanel(
    class_tab: HtmlDiv,
    species_tab: HtmlDiv,
    background_tab: HtmlDiv,
) -> HtmlDiv {
    div()
        .child(
            ul().classes("nav nav-tabs mb-3")
                .id("featuresTabs")
                .attr("role", "tablist")
                .child(vec![
                    Tab("class-tab", true, "Class"),
                    Tab("species-tab", false, "Species"),
                    Tab("background-tab", false, "Background"),
                ]),
        )
        .child(
            ul().style("padding-left", "0rem").child(
                div()
                    .classes("tab-content")
                    .id("featuresTabsContent")
                    .child(vec![
                        TabPanel("class-tab", true, class_tab),
                        TabPanel("species-tab", false, species_tab),
                        TabPanel("background-tab", false, background_tab),
                    ]),
            ),
        )
}

/// Tab of the feature menu that renders the
/// features from the character's class
pub fn ClassTab() -> HtmlDiv {
    let features = get_current_features();
    let filter = |f: &Feature| {
        f.source_slug.split(':').next() == Some("class") && !f.hidden
    };
    let feature_list = move || {
        features()
            .into_iter()
            .filter(filter)
            .map(FeatureDiv)
            .collect::<DivList>()
    };
    div().child(
        div()
            .classes("accordion")
            .id("featuresAccordion")
            .child(feature_list),
    )
}

fn FeatureDiv(f: Feature) -> HtmlDiv {
    let feature_display = match &f.feature_type {
        FeatureType::Option(feature_op) => {
            RenderOptionFeature(feature_op, &f.desc, &f.feature_slug())
        }
        _ => div().inner_html(parse_markdown_table(&f.desc)),
    };
    AccordionItem(
        div().child(format!("{} (Level {})", f.name, f.level)),
        div().child(feature_display),
    )
}

fn RenderOptionFeature(
    feature_op: &FeatureOptions,
    f_desc: &String,
    f_slug: &String,
) -> HtmlDiv {
    let num_choices = feature_op.num_choices;
    let options = &feature_op.options;
    let generate_dropdown = |i: i32| {
        // We need this slug to represent BOTH
        // the feature this selection came from,
        // as well as WHICH option box it was
        // selected in.
        let slug = format!("{}:{}", f_slug, i);
        FeatureOptionDropdown(slug, options)
    };
    let dropdowns = (0..num_choices)
        .map(generate_dropdown)
        .collect::<Vec<HtmlElement<Select>>>();
    div().child(f_desc).child(dropdowns)
}

fn FeatureOptionDropdown(
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
        expect_context::<RwSignal<Vec<FeatureOptionsSelection>>>();
    selected_optional_features.with_untracked(move |selected| {
        if let Some(thing) = selected.iter().find(matches_slug) {
            *selected_index_ptr = thing.selection;
        }
    });
    CustomSelect()
        .child(option().child("Select...").attr("value", 99))
        .child(
            options
                .iter()
                // Enumerate so we can get the index
                // of each item.
                .enumerate()
                .map(|i| SelectFeatureOption(i, selected_index))
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
    let feature_option_slug = slug;
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
    (i, op): (usize, &Feature),
    selected_index: usize,
) -> HtmlElement<Option_> {
    let out = match &op.feature_type {
        FeatureType::Asi(asi) => SelectFeatureOptionAsi(asi),
        FeatureType::SkillProficency(prof) => {
            SelectFeatureOptionProficiency(prof)
        }
        FeatureType::SavingThrow(ab) => SelectFeatureOptionSave(ab),
        _ => option(),
    };
    out.prop("value", i).prop("selected", i == selected_index)
}

fn SelectFeatureOptionAsi(asi: &CharacterAsi) -> HtmlElement<Option_> {
    let asi_name = asi.score.to_string();
    option().child(asi_name.to_string())
}
fn SelectFeatureOptionProficiency(prof: &str) -> HtmlElement<Option_> {
    option().child(prof.to_owned())
}
fn SelectFeatureOptionSave(ability: &Ability) -> HtmlElement<Option_> {
    let ability_name = ability.to_string().to_owned();
    option().child(ability_name)
}

/* pub fn DisplayClassFeatures(

    class: &Class,
    level: Signal<i32>,
) -> HtmlDiv {
    let class = class.clone();
    div()
        .classes("accordion")
        .id("featuresAccordion")
        .child(move || {
            class
                .features()
                .iter()
                .filter(|f| f.level <= level())
                .cloned()
                .map(|f| FeatureItem(f))
                .collect::<DivList>()
        })
} */

/// Tab of the feature menu that renders the
/// features from the character's species and subspecies
pub fn SpeciesTab() -> HtmlDiv {
    div().child(move || {
        if let Some(s) = get_current_species()() {
            SpeciesDisplay(s)
        } else {
            div()
        }
    })
}

fn SpeciesDisplay(species: Species) -> HtmlDiv {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    let get_subspecies =
        create_read_slice(character, |c| c.subspecies.to_string());
    //let subspecies_list = species.subraces.clone();
    let mut features = species.features();
    let my_subspecies =
        species.subraces.iter().find(|s| s.slug == get_subspecies());
    if let Some(subspecies) = my_subspecies {
        features.append(&mut subspecies.features());
    }
    let dropdown_maybe = if !species.subraces.is_empty() {
        div().child(SubspeciesDropdown(species.subraces))
    } else {
        div()
    };

    let features_div = if !features.is_empty() {
        div().classes("accordion").child(
            features
                .iter()
                .filter(|f| !f.hidden)
                .map(|f| {
                    AccordionItem(
                        div().child(&f.name),
                        div().inner_html(parse_markdown_table(&f.desc)),
                    )
                })
                .collect::<DivList>(),
        )
    } else {
        div()
    };

    div().child(dropdown_maybe).child(features_div)
}

fn SubspeciesDropdown(subspecies: Vec<Subspecies>) -> impl IntoView {
    let options = subspecies
        .iter()
        .map(|ss| {
            // TODO: Look at whether this can be refactored
            OptionWithDocTitle(
                &get_subspecies()(),
                &ss.slug,
                &ss.name,
                &ss.document_title,
            )
        })
        .collect::<OptionList>();
    CustomSelect()
        .classes("form-select mb-3")
        .prop("value", get_subspecies())
        .on(ev::change, move |e| {
            set_subspecies()(event_target_value(&e))
        })
        .child(option().prop("value", "").child("Select a subspecies..."))
        .child(options)
}

/// Tab of the feature menu that renders the
/// features from the character's background.
pub fn BackgroundTab() -> HtmlDiv {
    let features = get_current_features();
    div().classes("accordion").child(move || {
        features()
            .iter()
            .filter(|f| {
                !f.hidden
                    && f.source_slug.split(':').next() == Some("background")
            })
            .map(|f| {
                AccordionItem(
                    div().child(&f.name),
                    div().inner_html(parse_markdown_table(&f.desc)),
                )
            })
            .collect::<DivList>()
    })
}

/*
fn FeatureOptionsList( options: &[Feature]) -> OptionList {
    options
        .clone()
        .iter()
        .map(|op| match &op.feature_type {
            FeatureType::Asi(_) => option(),
            FeatureType::Proficiency(prof) => option().child(prof.clone()),
            FeatureType::SavingThrow(_) => option(),
            _ => option(),
        })
        .collect::<OptionList>()
}

pub fn FeatureItem( f: Feature) -> HtmlDiv {
    let f_desc = f.desc.to_string();
    let feature_display = match f.feature_type.clone() {
        FeatureType::Option(o) => {
            let options = o.clone().options;
            div().child(f_desc).child(
                (0..o.num_choices)
                    .map(|_| {
                        CustomSelect().child(FeatureOptionsList(&options))
                    })
                    .collect::<Vec<HtmlElement<Select>>>(),
            )
        }
        _ => div().inner_html(parse_markdown_table(&f.desc)),
    };
    AccordionItem(
        cx,
        div().child(format!("{} (Level {})", f.name.clone(), f.level)),
        div().child(feature_display),
    )
}
 */
