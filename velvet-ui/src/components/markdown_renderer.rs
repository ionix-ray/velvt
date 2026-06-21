//! Renders a case study's markdown body to HTML via pulldown-cmark (GFM).
//! Source markdown is first-party, repo-committed content
//! (`docs/cse_studies/`) embedded at compile time — not user input — so
//! `dangerous_inner_html` carries no XSS risk here; it's the standard
//! Dioxus pattern for pre-rendered HTML.

use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser, html};

/// Markdown -> HTML, GFM-flavored (tables, strikethrough, task lists,
/// footnotes, smart punctuation, heading attributes).
pub fn render_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn MarkdownRenderer(content: String) -> Element {
    let html_output = render_markdown(&content);
    rsx! {
        div { class: "v-prose", dangerous_inner_html: "{html_output}" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::VirtualDom;

    #[test]
    fn render_markdown_converts_headings_and_emphasis() {
        let html = render_markdown("# Title\n\nSome **bold** text.");
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn render_markdown_supports_gfm_tables_and_strikethrough() {
        let html = render_markdown("~~old~~\n\n| A | B |\n|---|---|\n| 1 | 2 |");
        assert!(html.contains("<del>old</del>"));
        assert!(html.contains("<table>"));
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapMarkdown() -> Element {
        rsx! {
            MarkdownRenderer { content: "Hello **world**".to_string() }
        }
    }

    #[test]
    fn markdown_renderer_component_wraps_output_in_prose_div() {
        let html = render(WrapMarkdown);
        assert!(html.contains("v-prose"));
        assert!(html.contains("<strong>world</strong>"));
    }
}
