//! AI panel — interface mockup + feature cards.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn AiPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "ai",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "AI-Powered" }
                        h2 { class: "v-display-2", "{site.ai.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.ai.sub}"
                        }
                    }
                    div { class: "v-ai__grid",
                        div { class: "v-ai__interface v-reveal",
                            div { class: "v-ai__header",
                                div { class: "v-ai__avatar", "AI" }
                                div {
                                    div { class: "v-ai__name", "Vaelvet Assistant" }
                                    div { class: "v-ai__status", "\u{25CF} Online - Ready to help" }
                                }
                            }
                            div { class: "v-ai__messages",
                                div { class: "v-ai__msg v-ai__msg--bot",
                                    "Hello! I can analyze your campaign data, suggest optimizations, and predict performance. How can I help?"
                                }
                                div { class: "v-ai__msg v-ai__msg--user",
                                    "Show me insights for the current campaign"
                                }
                                div { class: "v-ai__msg v-ai__msg--bot",
                                    "Your campaign is performing 34% above benchmark. I recommend increasing the budget allocation to video ads — they're driving 2.8x more engagement."
                                }
                            }
                        }
                        div { class: "v-ai__features v-reveal-right",
                            for feature in site.ai.features.iter() {
                                div { class: "v-ai__feature",
                                    div { class: "v-ai__feature-icon",
                                        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
                                            circle { cx: "16", cy: "16", r: "10" }
                                            path { d: "M12 16l3 3 6-6" }
                                        }
                                    }
                                    div {
                                        h4 { "{feature.title}" }
                                        p { "{feature.body}" }
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
