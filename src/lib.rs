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
use crate::dice::roll_dice;
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

use leptos::{component, IntoView, Scope};
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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Create reactive signal to store character state
    let character = create_rw_signal(cx, load_character());
    // Store that state globally
    provide_context(cx, character);

    // Update local storage whenever the character details change
    create_effect(cx, move |_| write_character_to_local_storage(character));

    // Create wrapper for async access to data from Open5e
    // and store it globally
    provide_context(cx, FuturesWrapper::new(cx));

    // TODO: Fix bug where selected optional features for a class are
    // retained if you change classes?
    let selected_optional_features: RwSignal<Vec<FeatureOptionsSelection>> =
        create_rw_signal(cx, load_selected_optional_features());
    provide_context(cx, selected_optional_features);

    // Update local storage when the selected optional features change
    create_effect(cx, move |_| {
        write_optional_features_to_local_storage(selected_optional_features)
    });

    provide_context(
        cx,
        AbilityScoresReactive {
            ability_scores: create_read_slice(cx, character, |c| {
                c.ability_scores.clone()
            }),
            asis: get_current_asis(cx),
        },
    );

    let attack_list: RwSignal<Vec<AttackAction>> =
        create_rw_signal(cx, load_attack_list());
    provide_context(cx, attack_list);
    create_effect(cx, move |_| write_attack_list_to_local_storage(attack_list));

    // ==============
    // RENDER
    // ==============
    vec![
        div(cx)
            .id("dice-box")
            .classes("position-absolute top-0 start-0 w-100 h-100 pe-none"),
        div(cx).classes("position-absolute top-0 start-0").child(
            a(cx).child("Reset").on(ev::click, move |_| {
                // Reset all global data to default
                character.update(|c| *c = CharacterDetails::new());
                selected_optional_features.update(|s| *s = vec![]);
                attack_list.update(|a| *a = vec![])
            }),
        ),
        HeaderPanel(cx),
        // Stats row
        div(cx).classes("container").child(StatsPanel(cx)),
        div(cx).attr("class", "container").child(
            GridRow(cx)
                // Left column
                .child(GridCol(cx).child(ProfPanel(cx)))
                // Center column
                .child(CenterColumn(cx))
                // Right column
                .child(RightColumn(cx)),
        ),
        // OptionSelectionModal(cx),
        div(cx)
            .classes("position-absolute bottom-0 end-0 p-3")
            .child(
                a(cx)
                    .child(
                        img(cx)
                            .attr("src", "github-mark-white.svg")
                            .attr("alt", "Github Logo")
                            .style("height", "5vh")
                            .style("width", "5vh"),
                    )
                    .attr("href", "https://github.com/greshdev/goblin_sheet"),
            ),
    ]
}

pub fn CenterColumn(cx: Scope) -> HtmlDiv {
    GridCol(cx)
        .child(
            div(cx).classes("container border rounded pt-2 mb-2").child(
                GridRow(cx).child([ProfBonusBox(cx), ACBox(cx), HPBox(cx)]),
            ),
        )
        .child(
            BoxedColumnFlexible(cx)
                .style("height", "49.5vh")
                .child(CenterPanel(cx)),
        )
}

fn ProfBonusBox(cx: Scope) -> HtmlElement<Div> {
    GridCol(cx).child(
        div(cx)
            .classes("d-flex flex-column align-items-center")
            .child("Proficiency")
            .child([
                div(cx)
                .classes("border rounded my-auto d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div(cx))
                .child(
                    h2(cx)
                        .child(get_prof_bonus(cx))
                        .style("margin-top", "-10%")),
                div(cx).child("Bonus")]
            )
        )
}
fn HPBox(cx: Scope) -> HtmlElement<Div> {
    GridCol(cx).child(
        div(cx)
        .classes("d-flex flex-column align-items-center")
        .child("HP")
        .child(
            div(cx)
                .classes("border rounded my-auto d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div(cx))
                .child(h2(cx).child(
                    input(cx)
                        .prop("value", get_current_hp(cx))
                        .classes("mx-auto w-100")
                        //.style("margin-top", "-1rem")
                        .style("text-align", "center")
                        .style("background", "var(--bs-body-bg)")
                        .style("border", "none")
                )
                .style("margin-top", "-10%")),
            )
            .child(
                div(cx)
                    .classes("border rounded mx-auto pt-1")
                    .style("width", "2rem")
                    .style("height", "2rem")
                    .style("margin-top", "-1rem")
                    .style("text-align", "center")
                    .style("background", "var(--bs-body-bg)")
                    .child(get_max_hp(cx))
            )
    )
}
fn ACBox(cx: Scope) -> HtmlElement<Div> {
    GridCol(cx).child(
        div(cx)
            .classes("d-flex flex-column align-items-center")
            .child("AC")
            .child([div(cx)
                .classes("border rounded my-auto")
                .classes("d-flex align-items-center justify-content-center")
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                //.child(div(cx))
                .child(h2(cx).style("margin-top", "-10%"))]),
    )
}

/*====================================
 *
 *  RIGHT COLUMN
 *
 *===================================*/

pub fn RightColumn(cx: Scope) -> HtmlDiv {
    GridCol(cx).child(
        ScrollableContainerBox(cx)
            .child(h1(cx).child("Features:"))
            .child(FeaturePanel(
                cx,
                ClassTab(cx),
                SpeciesTab(cx),
                BackgroundTab(cx),
            )),
    )
}

pub fn get_prof_bonus(cx: Scope) -> Signal<i32> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, CharacterDetails::prof_bonus)
}

