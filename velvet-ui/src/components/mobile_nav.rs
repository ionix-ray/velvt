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
                    class: if idx == current_panel { "v-mobile-nav__item active" } else { "v-mobile-nav__item" },
                    onclick: move |_| {
                        on_navigate.call(idx);
                    },
                    "{label}"
                }
            }
        }
    }
}
