use leptos::{html::*, Scope};
use uuid::Uuid;

/* pub fn FlexRow(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "d-flex flex-row mb-3")
} */

pub fn GridRow(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "row")
}
pub fn GridRowMarginBottom(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "row mb-2")
}
pub fn GridCol(cx: Scope) -> HtmlElement<Div> {
    div(cx).attr("class", "col")
}
pub fn GridColJustify(cx: Scope) -> HtmlElement<Div> {
    div(cx)
        .classes("col")
        .classes("d-flex justify-content-center")
}
pub fn Box(cx: Scope) -> HtmlElement<Div> {
    div(cx)
        .classes("border rounded")
        .style("width", "5vw")
        .style("height", "5vw")
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
        .classes("container border rounded pt-2")
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

pub fn Tab(cx: Scope, id: &str, active: bool, text: &str) -> HtmlElement<Li> {
    let mut button = button(cx)
        .classes("nav-link")
        .id(id.to_string())
        .attr("data-bs-toggle", "tab")
        .attr("data-bs-target", format!("#{}-pane", id.to_string()))
        .attr("type", "button")
        .attr("role", "tab")
        .attr("aria-controls", format!("{}-pane", id.to_string()))
        .attr("aria-selected", "true")
        .child(text.to_string());
    if active {
        button = button.classes("active");
    }
    li(cx)
        .classes("nav-item")
        .attr("role", "presentation")
        .child(button)
}

pub fn TabPanel(
    cx: Scope,
    id: &str,
    active: bool,
    child: HtmlElement<Div>,
) -> HtmlElement<Div> {
    let classes = if active {
        "tab-pane fade show active"
    } else {
        "tab-pane fade"
    };
    div(cx)
        .classes(classes)
        .id(format!("{}-pane", id.to_string()))
        .attr("role", "tabpanel")
        .attr("aria-labelledby", id.to_string())
        .attr("tabindex", "0")
        .child(child)
}

pub fn HorizontalPanel(cx: Scope) -> HtmlElement<Div> {
    div(cx).classes("border rounded my-2 py-3 text-center")
}
