use leptos::{html::*, Scope};
use uuid::Uuid;

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
    select(cx).classes("form-select")
    //.style("background-color", "transparent")
    //.style("border", "none")
    //.style("border-bottom", "thin solid white")
}

pub fn ScrollableContainerBox(cx: Scope) -> HtmlElement<Div> {
    div(cx)
        .style("height", "80vh")
        .style("overflow-y", "auto")
        .classes("container border")
}

pub fn AccordionHeader(
    cx: Scope,
    accordion_id: String,
    content: HtmlElement<Div>,
) -> HtmlElement<H2> {
    h2(cx).classes("accordion-header").child(
        button(cx)
            .classes("accordion-button collapsed")
            .prop("type", "button")
            .attr("data-bs-toggle", "collapse")
            .attr("data-bs-target", format!("#{}", accordion_id))
            .attr("aria-controls", accordion_id)
            .child(content),
    )
}

pub fn AccordionItem(
    cx: Scope,
    header_content: HtmlElement<Div>,
    dropdown_content: HtmlElement<Div>,
) -> HtmlElement<Div> {
    let id = format!("collapse-{}", Uuid::new_v4());
    div(cx)
        .classes("accordion-item")
        .child(AccordionHeader(cx, id.clone(), header_content))
        .child(
            div(cx)
                .id(id)
                .classes("accordion-collapse collapse")
                .attr("data-bs-parent", "#featuresAccordion")
                .child(dropdown_content.classes("accordion-body")),
        )
}
