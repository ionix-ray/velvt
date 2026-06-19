//! Work With Us — contact/inquiry section.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn WorkWithUs(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "inquiry",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-wwu v-reveal",
                        div { class: "v-wwu__content",
                            span { class: "v-eyebrow", "Work With Us" }
                            h2 { class: "v-display-2", "Ready to Start Your Next Campaign?" }
                            p { class: "v-body",
                                "Tell us about your vision. Our team will craft a bespoke experience that elevates your brand."
                            }
                        }
                        div { class: "v-wwu__form",
                            a { class: "v-btn v-btn--primary", href: "mailto:{site.contact.email_general}",
                                span { "Send us a message" }
                                span { class: "v-btn__arrow", "\u{2192}" }
                            }
                            div { class: "v-wwu__details",
                                div { class: "v-wwu__detail",
                                    span { class: "v-wwu__detail-label", "Email" }
                                    a { href: "mailto:{site.contact.email_general}", "{site.contact.email_general}" }
                                }
                                div { class: "v-wwu__detail",
                                    span { class: "v-wwu__detail-label", "Press" }
                                    a { href: "mailto:{site.contact.email_press}", "{site.contact.email_press}" }
                                }
                                for a in site.contact.ateliers.iter() {
                                    div { class: "v-wwu__detail",
                                        span { class: "v-wwu__detail-label", "Studio" }
                                        span { "{a}" }
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
