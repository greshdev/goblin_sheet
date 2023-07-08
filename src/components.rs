use leptos::{html::*, Scope};

/* pub fn FlexRow(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "d-flex flex-row mb-3")
} */

pub fn GridRow(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "row mb-2")
}
pub fn GridCol(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "col")
}

pub fn CustomSelect(cx: Scope) -> HtmlElement<Select> {
    select(cx)
        .classes("form-select")
        .style("background-color", "transparent")
        .style("border", "none")
        .style("border-bottom", "thin solid white")
}
