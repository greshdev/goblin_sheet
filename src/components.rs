#![allow(dead_code)]
use leptos::html::*;
use uuid::Uuid;

/* pub fn FlexRow() -> HtmlDiv {
    div().attr("class", "d-flex flex-row mb-3")
} */

pub type HtmlDiv = HtmlElement<Div>;
pub type OptionList = Vec<HtmlElement<Option_>>;
pub type DivList = Vec<HtmlDiv>;

pub fn GridRow() -> HtmlDiv {
    div().attr("class", "row")
}
pub fn GridRowMarginBottom() -> HtmlDiv {
    div().attr("class", "row mb-2")
}
pub fn GridCol() -> HtmlDiv {
    div().attr("class", "col")
}
pub fn GridColJustify() -> HtmlDiv {
    div()
        .classes("col")
        .classes("d-flex justify-content-center")
}
pub fn Box() -> HtmlDiv {
    div()
        .classes("border rounded")
        .style("width", "5vw")
        .style("height", "5vw")
}

pub fn CustomSelect() -> HtmlElement<Select> {
    select().classes("form-select")
    //.style("background-color", "transparent")
    //.style("border", "none")
    //.style("border-bottom", "thin solid white")
}

///
/// # Arguments
///
/// * `current` - The currently selected item. An item with a slug matching this
/// string will be marked "selected"
/// * `slug` - The "value" of this option
/// * `name` - Part of the title of this option
/// * `doc_title` - The part of the title of this option, which goes in
/// parenthesis after the name.
pub fn OptionWithDocTitle(
    current: &str,
    slug: &str,
    name: &str,
    doc_title: &str,
) -> HtmlElement<Option_> {
    option()
        .prop("value", slug)
        .child(format!("{} ({})", name, doc_title))
        .prop("selected", slug == current)
}

pub fn ScrollableContainerBox() -> HtmlDiv {
    BoxedColumn().style("overflow-y", "auto")
}

pub fn BoxedColumn() -> HtmlDiv {
    div()
        .style("height", "65vh")
        .classes("container border rounded pt-2")
}

pub fn BoxedColumnFlexible() -> HtmlDiv {
    div().classes("container border rounded pt-2")
}

pub fn AccordionHeader(
    accordion_id: &str,
    content: HtmlDiv,
) -> HtmlElement<H2> {
    h2().classes("accordion-header").child(
        button()
            .classes("accordion-button collapsed")
            .prop("type", "button")
            .attr("data-bs-toggle", "collapse")
            .attr("data-bs-target", format!("#{}", accordion_id))
            .attr("aria-controls", accordion_id.to_string())
            .child(content),
    )
}

pub fn AccordionItem(
    header_content: HtmlDiv,
    dropdown_content: HtmlDiv,
) -> HtmlDiv {
    let id = format!("collapse-{}", Uuid::new_v4());
    div()
        .classes("accordion-item")
        .child(AccordionHeader(&id, header_content))
        .child(
            div()
                .id(id)
                .classes("accordion-collapse collapse")
                .attr("data-bs-parent", "#featuresAccordion")
                .child(dropdown_content.classes("accordion-body")),
        )
}

pub fn Tab(id: &str, active: bool, text: &str) -> HtmlElement<Li> {
    let mut button = button()
        .classes("nav-link")
        .id(id.to_string())
        .attr("data-bs-toggle", "tab")
        .attr("data-bs-target", format!("#{}-pane", id))
        .attr("type", "button")
        .attr("role", "tab")
        .attr("aria-controls", format!("{}-pane", id))
        .attr("aria-selected", "true")
        .child(text.to_string());
    if active {
        button = button.classes("active");
    }
    li().classes("nav-item")
        .attr("role", "presentation")
        .child(button)
}

pub fn TabPanel(id: &str, active: bool, child: HtmlDiv) -> HtmlDiv {
    let classes = if active {
        "tab-pane fade show active"
    } else {
        "tab-pane fade"
    };
    div()
        .classes(classes)
        .id(format!("{}-pane", id))
        .attr("role", "tabpanel")
        .attr("aria-labelledby", id.to_string())
        .attr("tabindex", "0")
        .child(child)
}

pub fn HorizontalPanel() -> HtmlDiv {
    div().classes("border rounded my-2 py-3 text-center")
}
