use leptos::{html::*, *};

use crate::{
    api::api_model::Class, api::*, character_model::CharacterDetails,
    components::*, OptionList,
};

pub fn HeaderPanel(cx: Scope) -> HtmlElement<Div> {
    let character = expect_context::<RwSignal<CharacterDetails>>(cx);
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

    div(cx).classes("container").child(
        HorizontalPanel(cx).child(
            GridRow(cx)
                .classes("row container gx-5")
                .child(
                    div(cx)
                        .classes("col d-flex align-items-center")
                        .child(NameInputBox(cx, name, set_name)),
                )
                .child(
                    GridCol(cx)
                        .child(
                            GridRowMarginBottom(cx)
                                .child(ClassDropdown(cx, class, set_class)),
                        )
                        .child(GridRow(cx).child(SpeciesDropdown(
                            cx,
                            species,
                            set_species,
                        ))),
                )
                .child(
                    GridCol(cx)
                        //.attr("class", "col-sm-3")
                        .child(
                            GridRowMarginBottom(cx)
                                .child(LevelDropdown(cx, level, set_level)),
                        )
                        .child(GridRow(cx).child(div(cx).child(
                            BackgroundDropdown(cx, background, set_background),
                        ))),
                ),
        ),
    )
}

fn SpeciesDropdown(
    cx: Scope,
    species: Signal<String>,
    set_species: SignalSetter<String>,
) -> impl IntoView {
    let future = expect_context::<FuturesWrapper>(cx).species;
    let change_species = move |e| {
        set_species(event_target_value(&e));
    };
    CustomSelect(cx)
        //.classes("mb-3")
        .prop("value", species)
        .on(ev::change, change_species)
        .attr("placeholder", "Species")
        .child(option(cx).prop("value", "").child("Select a species..."))
        .child(move || {
            future
                .with(cx, |species_list| {
                    species_list
                        .iter()
                        .map(|s| {
                            OptionWithDocTitle(
                                cx,
                                &species(),
                                &s.slug,
                                &s.name,
                                &s.document_title,
                            )
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading....")])
        })
}

fn ClassOptionList(
    cx: Scope,
    classes: &[Class],
    class: Signal<String>,
) -> OptionList {
    classes
        .iter()
        .map(|c| {
            option(cx)
                .prop("value", c.slug.clone())
                .prop("selected", c.slug == class())
                .child(c.name.clone())
        })
        .collect::<OptionList>()
}

fn ClassDropdown(
    cx: Scope,
    class: Signal<String>,
    set_class: SignalSetter<String>,
) -> impl IntoView {
    let future = expect_context::<FuturesWrapper>(cx).classes;
    CustomSelect(cx)
        .prop("value", class)
        .on(ev::change, move |e| set_class(event_target_value(&e)))
        .child(option(cx).child("Select a class...").prop("value", ""))
        .child(move || {
            future
                .with(cx, |c| ClassOptionList(cx, c, class))
                .unwrap_or(vec![option(cx).child("Loading...")])
        })
}

fn BackgroundDropdown(
    cx: Scope,
    background: Signal<String>,
    set_background: SignalSetter<String>,
) -> HtmlElement<Select> {
    let future = expect_context::<FuturesWrapper>(cx).backgrounds;
    CustomSelect(cx)
        .prop("value", background)
        .on(ev::change, move |e| set_background(event_target_value(&e)))
        .child(option(cx).child("Select a background...").prop("value", ""))
        .child(move || {
            future
                .with(cx, |bg| {
                    bg.iter()
                        .map(|c| {
                            OptionWithDocTitle(
                                cx,
                                &background.get(),
                                &c.slug,
                                &c.name,
                                &c.document_title,
                            )
                        })
                        .collect::<OptionList>()
                })
                .unwrap_or(vec![option(cx).child("Loading...")])
        })
}

pub fn NameInputBox(
    cx: Scope,
    name: Signal<String>,
    set_name: SignalSetter<String>,
) -> HtmlElement<Input> {
    input(cx)
        .classes("form-control")
        .attr("placeholder", "Character Name")
        .on(ev::input, move |e| set_name(event_target_value(&e)))
        .prop("value", name)
}

pub fn LevelDropdown(
    cx: Scope,
    level: Signal<i32>,
    set_level: SignalSetter<i32>,
) -> HtmlElement<Div> {
    div(cx)
        .classes("input-group")
        .child(div(cx).classes("input-group-text").child("Level:"))
        .child(
            CustomSelect(cx)
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
                            option(cx)
                                .prop("value", i)
                                .prop("selected", i == level.get())
                                .child(i.to_string())
                        })
                        .collect::<OptionList>(),
                ),
        )
}
