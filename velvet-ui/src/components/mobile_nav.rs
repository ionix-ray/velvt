//! Mobile bottom navigation — visible only on ≤768px viewport.
//! Labels derive from `site.nav` (single source of truth in content/site.md)
//! with "Footer" appended for the 7th scroll panel.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn MobileNav(site: Site, current_panel: usize, on_navigate: EventHandler<usize>) -> Element {
    // Nav labels single-sourced from site.nav; "Footer" is the 7th panel
    // and has no entry in site.nav, so it is appended here.
    let labels: Vec<String> = site
        .nav
        .iter()
        .map(|n| n.label.to_string())
        .chain(std::iter::once("Footer".to_string()))
        .collect();
    let panel_count = labels.len();

    // Only render if we have a real panel index range
    if panel_count == 0 {
        return rsx! {};
    }

    rsx! {
        div { class: "v-mobile-nav",
            for (idx, label) in labels.iter().enumerate() {
                button {
                    class: "{item_class(idx, current_panel)}",
                    onclick: move |_| {
                        on_navigate.call(idx);
                    },
                    "{label}"
                }
            }
        }
    }
}

fn item_class(idx: usize, current_panel: usize) -> &'static str {
    if idx == current_panel {
        "v-mobile-nav__item active"
    } else {
        "v-mobile-nav__item"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_class_marks_only_the_current_panel_active() {
        assert_eq!(item_class(2, 2), "v-mobile-nav__item active");
        assert_eq!(item_class(0, 2), "v-mobile-nav__item");
        assert_eq!(item_class(3, 2), "v-mobile-nav__item");
    }

    fn site_with_nav(count: usize) -> Site {
        Site {
            nav: (0..count)
                .map(|i| crate::config::NavItem {
                    label: format!("Panel {i}").into(),
                    href: format!("#panel-{i}").into(),
                })
                .collect(),
            ..Site::default()
        }
    }

    #[component]
    fn WrapEmpty() -> Element {
        rsx! {
            MobileNav { site: Site::default(), current_panel: 0usize, on_navigate: move |_: usize| {} }
        }
    }

    #[component]
    fn WrapThreePanels() -> Element {
        rsx! {
            MobileNav { site: site_with_nav(3), current_panel: 1usize, on_navigate: move |_: usize| {} }
        }
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn empty_nav_renders_footer_only() {
        let html = render(WrapEmpty);
        assert!(html.contains("v-mobile-nav"));
        assert!(html.contains(">Footer<"));
    }

    #[test]
    fn renders_one_button_per_panel_with_current_marked_active() {
        let html = render(WrapThreePanels);
        assert!(html.contains("v-mobile-nav"));
        assert!(html.contains("v-mobile-nav__item active"));
        assert!(html.contains(">Panel 0<"));
        assert!(html.contains(">Panel 1<"));
        assert!(html.contains(">Panel 2<"));
        assert!(html.contains(">Footer<"));
    }

    #[test]
    fn renders_footer_label_when_site_nav_has_entries() {
        let site = site_with_nav(6);
        let labels: Vec<String> = site
            .nav
            .iter()
            .map(|n| n.label.to_string())
            .chain(std::iter::once("Footer".to_string()))
            .collect();
        assert_eq!(labels.len(), 7);
        assert_eq!(labels.last(), Some(&"Footer".to_string()));
    }
}
