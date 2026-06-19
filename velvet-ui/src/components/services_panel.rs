//! Services panel — portfolio capabilities.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn ServicesPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "portfolio",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-services__grid",
                        div { class: "v-reveal-left",
                            span { class: "v-eyebrow", "Portfolio" }
                            h2 { class: "v-display-2", "{site.services.title}" }
                            p { class: "v-body", style: "margin-bottom: 1.5rem;",
                                "{site.services.sub}"
                            }
                            for item in site.services.items.iter() {
                                div { class: "v-service-item",
                                    div { class: "v-service-item__header",
                                        span { class: "v-service-item__num", "{item.num}" }
                                        h3 { "{item.title}" }
                                    }
                                    p { "{item.body}" }
                                }
                            }
                        }
                        div { class: "v-reveal-right",
                            div { class: "v-service-card",
                                div { class: "v-service-card__icon",
                                    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
                                        path { d: "M6 16h20M20 10l6 6-6 6" }
                                    }
                                }
                                h3 { class: "v-h3", "End-to-End Excellence" }
                                p { class: "v-body", "Events × Talent × Creativity × Results" }
                            }
                        }
                    }
                }
            }
        }
    }
}
