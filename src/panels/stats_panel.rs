use leptos::{
    create_signal, create_slice, ev, event_target_value, html::*, SignalSetter,
};
use leptos::{HtmlElement, RwSignal, Scope, Signal};

use crate::character_model::{
    AbilityScores, AbilityScoresReactive, CharacterDetails,
};
use crate::components::*;

pub fn StatsPanel(
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
                Signal::derive(cx, move || ability_scores.str_score()),
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
