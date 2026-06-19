//! About Aggregated Panel — Story, Analytics, and pillars in one panel.
//! Compact layout designed to fit entirely within 100vh without overflow.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn AboutAggregatedPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "about",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-about-grid",
                        // ── Left: Story ────────────────────────────────────────
                        div { class: "v-about-grid__story",
                            span { class: "v-eyebrow", "About Velvt" }
                            h2 { class: "v-display-2 v-about-grid__title",
                                "{site.story.title}"
                            }
                            p { class: "v-about-grid__sub", "{site.story.sub}" }

                            // Pillar list (replaces complex timeline for compact fit)
                            div { class: "v-pillars",
                                for item in site.story.items.iter() {
                                    div { class: "v-pillar",
                                        span { class: "v-pillar__num", "{item.year}" }
                                        div { class: "v-pillar__content",
                                            h4 { class: "v-pillar__title", "{item.title}" }
                                            p { class: "v-pillar__body", "{item.body}" }
                                        }
                                    }
                                }
                            }
                        }

                        // ── Right: Stats ───────────────────────────────────────
                        div { class: "v-about-grid__stats",
                            span { class: "v-eyebrow", "By the Numbers" }
                            h3 { class: "v-about-grid__stats-title",
                                "{site.analytics.title}"
                            }
                            div { class: "v-about-stats",
                                for stat in site.analytics.stats.iter() {
                                    div { class: "v-about-stat",
                                        div { class: "v-about-stat__value", "{stat.value}" }
                                        div { class: "v-about-stat__label", "{stat.label}" }
                                        span { class: "v-tag--green", "{stat.change}" }
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
