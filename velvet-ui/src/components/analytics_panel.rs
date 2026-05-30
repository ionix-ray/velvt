//! Analytics panel — success metrics / stats.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn AnalyticsPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "stories",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "By the Numbers" }
                        h2 { class: "v-display-2", "{site.analytics.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.analytics.sub}"
                        }
                    }
                    div { class: "v-dashboard",
                        div { class: "v-stats-grid",
                            for (i, stat) in site.analytics.stats.iter().enumerate() {
                                div { class: "v-stat-mini v-reveal",
                                    style: "transition-delay: {format_delay(i)}ms;",
                                    div { class: "v-stat-mini__value", "{stat.value}" }
                                    div { class: "v-stat-mini__label", "{stat.label}" }
                                    div { class: "v-stat-mini__change", "{stat.change}" }
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
