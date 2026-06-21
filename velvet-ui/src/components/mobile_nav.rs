//! Mobile bottom navigation — visible only on ≤768px viewport.

use dioxus::prelude::*;

const LABELS: &[&str] = &[
    "Home",
    "About",
    "Stories",
    "Showcase",
    "Portfolio",
    "Contact",
    "Footer",
];

#[component]
pub fn MobileNav(
    current_panel: usize,
    panel_count: usize,
    on_navigate: EventHandler<usize>,
) -> Element {
    // Only render if we have a real panel index range
    if panel_count == 0 {
        return rsx! {};
    }

    rsx! {
        div { class: "v-mobile-nav",
            for (idx, label) in LABELS.iter().enumerate().take(panel_count) {
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

    #[component]
    fn WrapEmpty() -> Element {
        rsx! {
            MobileNav { current_panel: 0usize, panel_count: 0usize, on_navigate: move |_: usize| {} }
        }
    }

    #[component]
    fn WrapThreePanels() -> Element {
        rsx! {
            MobileNav { current_panel: 1usize, panel_count: 3usize, on_navigate: move |_: usize| {} }
        }
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn zero_panel_count_renders_nothing() {
        assert_eq!(render(WrapEmpty), "");
    }

    #[test]
    fn renders_one_button_per_panel_with_current_marked_active() {
        let html = render(WrapThreePanels);
        assert!(html.contains("v-mobile-nav"));
        assert!(html.contains("v-mobile-nav__item active"));
        assert!(html.contains(">Home<"));
        assert!(html.contains(">About<"));
        assert!(html.contains(">Stories<"));
        assert!(!html.contains(">Showcase<"));
    }
}
