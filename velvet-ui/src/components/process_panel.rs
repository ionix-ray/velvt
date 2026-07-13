//! Process panel — 5-step workflow, compact to fit 100vh.

use crate::Site;
use crate::components::card_step::CardStep;
use dioxus::prelude::*;

#[component]
pub fn ProcessPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "ideology",
            div { class: "v-cinematic-bg" }
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-panel-header v-reveal",
                        if !site.process.eyebrow.is_empty() {
                            span { class: "v-eyebrow", "{site.process.eyebrow}" }
                        }
                        h2 { class: "v-display-2", "{site.process.title}" }
                        p { class: "v-panel-header__sub", "{site.process.sub}" }
                    }
                    div { class: "v-process",
                        for (i, step) in site.process.steps.iter().enumerate() {
                            CardStep {
                                logo: site.hero.logo.to_string(),
                                num: Some(step.num.to_string()),
                                title: step.title.to_string(),
                                body: step.body.to_string(),
                                delay_ms: (i + 1) * 120,
                            }
                        }
                    }
                }
            }
        }
    }
}
