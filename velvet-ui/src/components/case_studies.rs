//! Case studies — cinematic slabs, two-column on wide viewports.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CaseStudies(site: Site) -> Element {
    rsx! {
        section { id: "cases", class: "v-section",
            div { class: "v-container",
                header { class: "v-section__head v-reveal",
                    span { class: "v-eyebrow", "Selected work" }
                    h2 { class: "v-display-2", "Quiet campaigns. Loud outcomes." }
                    div { class: "v-divider" }
                }
                div { class: "v-cases",
                    for c in site.cases.iter() {
                        article { class: "v-case v-reveal",
                            div {
                                p { class: "v-case__client", "{c.client}" }
                                p { class: "v-case__metric", "{c.metric}" }
                            }
                            p { class: "v-case__sum v-body", "{c.summary}" }
                        }
                    }
                }
            }
        }
    }
}
