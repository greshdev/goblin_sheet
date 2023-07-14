#![allow(non_snake_case)]

mod api;
mod character_model;
mod components;
mod header_panel;
mod markdown;

use crate::api::api_model::Class;
use crate::api::FuturesWrapper;
use crate::character_model::AbilityScoresReactive;
use crate::character_model::CharacterAsi;
use crate::character_model::CharacterDetails;
use crate::components::*;
use crate::markdown::*;
use api::api_extensions::*;

use api::api_model::Species;
use api::api_model::Subspecies;
use character_model::Ability;
use character_model::AbilityScores;
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
    let class_slice = create_read_slice(cx, character, |c| c.class.to_string());
    let background_slice =
        create_read_slice(cx, character, |c| c.background.to_string());
    let level = create_read_slice(cx, character, CharacterDetails::level);

    // Closure to reactively get the API definition of the current species.
    let current_species = Signal::derive(cx, move || {
        let species = species_slice();
        if let Some(species_list) = api_data.species.read(cx) {
            species_list.iter().find(|s| s.slug == species).cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current
    // subspecies. Only returns a result if the current subspecies is
    // a subspecies of the current species.
    let current_subspecies = Signal::derive(cx, move || {
        let subspecies = subspecies_slice();
        if let Some(species) = current_species() {
            species
                .subraces
                .iter()
                .find(|s| s.slug == subspecies)
                .cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current class.
    let current_class = Signal::derive(cx, move || {
        let class = class_slice();
        if let Some(class_list) = api_data.classes.read(cx) {
            class_list.iter().find(|s| s.slug == class).cloned()
        } else {
            None
        }
    });

    // Closure to reactively get the API definition of the current background.
    let current_background = Signal::derive(cx, move || {
        let background = background_slice();
        if let Some(background_list) = api_data.backgrounds.read(cx) {
            background_list
                .iter()
                .find(|s| s.slug == background)
                .cloned()
        } else {
            None
        }
    });

    let character_features = Signal::derive(cx, move || {
        let mut features_out: Vec<Feature> = vec![];
        // These are currently the only character properties
        // that can supply features, so they're the only ones
        // we need to listen too for now.
        // Subclass should be added later.

        // Species
        if let Some(species_def) = current_species() {
            features_out.append(&mut species_def.features());
        }

        // Subspecies
        if let Some(subspecies_def) = current_subspecies() {
            let f = &mut subspecies_def.features();
            features_out.append(f);
        }

        // Class
        if let Some(class) = current_class() {
            features_out.append(&mut class.features());
        }

        // Background
        if let Some(background) = current_background() {
            features_out.append(&mut background.features());
        }

        features_out
            .iter()
            .filter(|f| f.level <= level())
            .cloned()
            .collect::<Vec<Feature>>()
    });

    let current_asis = Signal::derive(cx, move || {
        character_features()
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
    });

    let ability_scores = AbilityScoresReactive {
        ability_scores: create_read_slice(cx, character, |c| {
            c.ability_scores.clone()
        }),
        asis: current_asis,
    };

    let _proficiencies = Signal::derive(cx, move || {
        character_features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Proficiency(profs) = &f.feature_type {
                    Some(profs)
                } else {
                    None
                }
            })
            .flatten()
            .cloned()
            .collect::<Vec<String>>()
    });

    let proficency_bonus =
        create_read_slice(cx, character, CharacterDetails::prof_bonus);

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
        div(cx).classes("container").child(StatsRow(
            cx,
            character,
            proficency_bonus,
            ability_scores,
        )),
        div(cx).attr("class", "container").child(
            GridRow(cx)
                // Left column
                .child(LeftColumn(
                    cx,
                    character_features,
                    proficency_bonus,
                    ability_scores,
                ))
                // Center column
                .child(GridCol(cx).child(ScrollableContainerBox(cx)))
                // Right column
                .child(RightColumn(
                    cx,
                    character_features,
                    current_species,
                    subspecies_signals,
                )),
        ),
    ]
}

type OptionList = Vec<HtmlElement<Option_>>;
type DivList = Vec<HtmlElement<Div>>;

/*====================================
 *
 *  LEFT COLUMN
 *
 *===================================*/

pub fn LeftColumn(
    cx: Scope,
    features: Signal<Vec<Feature>>,
    proficiency_bonus: Signal<i32>,
    ability_scores: AbilityScoresReactive,
) -> HtmlElement<Div> {
    let saves = Signal::derive(cx, move || {
        features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::SavingThrow(ability) = &f.feature_type {
                    Some(ability.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Ability>>()
    });
    let skills = Signal::derive(cx, move || {
        features()
            .iter()
            .filter_map(|f| {
                if let FeatureType::Proficiency(profs) = &f.feature_type {
                    Some(profs)
                } else {
                    None
                }
            })
            .flatten()
            .cloned()
            .collect::<Vec<String>>()
    });
    GridCol(cx).child(
        BoxedColumn(cx)
            .child(h1(cx).child("Proficencies:"))
            .child(
                ul(cx)
                    .classes("nav nav-tabs mb-3")
                    .id("proficencyTabs")
                    .attr("role", "tablist")
                    .child(vec![
                        Tab(cx, "saves-tab", true, "Saves"),
                        Tab(cx, "skills-tab", false, "Skills"),
                        Tab(cx, "other-tab", false, "Other"),
                    ]),
            )
            .child(
                ul(cx).style("padding-left", "0rem").child(
                    div(cx)
                        .classes("tab-content")
                        .id("proficencyTabsContent")
                        .child(vec![
                            TabPanel(
                                cx,
                                "saves-tab",
                                true,
                                SavesDisplay(
                                    cx,
                                    saves,
                                    proficiency_bonus,
                                    ability_scores,
                                ),
                            ),
                            TabPanel(
                                cx,
                                "skills-tab",
                                false,
                                SkillsDisplay(cx, skills),
                            ),
                            //TabPanel(cx, "other-tab", false, background_tab),
                        ]),
                ),
            ),
    )
}

fn calc_save(
    ability_scores: AbilityScoresReactive,
    saves: Signal<Vec<Ability>>,
    ability: Ability,
    proficiency_bonus: Signal<i32>,
) -> i32 {
    let bonus = if saves().contains(&ability) {
        proficiency_bonus()
    } else {
        0
    };
    ability_scores.get_ability_mod(&ability) + bonus
}

pub fn SavesDisplay(
    cx: Scope,
    saves: Signal<Vec<Ability>>,
    proficiency_bonus: Signal<i32>,
    ability_scores: AbilityScoresReactive,
) -> HtmlElement<Div> {
    div(cx).child(
        ul(cx).classes("list-group").child(
            [
                Ability::Strength,
                Ability::Dexterity,
                Ability::Constitution,
                Ability::Wisdom,
                Ability::Intelligence,
                Ability::Charisma,
            ]
            .iter()
            .map(|ability| {
                li(cx).classes("list-group-item").child(
                    div(cx)
                        .classes("d-flex justify-content-between")
                        .child(div(cx).child(ability.to_string().to_string()))
                        .child(div(cx).child(move || {
                            calc_save(
                                ability_scores,
                                saves,
                                ability.clone(),
                                proficiency_bonus,
                            )
                        })),
                )
            })
            .collect::<Vec<HtmlElement<Li>>>(),
        ),
    )
}

pub fn SkillsDisplay(
    cx: Scope,
    skills: Signal<Vec<String>>,
) -> HtmlElement<Div> {
    div(cx).child(ul(cx).classes("list-group").child(move || {
        skills()
            .iter()
            .map(|skill| {
                li(cx)
                    .classes("list-group-item")
                    .child(div(cx).child(skill.to_string()))
            })
            .collect::<Vec<HtmlElement<Li>>>()
    }))
}

/*====================================
 *
 *  RIGHT COLUMN
 *
 *===================================*/

pub fn RightColumn(
    cx: Scope,
    features: Signal<Vec<Feature>>,
    current_species: Signal<Option<Species>>,
    subspecies_signals: (Signal<String>, SignalSetter<String>),
) -> HtmlElement<Div> {
    GridCol(cx).child(
        ScrollableContainerBox(cx)
            .child(h1(cx).child("Features:"))
            .child(FeaturePanel(
                cx,
                ClassTab(cx, features),
                SpeciesTab(cx, subspecies_signals, current_species),
                BackgroundTab(cx, features),
            )),
    )
}

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

pub fn StatsRow(
    cx: Scope,
    character: RwSignal<CharacterDetails>,
    proficiency_bonus: Signal<i32>,
    ability_scores: AbilityScoresReactive,
) -> HtmlElement<Div> {
    HorizontalPanel(cx).child(
        GridRow(cx)
            .child(GridCol(cx).child(div(cx)
            .classes("d-flex flex-column")
            .child("Proficiency")
            .child(
                div(cx)
                    .classes("border rounded mx-auto d-flex align-items-center justify-content-center")
                    .child(div(cx))
                    .style("width", "4rem")
                    .style("height", "4rem")
                    .style("text-align", "center")
                    .child(h2(cx).child(proficiency_bonus).style("margin-top", "-10%")),
                )
                .child("Bonus")
            ))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Strength",
                Signal::derive(cx, move || ability_scores.dex_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_str,
                    |c, v| c.ability_scores.base_str = v,
                ),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Dexterity",
                Signal::derive(cx, move || ability_scores.dex_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_dex,
                    |c, v| c.ability_scores.base_dex = v,
                ),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Constitution",
                Signal::derive(cx, move || ability_scores.con_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_con,
                    |c, v| c.ability_scores.base_con = v,
                ),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Wisdom",
                Signal::derive(cx, move || ability_scores.wis_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_wis,
                    |c, v| c.ability_scores.base_wis = v,
                ),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Intelligence",
                Signal::derive(cx, move || ability_scores.int_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_int,
                    |c, v| c.ability_scores.base_int = v,
                ),
            )))
            .child(GridCol(cx).child(AbilityScoreBox(
                cx,
                "Charisma",
                Signal::derive(cx, move || ability_scores.cha_score()),
                create_slice(
                    cx,
                    character,
                    |c| c.ability_scores.base_cha,
                    |c, v| c.ability_scores.base_cha = v,
                ),
            ))),
    )
}
fn AbilityScoreBox(
    cx: Scope,
    score_name: &str,
    score: Signal<i32>,
    (score_base, set_score_base): (Signal<i32>, SignalSetter<i32>),
) -> HtmlElement<Div> {
    let score_mod =
        Signal::derive(cx, move || AbilityScores::score_to_mod(score()));

    let (edit_mode, set_edit_mode) = create_signal(cx, false);
    let display_score =
        move || if edit_mode() { score_base() } else { score() };

    div(cx)
        .classes("d-flex flex-column")
        .child(score_name.to_string())
        .child(
            div(cx)
                .classes("border rounded mx-auto d-flex align-items-center justify-content-center")
                .child(div(cx))
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                .child(h2(cx).child(score_mod).style("margin-top", "-10%")),
        )
        .child(
            input(cx)
                //div(cx)
                .classes("border rounded mx-auto")
                .style("width", "2rem")
                .style("height", "2rem")
                .style("margin-top", "-1rem")
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
