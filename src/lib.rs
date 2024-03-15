#![allow(non_snake_case)]

mod api;
mod character_model;
mod components;
mod dice;
mod markdown;
mod panels;

use crate::api::FuturesWrapper;
use crate::character_model::*;
use crate::components::*;
use api::api_extensions::*;
use api::api_model;
use api::api_model::Background;
use api::api_model::Species;
use api::api_model::Subspecies;
use panels::center_panel::CenterPanel;
use panels::feature_panel::*;
use panels::header_panel::HeaderPanel;
use panels::proficencies_panel::ProfPanel;
use panels::stats_panel::StatsPanel;

use leptos::{html::*, *};

const CHAR_STORAGE_KEY: &str = "char_sheet_character";
const OPTIONS_STORAGE_KEY: &str = "char_sheet_selected_optional_features";
const ATTACKS_STORAGE_KEY: &str = "char_sheet_attack_actions";

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
fn write_attack_list_to_local_storage(options: RwSignal<Vec<AttackAction>>) {
    // Make sure we can actually correctly access local storage
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            options.with(|options| {
                // Serialize the character to json
                if let Ok(json) = serde_json::to_string(options) {
                    // Store the json
                    let _ = local_storage.set_item(ATTACKS_STORAGE_KEY, &json);
                }
            })
        }
    }
}

fn load_attack_list() -> Vec<AttackAction> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(data)) = local_storage.get_item(ATTACKS_STORAGE_KEY)
            {
                if let Ok(options) =
                    serde_json::from_str::<Vec<AttackAction>>(&data)
                {
                    return options;
                }
            }
        }
    }
    vec![]
}

#[derive(Clone, Copy)]
pub struct FeaturesWrapper {
    pub all: Signal<Vec<Feature>>,
    pub optional_features: RwSignal<Vec<Feature>>,
}

pub fn App() -> impl IntoView {
    // Create reactive signal to store character state
    let character = create_rw_signal(load_character());
    // Store that state globally
    provide_context(character);

    // Update local storage whenever the character details change
    create_effect(move |_| write_character_to_local_storage(character));

    // Create wrapper for async access to data from Open5e
    // and store it globally
    provide_context(FuturesWrapper::new());

    // TODO: Fix bug where selected optional features for a class are
    // retained if you change classes?
    let selected_optional_features: RwSignal<Vec<FeatureOptionsSelection>> =
        create_rw_signal(load_selected_optional_features());
    provide_context(selected_optional_features);

    // Update local storage when the selected optional features change
    create_effect(move |_| {
        write_optional_features_to_local_storage(selected_optional_features)
    });

    provide_context(AbilityScoresReactive {
        ability_scores: create_read_slice(character, |c| {
            c.ability_scores.clone()
        }),
        asis: get_current_asis(),
    });

    let attack_list: RwSignal<Vec<AttackAction>> =
        create_rw_signal(load_attack_list());
    provide_context(attack_list);
    create_effect(move |_| write_attack_list_to_local_storage(attack_list));

    // ==============
    // RENDER
    // ==============
    vec![
        div()
            .id("dice-box")
            .classes("position-absolute top-0 start-0 w-100 h-100 pe-none"),
        div().classes("position-absolute top-0 start-0").child(
            a().child("Reset").on(ev::click, move |_| {
                // Reset all global data to default
                character.update(|c| *c = CharacterDetails::new());
                selected_optional_features.update(|s| *s = vec![]);
                attack_list.update(|a| *a = vec![])
            }),
        ),
        HeaderPanel(),
        // Stats row
        div().classes("container").child(StatsPanel()),
        div().attr("class", "container").child(
            GridRow()
                // Left column
                .child(GridCol().child(ProfPanel()))
                // Center column
                .child(CenterColumn())
                // Right column
                .child(RightColumn()),
        ),
        // OptionSelectionModal(),
        div()
            .classes("position-absolute bottom-0 end-0 p-3 z-2")
            .child(
                a().child(
                    img()
                        .attr("src", "github-mark-white.svg")
                        .attr("alt", "Github Logo")
                        .style("height", "5vh")
                        .style("width", "5vh"),
                )
                .attr("href", "https://github.com/greshdev/goblin_sheet"),
            ),
    ]
}

pub fn CenterColumn() -> HtmlDiv {
    GridCol()
        .child(
            div()
                .classes("container border rounded pt-2 mb-2")
                .child(GridRow().child([ProfBonusBox(), ACBox(), HPBox()])),
        )
        .child(
            BoxedColumnFlexible()
                .style("height", "49.5vh")
                .child(CenterPanel()),
        )
}

