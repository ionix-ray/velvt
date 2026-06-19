//! Scroll progress bar — thin crimson line at top.

use dioxus::prelude::*;

#[component]
pub fn ScrollProgress(progress: f64) -> Element {
    let style = format!("width: {}%", (progress * 100.0).min(100.0));
    rsx! {
        div { class: "v-progress", style: "{style}" }
    }
}
