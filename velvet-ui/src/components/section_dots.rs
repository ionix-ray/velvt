//! Section navigation — camera-reel counter dial.
//! Rust-driven animation: items slide vertically like film-advance frames.
//! The active page locks into centre as a tag; the reel rotates mechanically.

use dioxus::prelude::*;

#[component]
pub fn SectionDots(
    count: usize,
    current: usize,
    labels: Vec<String>,
    on_navigate: EventHandler<usize>,
) -> Element {
    let mut animated = use_signal(|| current as f64);
    let mut prev = use_signal(|| current);
    let mut generation = use_signal(|| 0u64);

    use_effect(move || {
        let old = *prev.read();
        if old == current {
            return;
        }
        prev.set(current);

        let start_val: f64 = *animated.read();
        let end_val: f64 = current as f64;
        let range: f64 = end_val - start_val;
        if range.abs() < f64::EPSILON {
            return;
        }

        let this_gen = *generation.read() + 1;
        generation.set(this_gen);

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(perf) = window.performance() else {
            return;
        };

        dioxus::prelude::spawn(async move {
            let start: f64 = perf.now();
            let duration: f64 = 200.0;

            loop {
                if *generation.read() != this_gen {
                    break;
                }

                let elapsed: f64 = perf.now() - start;
                let t: f64 = (elapsed / duration).min(1.0);
                let eased: f64 = 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t) * (1.0 - t);
                animated.set(start_val + range * eased);

                if t >= 1.0 {
                    break;
                }
                gloo_timers::future::TimeoutFuture::new(16).await;
            }
        });
    });

    let anim: f64 = *animated.read();

    let states: Vec<_> = (0..count)
        .map(|i| {
            let dy = i as f64 - anim;
            let dist = dy.abs();
            let pe = if dist * 0.35 > 0.95 {
                "pointer-events:none;"
            } else {
                ""
            };
            let style = if i == current && dist < 0.01 {
                String::new()
            } else {
                let y = dy * 32.0;
                let scale = (1.0 - dist * 0.3).max(0.1);
                let op = (1.0 - dist * 0.35).max(0.0);
                format!("transform:translateY({y:.1}px)scale({scale:.2});opacity:{op:.2};{pe}")
            };
            let is_active = i == current;
            (i, is_active, style)
        })
        .collect();

    rsx! {
        nav { class: "v-dots", "aria-label": "Section navigation",
            for (i, is_active, style) in states {
                button {
                    class: if is_active { "v-dot active" } else { "v-dot" },
                    style: "{style}",
                    "aria-current": if is_active { "page" } else { "false" },
                    onclick: move |_| on_navigate.call(i),
                    span { class: "v-dot__label",
                        "{labels.get(i).map_or(\"\", |l| l.as_str())}"
                    }
                }
            }
        }
    }
}
