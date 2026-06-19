//! Services section — Carbon 3-col grid, rim-light on hover.

use crate::Site;
use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn Services(site: Site) -> Element {
    rsx! {
        section { id: "services", class: "v-section",
            div { class: "v-container",
                header { class: "v-section__head v-reveal",
                    span { class: "v-eyebrow", "What we do" }
                    h2 { class: "v-display-2", "Five disciplines. One craft." }
                    div { class: "v-divider" }
                }
                div { class: "v-services",
                    for s in site.services.iter() {
                        article { class: "v-card v-reveal",
                            span { class: "v-card__icon", Icon { name: s.icon.clone() } }
                            h3   { class: "v-h3 v-card__title", "{s.title}" }
                            p    { class: "v-card__body", "{s.body}" }
                        }
                    }
                }
            }
        }
    }
}
