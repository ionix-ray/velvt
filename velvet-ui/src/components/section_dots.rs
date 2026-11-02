//! Section navigation — 3D Spindle wheel.
//! Pure CSS-transition driven — no JS coroutine or gloo_timers.
//! The active page is in the middle, prev above (rotated away), next below.
//! Labels derive from `site.nav` (single source of truth in content/site.md)
//! with "Footer" appended for the 7th scroll panel.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn SectionDots(site: Site, current: usize, on_navigate: EventHandler<usize>) -> Element {
    // Nav labels single-sourced from site.nav; "Footer" is the 7th panel
    // and has no entry in site.nav, so it is appended here.
    let labels: Vec<String> = site
        .nav
        .iter()
        .map(|n| n.label.to_string())
        .chain(std::iter::once("Footer".to_string()))
        .collect();
    let count = labels.len();

    let states: Vec<_> = (0..count)
        .map(|i| {
            let is_active = i == current;
            let style = if is_active {
                "opacity: 1;"
            } else {
                "opacity: 0.5;"
            };
            let label = labels.get(i).map(|l| l.as_str()).unwrap_or("");
            (i, is_active, style, label)
        })
        .collect();

    rsx! {
        nav {
            class: "v-spindle",
            "aria-label": "Page navigation",
            for (i, is_active, style, label) in states {
                button {
                    class: if is_active { "v-spindle-item active" } else { "v-spindle-item" },
                    style: "{style}",
                    "aria-current": if is_active { "page" } else { "false" },
                    onclick: move |_| on_navigate.call(i),
                    span { class: "v-spindle-item__label",
                        span { class: "v-spindle-item__text", "{label}" }
                    }
                }
            }
        }
    }
}
