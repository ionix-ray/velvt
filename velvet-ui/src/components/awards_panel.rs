//! Awards panel — recognition cards.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn AwardsPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "awards",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "Recognition" }
                        h2 { class: "v-display-2", "{site.awards.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.awards.sub}"
                        }
                    }
                    div { class: "v-awards",
                        for (i, item) in site.awards.items.iter().enumerate() {
                            div { class: "v-award-card v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
                                span { class: "v-award-card__icon", "\u{1F3C6}" }
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

fn format_delay(i: usize) -> String {
    format!("{}", (i + 1) * 80)
}
