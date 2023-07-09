use comrak::{markdown_to_html, ComrakOptions};
pub fn parse_markdown(markdown: &str) -> String {
    markdown_to_html(markdown, &ComrakOptions::default())
}
pub fn parse_markdown_table(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    markdown_to_html(markdown, &options)
}
