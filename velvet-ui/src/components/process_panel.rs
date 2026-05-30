//! Process panel — 5-step workflow, compact to fit 100vh.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn ProcessPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "stories",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-panel-header v-reveal",
                        span { class: "v-eyebrow", "How We Work" }
                        h2 { class: "v-display-2", "{site.process.title}" }
                        p { class: "v-panel-header__sub", "{site.process.sub}" }
                    }
                    div { class: "v-process",
                        for (i, step) in site.process.steps.iter().enumerate() {
                            div {
                                class: "v-process__step v-reveal",
                                style: "transition-delay: {(i + 1) * 60}ms;",
                                div { class: "v-process__num", "{step.num}" }
                                h4 { "{step.title}" }
                                p { "{step.body}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
