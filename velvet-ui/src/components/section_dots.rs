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
    // Compute per-item transform/opacity purely in Rust.
    // CSS `transition` handles smooth interpolation — zero JS animation.
    let states: Vec<_> = (0..count)
        .map(|i| {
            let dy = i as f64 - current as f64;
            let dist = dy.abs();
            let is_active = i == current;

            // 22 degrees per step on the reel — gentler tilt so far items
            // still read as legible labels rather than edge-on slivers.
            let angle = -dy * 22.0;
            // Slow fade with a visibility floor so every section name stays
            // readable from any panel — the dial is the site map.
            let opacity = (1.0 - dist * 0.16).max(0.45);
            // All items remain clickable; the dial is the primary
            // discoverability surface for what the site has to offer.
            let pe = "";

            let style = if is_active {
                // Active item sits flat (no rotation)
                format!("opacity:1;transform:rotateX(0deg);{pe}")
            } else {
                format!("transform:rotateX({angle:.1}deg);opacity:{opacity:.2};{pe}")
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
                        if let Some((num, text)) = labels.get(i)
                            .map(|l| l.as_str())
                            .unwrap_or("")
                            .split_once(' ')
                        {
                            span { class: "v-spindle-item__num", "{num}" }
                            span { class: "v-spindle-item__text", "{text}" }
                        } else {
                            "{labels.get(i).map(|l| l.as_str()).unwrap_or(\"\")}"
                        }
                    }
                }
            }
        }
    }
}