pub fn get_level(cx: Scope) -> Signal<i32> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, CharacterDetails::level)
}

pub fn get_species(cx: Scope) -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, |c| c.species.to_string())
}

pub fn get_subspecies(cx: Scope) -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, |c| c.subspecies.to_string())
}

pub fn set_subspecies(cx: Scope) -> SignalSetter<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_write_slice(cx, character, |c, v| c.subspecies = v)
}

pub fn get_class(cx: Scope) -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, |c| c.class.to_string())
}

pub fn get_background(cx: Scope) -> Signal<String> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
    create_read_slice(cx, character, |c| c.background.to_string())
}

pub fn get_current_species(cx: Scope) -> Signal<Option<Species>> {
    let api_data = expect_context::<FuturesWrapper>(cx);
    Signal::derive(cx, move || {
        let species = get_species(cx)();
        if let Some(species_list) = api_data.species.read(cx) {
            species_list.iter().find(|s| s.slug == species).cloned()
        } else {
            None
        }
    })
}

// Only returns a result if the current subspecies is a subspecies
// of the current species.
pub fn get_current_subspecies(cx: Scope) -> Signal<Option<Subspecies>> {
    Signal::derive(cx, move || {
        let subspecies = get_subspecies(cx)();
        if let Some(species) = get_current_species(cx)() {
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

pub fn get_current_class(cx: Scope) -> Signal<Option<api_model::Class>> {
    let api_data = expect_context::<FuturesWrapper>(cx);
    Signal::derive(cx, move || {
        let class = get_class(cx)();
        if let Some(class_list) = api_data.classes.read(cx) {
            class_list.iter().find(|s| s.slug == class).cloned()
        } else {
            None
        }
    })
}

pub fn get_current_background(cx: Scope) -> Signal<Option<Background>> {
    let api_data = expect_context::<FuturesWrapper>(cx);
    Signal::derive(cx, move || {
        let background = get_background(cx)();
        if let Some(background_list) = api_data.backgrounds.read(cx) {
            background_list
                .iter()
                .find(|s| s.slug == background)
                .cloned()
        } else {
            None
        }
    })
}

pub fn get_base_hp(cx: Scope) -> Signal<i32> {
    Signal::derive(cx, move || {
        if let Some(class) = get_current_class(cx)() {
            class.base_hp()
        } else {
            0
        }
    })
}

pub fn get_current_hp(cx: Scope) -> Signal<i32> {
    let ability_scores = expect_context::<AbilityScoresReactive>(cx);
    Signal::derive(cx, move || {
        get_base_hp(cx)() + (ability_scores.con_mod() * get_level(cx)())
    })
}

pub fn get_max_hp(cx: Scope) -> Signal<i32> {
    get_current_hp(cx)
}

pub fn get_base_features(cx: Scope) -> Signal<Vec<Feature>> {
    Signal::derive(cx, move || {
        let mut features_out: Vec<Feature> = vec![];

        // These are currently the only character properties
        // that can supply features, so they're the only ones
        // we need to listen too for now.
        // Subclass should be added later.

        // Species
        if let Some(species_def) = get_current_species(cx)() {
            features_out.append(&mut species_def.features());
        }

        // Subspecies
        if let Some(subspecies_def) = get_current_subspecies(cx)() {
            let f = &mut subspecies_def.features();
            features_out.append(f);
        }

        // Class
        if let Some(class) = get_current_class(cx)() {
            features_out.append(&mut class.features());
        }

        // Background
        if let Some(background) = get_current_background(cx)() {
            features_out.append(&mut background.features());
        }

        features_out
            .iter()
            .filter(|f| f.level <= get_level(cx)())
            .cloned()
            .collect::<Vec<Feature>>()
    })
}

pub fn get_optional_features(
    cx: Scope,
) -> Signal<Vec<(String, FeatureOptions)>> {
    Signal::derive(cx, move || {
        get_base_features(cx)()
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

pub fn get_current_features(cx: Scope) -> Signal<Vec<Feature>> {
    let selected_optional_features =
        expect_context::<RwSignal<Vec<FeatureOptionsSelection>>>(cx);

    Signal::derive(cx, move || {
        let mut features_out: Vec<Feature> = get_base_features(cx)();

        selected_optional_features.with(|selected| {
            for select in selected {
                let op_features = get_optional_features(cx)();
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

pub fn get_current_asis(cx: Scope) -> Signal<Vec<CharacterAsi>> {
    Signal::derive(cx, move || {
        get_current_features(cx)()
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

pub fn get_skill_proficencies(cx: Scope) -> Signal<Vec<String>> {
    Signal::derive(cx, move || {
        get_current_features(cx)()
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
