//! Shared header for the standalone case-study pages (`/cases`,
//! `/cases/tag/:tag`, `/cases/:slug`). These pages are reached via full
//! page navigation (fresh WASM boot), so unlike `Home` they never inherit
//! the `data-theme` attribute set by `Home`'s own signal/effect — this
//! component owns its own copy of that exact mechanism so dark/light mode
//! works consistently here too, plus the real brand mark (not text) and a
//! "back" link slot.

use crate::components::topbar::{theme_icon_for, toggle_theme};
use crate::theme::brand::brand_mark;
use dioxus::prelude::*;

#[component]
pub fn CaseHeader(
    #[props(default)] back_href: Option<String>,
    #[props(default)] back_label: Option<String>,
) -> Element {
    let mut theme = use_signal(|| "dark".to_string());

    // Apply theme attribute to <html> — same pattern as Home (home.rs), kept
    // SSR-safe by the `if let Some(win) = web_sys::window()` guard, which is
    // `None` outside a browser and no-ops there.
    {
        use_effect(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(root) = doc.document_element() {
                        let _ = root.set_attribute("data-theme", &theme.read());
                    }
                }
            }
        });
    }

    let theme_icon = theme_icon_for(&theme.read());

    rsx! {
        header { class: "v-topbar v-topbar--standalone", id: "topbar",
            a { class: "v-topbar__brand", href: "/", "data-spa": "true",
                img { src: brand_mark(), alt: "VELVT" }
            }
            div { class: "v-topbar__actions",
                if let (Some(href), Some(label)) = (back_href.clone(), back_label.clone()) {
                    a {
                        class: "v-case-page__back",
                        href: "{href}",
                        "data-spa": "true",
                        "{label}"
                    }
                }
                button {
                    class: "v-theme-toggle",
                    title: "Toggle theme",
                    onclick: move |_| {
                        let next = toggle_theme(&theme.read());
                        theme.set(next);
                    },
                    "{theme_icon}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::VirtualDom;

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapNoBack() -> Element {
        rsx! {
            CaseHeader {}
        }
    }

    #[component]
    fn WrapWithBack() -> Element {
        rsx! {
            CaseHeader { back_href: "/cases".to_string(), back_label: "All case studies".to_string() }
        }
    }

    #[test]
    fn renders_the_brand_image_not_text() {
        let html = render(WrapNoBack);
        assert!(html.contains("<img"));
        assert!(html.contains("new-logo-1"));
        assert!(!html.contains(">VAELVET<"));
    }

    #[test]
    fn renders_a_theme_toggle_button() {
        let html = render(WrapNoBack);
        assert!(html.contains("v-theme-toggle"));
        assert!(html.contains("Toggle theme"));
    }

    #[test]
    fn omits_back_link_when_not_provided() {
        let html = render(WrapNoBack);
        assert!(!html.contains("v-case-page__back"));
    }

    #[test]
    fn renders_back_link_when_provided() {
        let html = render(WrapWithBack);
        assert!(html.contains("v-case-page__back"));
        assert!(html.contains("href=\"/cases\""));
        assert!(html.contains("All case studies"));
    }
}
