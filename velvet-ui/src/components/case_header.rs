//! Shared header for the standalone case-study pages (`/achivements`,
//! `/achivements/tag/:tag`, `/achivements/:slug`). These pages are reached via full
//! page navigation (fresh WASM boot), so unlike `Home` they never inherit
//! the `data-theme` attribute set by `Home`'s own signal/effect — this
//! component owns its own copy of that exact mechanism so dark/light mode
//! works consistently here too, plus the real brand mark (not text) and a
//! "back" link slot.

use crate::components::topbar::{theme_icon_for, toggle_theme};
use crate::theme::wordmark::Wordmark;
use dioxus::prelude::*;

#[component]
pub fn CaseHeader(
    #[props(default)] back_href: Option<String>,
    #[props(default)] back_label: Option<String>,
) -> Element {
    let mut theme = use_signal(|| "light".to_string());

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
                aria_label: "Velvt — home",
                Wordmark { class: "v-logo-svg".to_string(), title: None }
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
            CaseHeader { back_href: "/achivements".to_string(), back_label: "All case studies".to_string() }
        }
    }

    /// The brand must be the real lockup (inline SVG + emblem asset), never a
    /// bare text fallback — and the wordmark text must inherit `currentColor`
    /// so CSS owns its colour instead of a baked fill.
    #[test]
    fn renders_the_brand_lockup_not_baked_text() {
        let html = render(WrapNoBack);
        assert!(html.contains("<svg"), "brand must be an inline svg lockup");
        assert!(
            html.contains("velvet-mark"),
            "lockup must embed the emblem asset"
        );
        assert!(
            html.contains("currentColor"),
            "wordmark text must inherit currentColor"
        );
        assert!(
            !html.contains(">VAELVET<"),
            "no hardcoded uppercase text brand"
        );
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
        assert!(html.contains("href=\"/achivements\""));
        assert!(html.contains("All case studies"));
    }
}
