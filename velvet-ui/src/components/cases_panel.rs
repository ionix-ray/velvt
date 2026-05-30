//! Cases panel — 3 case study cards.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CasesPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "cases",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "Success Stories" }
                        h2 { class: "v-display-2", "{site.cases.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.cases.sub}"
                        }
                    }
                    div { class: "v-cases",
                        for (i, case) in site.cases.items.iter().enumerate() {
                            div { class: "v-case-card v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
                                div { class: "v-case-card__visual",
                                    div { class: "v-case-card__metric", "{case.metric}" }
                                }
                                div { class: "v-case-card__body",
                                    div { class: "v-case-card__client", "{case.client}" }
                                    p { class: "v-case-card__desc", "{case.desc}" }
                                    div { class: "v-tags",
                                        for tag in case.tags.iter() {
                                            span { class: "v-tag", "{tag}" }
                                        }
                                    }
                                }
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
