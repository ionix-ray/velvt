//! Section navigation — 3D Spindle wheel.
//! Pure CSS-transition driven — no JS coroutine or gloo_timers.
//! The active page is in the middle, prev above (rotated away), next below.

use dioxus::prelude::*;

#[component]
pub fn SectionDots(
    count: usize,
    current: usize,
    labels: Vec<String>,
    on_navigate: EventHandler<usize>,
) -> Element {
    let states: Vec<_> = (0..count)
        .map(|i| {
            let is_active = i == current;
            let style = if is_active {
                "opacity: 1;"
            } else {
                "opacity: 0.5;"
            };
            (i, is_active, style)
        })
        .collect();

    rsx! {
        nav {
            class: "v-spindle",
            "aria-label": "Page navigation",
            for (i, is_active, style) in states {
                button {
                    class: if is_active { "v-spindle-item active" } else { "v-spindle-item" },
                    style: "{style}",
                    "aria-current": if is_active { "page" } else { "false" },
                    onclick: move |_| on_navigate.call(i),
                    span { class: "v-spindle-item__label",
                        span { class: "v-spindle-item__text",
                            "{labels.get(i).map(|l| l.as_str()).unwrap_or(\"\")}"
                        }
                    }
                }
            }
        }
    }
}
