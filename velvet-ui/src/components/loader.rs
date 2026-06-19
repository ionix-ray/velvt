//! Loading screen — retro progress bar with cycling creative text.

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const PHRASES: &[&str] = &[
    "Curating presence…",
    "Composing entrances…",
    "Seating the front row…",
    "Tuning the lights…",
    "Adjusting the lens…",
];

#[component]
pub fn Loader(hidden: bool) -> Element {
    let mut phrase_index = use_signal(|| 0);

    use_effect(move || {
        let index = *phrase_index.read();
        if index >= PHRASES.len().saturating_sub(1) {
            return;
        }
        spawn(async move {
            TimeoutFuture::new(400).await;
            phrase_index.set(index + 1);
        });
    });

    let class = if hidden {
        "v-loader hidden"
    } else {
        "v-loader"
    };

    let phrase = PHRASES.get(*phrase_index.read()).unwrap_or(&"");

    rsx! {
        div { class: "{class}",
            div { class: "v-loader__brand",
                img { class: "v-loader__logo",
                      src: asset!("/assets/images/velvt-logo.png"),
                      alt: "VELVT" }
            }
            div { class: "v-loader__bar",
                div { class: "v-loader__fill" }
            }
            div { class: "v-loader__phrase", key: "{phrase}", "{phrase}" }
        }
    }
}
