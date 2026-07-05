//! Stacked right-side navigation panel.

use crate::Site;
use crate::theme::brand::brand_mark;
use dioxus::prelude::*;

#[component]
pub fn StackedNav(
    open: bool,
    current_panel: usize,
    site: Site,
    on_navigate: EventHandler<usize>,
) -> Element {
    let class = if open {
        "v-stack-nav open"
    } else {
        "v-stack-nav"
    };
    rsx! {
        nav { class: "{class}", aria_label: "Section navigation",
            div { class: "v-stack-nav__brand",
                img { src: brand_mark(), alt: "VELVT" }
            }
            for (i, item) in site.nav.iter().enumerate() {
                button {
                    class: if i == current_panel { "v-stack-nav__item active" } else { "v-stack-nav__item" },
                    onclick: move |_| on_navigate.call(i),
                    span { class: "v-stack-nav__text", "{item.label}" }

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Site;

    #[component]
    fn WrapNav() -> Element {
        rsx! {
            StackedNav {
                open: false,
                current_panel: 0,
                site: Site::load().clone(),
                on_navigate: EventHandler::new(|_: usize| {}),
            }
        }
    }

    #[component]
    fn WrapNavOpen() -> Element {
        rsx! {
            StackedNav {
                open: true,
                current_panel: 2,
                site: Site::load().clone(),
                on_navigate: EventHandler::new(|_: usize| {}),
            }
        }
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn stacked_nav_renders_nav_items() {
        let html = render(WrapNav);
        assert!(html.contains("v-stack-nav"));
        assert!(html.contains("Home"));
        assert!(html.contains("About"));
    }

    #[test]
    fn stacked_nav_open_has_open_class() {
        let html = render(WrapNavOpen);
        assert!(html.contains("v-stack-nav open"));
    }

    #[test]
    fn stacked_nav_closed_does_not_have_open_class() {
        let html = render(WrapNav);
        assert!(!html.contains("v-stack-nav open"));
    }

    #[test]
    fn stacked_nav_shows_active_panel() {
        let html = render(WrapNavOpen);
        assert!(html.contains("active"));
    }
}
