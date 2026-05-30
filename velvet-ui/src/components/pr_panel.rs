//! PR panel — press coverage gallery.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn PrPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "pr",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "Press & Media" }
                        h2 { class: "v-display-2", "{site.pr.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.pr.sub}"
                        }
                    }
                    div { class: "v-pr-grid",
                        for (i, item) in site.pr.items.iter().enumerate() {
                            div { class: "v-pr-item v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
                                div { class: "v-pr-item__source", "{item.source}" }
                                div { class: "v-pr-item__overlay",
                                    h4 { "{item.title}" }
                                    p { "{item.body}" }
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
