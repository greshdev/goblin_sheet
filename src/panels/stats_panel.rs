use leptos::logging::log;
use leptos::{
    create_action, create_signal, create_slice, ev, event_target_value,
    expect_context, html::*, SignalSetter,
};
use leptos::{RwSignal, Signal};

use crate::character_model::{
    AbilityScores, AbilityScoresReactive, CharacterDetails,
};
use crate::components::*;
use crate::dice::get_dice_result;

pub fn StatsPanel() -> HtmlElement<Div> {
    let ability_scores = expect_context::<AbilityScoresReactive>();
    let character = expect_context::<RwSignal<CharacterDetails>>();
    HorizontalPanel().child(
        GridRow()
            .child(GridCol().child(AbilityScoreBox(
                "Strength",
                Signal::derive(move || ability_scores.str_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_str,
                    |c, v| c.ability_scores.base_str = v,
                ),
            )))
            .child(GridCol().child(AbilityScoreBox(
                "Dexterity",
                Signal::derive(move || ability_scores.dex_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_dex,
                    |c, v| c.ability_scores.base_dex = v,
                ),
            )))
            .child(GridCol().child(AbilityScoreBox(
                "Constitution",
                Signal::derive(move || ability_scores.con_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_con,
                    |c, v| c.ability_scores.base_con = v,
                ),
            )))
            .child(GridCol().child(AbilityScoreBox(
                "Wisdom",
                Signal::derive(move || ability_scores.wis_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_wis,
                    |c, v| c.ability_scores.base_wis = v,
                ),
            )))
            .child(GridCol().child(AbilityScoreBox(
                "Intelligence",
                Signal::derive(move || ability_scores.int_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_int,
                    |c, v| c.ability_scores.base_int = v,
                ),
            )))
            .child(GridCol().child(AbilityScoreBox(
                "Charisma",
                Signal::derive(move || ability_scores.cha_score()),
                create_slice(
                    character,
                    |c| c.ability_scores.base_cha,
                    |c, v| c.ability_scores.base_cha = v,
                ),
            ))),
    )
}
struct RollDiceParams {
    dice_string: String,
    bonus: i64,
}
fn AbilityScoreBox(
    score_name: &str,
    score: Signal<i32>,
    (score_base, set_score_base): (Signal<i32>, SignalSetter<i32>),
) -> HtmlElement<Div> {
    let score_mod =
        Signal::derive(move || AbilityScores::score_to_mod(score()));

    let (edit_mode, set_edit_mode) = create_signal(false);
    let display_score =
        move || if edit_mode() { score_base() } else { score() };

    let roll_dice = create_action(|input: &RollDiceParams| {
        // Get a copy of the input values so that we can move them into
        // the async block below.
        let roll_string = input.dice_string.clone();
        let roll_bonus = input.bonus;
        async move {
            let r = get_dice_result(&roll_string).await;
            if let Some(result) = r.first() {
                log!("{}", result.value + roll_bonus)
            }
        }
    });

    div()
        .classes("d-flex flex-column")
        .child(score_name.to_string())
        .child(
            div()
                .classes("border rounded mx-auto d-flex align-items-center justify-content-center")
                .child(div())
                .style("width", "4rem")
                .style("height", "4rem")
                .style("text-align", "center")
                .style("cursor", "pointer")
                .child(h2().child(score_mod).style("margin-top", "-10%"))
                .on(ev::click, move |_| {
                    roll_dice.dispatch(
                        RollDiceParams{
                            dice_string:"1d20".to_string(), 
                            bonus: i64::from(score_mod()) 
                        }
                    )
                }),
        )
        .child(
            input()
                //div()
                .classes("border rounded mx-auto")
                .style("width", "2rem")
                .style("height", "2rem")
                .style("margin-top", "-1rem")
                .style("text-align", "center")
                //.classes("p-1")
                .style("background", "var(--bs-body-bg)")
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
