//! Process panel — 5-step workflow.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn ProcessPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "process",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "How We Work" }
                        h2 { class: "v-display-2", "{site.process.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.process.sub}"
                        }
                    }
                    div { class: "v-process",
                        for (i, step) in site.process.steps.iter().enumerate() {
                            div { class: "v-process__step v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
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

fn format_delay(i: usize) -> String {
    format!("{}", (i + 1) * 80)
}
