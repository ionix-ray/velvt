//! Cases panel — client case study cards (compact, fit within 100vh).
//! Cards use v-card-modern with smaller height image sections.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CasesPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "cases",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-panel-header v-reveal",
                        span { class: "v-eyebrow", "Showcase" }
                        h2 { class: "v-display-2", "{site.cases.title}" }
                        p { class: "v-panel-header__sub", "{site.cases.sub}" }
                    }
                    div { class: "v-cases-grid",
                        for (i, case) in site.cases.items.iter().enumerate() {
                            div {
                                class: "v-card-modern v-reveal",
                                style: "transition-delay: {(i + 1) * 80}ms;",
                                div {
                                    class: "v-card-modern__image",
                                    style: "background-image: url('{case.bg_image}');"
                                }
                                div { class: "v-card-modern__content",
                                    img {
                                        class: "v-card-modern__logo",
                                        src: "{case.logo_image}",
                                        alt: "{case.client} logo"
                                    }
                                    div { class: "v-card-modern__client", "{case.client}" }
                                    div { class: "v-card-modern__metric", "{case.metric}" }
                                    div { class: "v-tags",
                                        for tag in case.tags.iter() {
                                            span { class: "v-tag--green", "{tag}" }
                                        }
                                    }
                                    p { class: "v-card-modern__desc", "{case.desc}" }
                                    a {
                                        class: "v-btn-glow",
                                        href: "{case.button_link}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "View Case Study"
                                    }
                                }
                                div { class: "v-card-modern__footer", "{case.footer_label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
