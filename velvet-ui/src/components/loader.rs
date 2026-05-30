//! Loading screen — classic progress bar with brand name.

use dioxus::prelude::*;

#[component]
pub fn Loader(hidden: bool) -> Element {
    let class = if hidden {
        "v-loader hidden"
    } else {
        "v-loader"
    };
    rsx! {
        div { class: "{class}",
            div { class: "v-loader__brand",
                img { class: "v-loader__logo", src: asset!("/assets/images/velvt-logo.png"), alt: "VAELVET" }
            }
            div { class: "v-loader__bar",
                div { class: "v-loader__fill" }
            }
        }
    }
}
