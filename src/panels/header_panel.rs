use leptos::{html::*, *};

use crate::{
    api::api_model::Class, api::*, character_model::CharacterDetails,
    components::*,
};

pub fn HeaderPanel() -> HtmlElement<Div> {
    let character = expect_context::<RwSignal<CharacterDetails>>();
    let (species, set_species) = create_slice(
        character,
        |c| c.species.to_string(),
        |c, n| {
            c.species = n;
            // Clear the subspecies when the species changes
            c.subspecies = String::new();
        },
    );
    let (class, set_class) =
        create_slice(character, |c| c.class.to_string(), |c, v| c.class = v);
    let (background, set_background) = create_slice(
        character,
        |c| c.background.to_string(),
        |c, v| c.background = v,
    );

    let (name, set_name) =
        create_slice(character, |c| c.name.to_string(), |c, n| c.name = n);

    let (level, set_level) = create_slice(
        character,
        CharacterDetails::level,
        CharacterDetails::set_level,
    );

    div().classes("container").child(
        HorizontalPanel().child(
            GridRow()
                .classes("row container gx-5")
                .child(
                    div()
                        .classes("col d-flex align-items-center")
                        .child(NameInputBox(name, set_name)),
                )
                .child(
                    GridCol()
                        .child(
                            GridRowMarginBottom()
                                .child(ClassDropdown(class, set_class)),
                        )
                        .child(
                            GridRow()
                                .child(SpeciesDropdown(species, set_species)),
                        ),
                )
                .child(
                    GridCol()
                        //.attr("class", "col-sm-3")
                        .child(
                            GridRowMarginBottom()
                                .child(LevelDropdown(level, set_level)),
                        )
                        .child(GridRow().child(div().child(
                            BackgroundDropdown(background, set_background),
                        ))),
                ),
        ),
    )
}

fn SpeciesDropdown(
    species: Signal<String>,
    set_species: SignalSetter<String>,
) -> impl IntoView {
    let future = expect_context::<FuturesWrapper>().species;
    let change_species = move |e| {
        set_species(event_target_value(&e));
    };
    CustomSelect()
        //.classes("mb-3")
        .prop("value", species)
        .on(ev::change, change_species)
        .attr("placeholder", "Species")
        .child(option().prop("value", "").child("Select a species..."))
        .child(move || {
            future.with(|species_list| {
                if let Some(species_list) = species_list {
                    species_list
                        .iter()
                        .map(|s| {
                            OptionWithDocTitle(
                                &species(),
                                &s.slug,
                                &s.name,
                                &s.document_title,
                            )
                        })
                        .collect::<OptionList>()
                } else {
                    vec![option().child("Loading....")]
                }
            })
            //.unwrap_or(vec![option().child("Loading....")])
        })
}

fn ClassOptionList(classes: &[Class], class: Signal<String>) -> OptionList {
    classes
        .iter()
        .map(|c| {
            option()
                .prop("value", c.slug.clone())
                .prop("selected", c.slug == class())
                .child(c.name.clone())
        })
        .collect::<OptionList>()
}

fn ClassDropdown(
    class: Signal<String>,
    set_class: SignalSetter<String>,
) -> impl IntoView {
    let future = expect_context::<FuturesWrapper>().classes;
    CustomSelect()
        .prop("value", class)
        .on(ev::change, move |e| set_class(event_target_value(&e)))
        .child(option().child("Select a class...").prop("value", ""))
        .child(move || {
            future.with(|c| {
                if let Some(c) = c {
                    ClassOptionList(c, class)
                } else {
                    vec![option().child("Loading...")]
                }
            })
        })
}

fn BackgroundDropdown(
    background: Signal<String>,
    set_background: SignalSetter<String>,
) -> HtmlElement<Select> {
    let future = expect_context::<FuturesWrapper>().backgrounds;
    CustomSelect()
        .prop("value", background)
        .on(ev::change, move |e| set_background(event_target_value(&e)))
        .child(option().child("Select a background...").prop("value", ""))
        .child(move || {
            future.with(|bg| {
                if let Some(bg) = bg {
                    bg.iter()
                        .map(|c| {
                            OptionWithDocTitle(
                                &background.get(),
                                &c.slug,
                                &c.name,
                                &c.document_title,
                            )
                        })
                        .collect::<OptionList>()
                } else {
                    vec![option().child("Loading...")]
                }
            })
            //.unwrap_or(vec![option().child("Loading...")])
        })
}

pub fn NameInputBox(
    name: Signal<String>,
    set_name: SignalSetter<String>,
) -> HtmlElement<Input> {
    input()
        .classes("form-control")
        .attr("placeholder", "Character Name")
        .on(ev::input, move |e| set_name(event_target_value(&e)))
        .prop("value", name)
}

pub fn LevelDropdown(
    level: Signal<i32>,
    set_level: SignalSetter<i32>,
) -> HtmlElement<Div> {
    div()
        .classes("input-group")
        .child(div().classes("input-group-text").child("Level:"))
        .child(
            CustomSelect()
                .prop("value", level)
                .on(ev::input, move |e| {
                    let event_val = event_target_value(&e);
                    if let Ok(num) = str::parse::<i32>(&event_val) {
                        set_level(num);
                    }
                })
                .child(
                    (1..=20)
                        .map(|i| {
                            option()
                                .prop("value", i)
                                .prop("selected", i == level.get())
                                .child(i.to_string())
                        })
                        .collect::<OptionList>(),
                ),
        )
}
