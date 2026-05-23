//! Manifesto — centered statement on a velvet radial wash.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn Manifesto(site: Site) -> Element {
    rsx! {
        section { id: "manifesto", class: "v-section v-manifesto v-reveal",
            div { class: "v-container",
                span { class: "v-eyebrow", "Manifesto" }
                h2 { class: "v-display-2 v-manifesto__title", "{site.manifesto.title}" }
                div { class: "v-divider" }
                p { class: "v-manifesto__body", "{site.manifesto.body}" }
            }
        }
    }
}