fn ProfBonusBox() -> HtmlElement<Div> {
    GridCol().child(
        div()
            .classes("d-flex flex-column align-items-center")
            .child("Proficiency")
            .child([
                div()
                .classes("border rounded my-auto d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div())
                .child(
                    h2()
                        .child(get_prof_bonus())
                        .style("margin-top", "-10%")),
                div().child("Bonus")]
            )
        )
}
fn HPBox() -> HtmlElement<Div> {
    GridCol().child(
        div()
        .classes("d-flex flex-column align-items-center")
        .child("HP")
        .child(
            div()
                .classes("border rounded my-auto d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div())
                .child(h2().child(
                    input()
                        .prop("value", get_current_hp())
                        .classes("mx-auto w-100")
                        //.style("margin-top", "-1rem")
                        .style("text-align", "center")
                        .style("background", "var(--bs-body-bg)")
                        .style("border", "none")
                )
                .style("margin-top", "-10%")),
            )
            .child(
                div()
                    .classes("border rounded mx-auto pt-1")
                    .style("width", "2rem")
                    .style("height", "2rem")
                    .style("margin-top", "-1rem")
                    .style("text-align", "center")
                    .style("background", "var(--bs-body-bg)")
                    .child(get_max_hp())
            )
    )
}
fn ACBox() -> HtmlElement<Div> {
    GridCol().child(
        div()
            .classes("d-flex flex-column align-items-center")
            .child("AC")
            .child([div()
                .classes("border rounded my-auto")
                .classes("d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div())
                .child(h2().style("margin-top", "-10%"))]),
    )
}

/*====================================
 *
 *  RIGHT COLUMN
 *
 *===================================*/

pub fn RightColumn() -> HtmlDiv {
    GridCol().child(
        ScrollableContainerBox()
            .child(h1().child("Features:"))
            .child(FeaturePanel(ClassTab(), SpeciesTab(), BackgroundTab())),
    )
}

pub fn get_prof_bonus() -> Signal<i32> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, CharacterDetails::prof_bonus)
}

pub fn get_level() -> Signal<i32> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, CharacterDetails::level)
}

pub fn get_species() -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, |c| c.species.to_string())
}

pub fn get_subspecies() -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, |c| c.subspecies.to_string())
}

pub fn set_subspecies() -> SignalSetter<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_write_slice(character, |c, v| c.subspecies = v)
}

pub fn get_class() -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, |c| c.class.to_string())
}

pub fn get_background() -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    create_read_slice(character, |c| c.background.to_string())
}

pub fn get_current_species() -> Signal<Option<Species>> {
    let api_data = expect_context::<FuturesWrapper>();
    Signal::derive(move || {
        let species = get_species()();
        if let Some(species_list) = api_data.species.get() {
            species_list.iter().find(|s| s.slug == species).cloned()
        } else {
            None
        }
    })
}

// Only returns a result if the current subspecies is a subspecies
// of the current species.
pub fn get_current_subspecies() -> Signal<Option<Subspecies>> {
    Signal::derive(move || {
        let subspecies = get_subspecies()();
        if let Some(species) = get_current_species()() {
            species
                .subraces
                .iter()
                .find(|s| s.slug == subspecies)
                .cloned()
        } else {
            None
        }
    })
}

pub fn get_current_class() -> Signal<Option<api_model::Class>> {
    let api_data = expect_context::<FuturesWrapper>();
    Signal::derive(move || {
        let class = get_class()();
        if let Some(class_list) = api_data.classes.get() {
            class_list.iter().find(|s| s.slug == class).cloned()
        } else {
            None
        }
    })
}

pub fn get_current_background() -> Signal<Option<Background>> {
    let api_data = expect_context::<FuturesWrapper>();
    Signal::derive(move || {
        let background = get_background()();
        if let Some(background_list) = api_data.backgrounds.get() {
            background_list
                .iter()
                .find(|s| s.slug == background)
                .cloned()
        } else {
            None
        }
    })
}

pub fn get_base_hp() -> Signal<i32> {
    Signal::derive(move || {
        if let Some(class) = get_current_class()() {
            class.base_hp()
        } else {
            0
        }
    })
}

pub fn get_current_hp() -> Signal<i32> {
    let ability_scores = expect_context::<AbilityScoresReactive>();
    Signal::derive(move || {
        get_base_hp()() + (ability_scores.con_mod() * get_level()())
    })
}

pub fn get_max_hp() -> Signal<i32> {
    get_current_hp()
}

pub fn get_base_features() -> Signal<Vec<Feature>> {
    Signal::derive(move || {
        let mut features_out: Vec<Feature> = vec![];

        // These are currently the only character properties
        // that can supply features, so they're the only ones
        // we need to listen too for now.
        // Subclass should be added later.

        // Species
        if let Some(species_def) = get_current_species()() {
            features_out.append(&mut species_def.features());
        }

        // Subspecies
        if let Some(subspecies_def) = get_current_subspecies()() {
            let f = &mut subspecies_def.features();
            features_out.append(f);
        }

        // Class
        if let Some(class) = get_current_class()() {
            features_out.append(&mut class.features());
        }

        // Background
        if let Some(background) = get_current_background()() {
            features_out.append(&mut background.features());
        }

        features_out
            .iter()
            .filter(|f| f.level <= get_level()())
            .cloned()
            .collect::<Vec<Feature>>()
    })
}

pub fn get_optional_features() -> Signal<Vec<(String, FeatureOptions)>> {
    Signal::derive(move || {
        get_base_features()()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Option(op) = &f.feature_type {
                    Some((f.feature_slug(), op.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, FeatureOptions)>>()
    })
}

pub fn get_current_features() -> Signal<Vec<Feature>> {
    let selected_optional_features =
        expect_context::<RwSignal<Vec<FeatureOptionsSelection>>>();

    Signal::derive(move || {
        let mut features_out: Vec<Feature> = get_base_features()();

        selected_optional_features.with(|selected| {
            for select in selected {
                let op_features = get_optional_features()();
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
    })
}

pub fn get_current_asis() -> Signal<Vec<CharacterAsi>> {
    Signal::derive(move || {
        get_current_features()()
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
    })
}

pub fn get_skill_proficencies() -> Signal<Vec<String>> {
    Signal::derive(move || {
        get_current_features()()
            .iter()
            .filter_map(|f| {
                if let FeatureType::SkillProficency(prof) = &f.feature_type {
                    Some(prof)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<String>>()
    })
}
