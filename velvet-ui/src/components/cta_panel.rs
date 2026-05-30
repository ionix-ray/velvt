//! CTA panel — final call to action before the footer.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CtaPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "contact",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-cta__inner v-reveal",
                        span { class: "v-eyebrow", "Get Started" }
                        h2 { class: "v-display-2", "{site.cta.title}" }
                        p { class: "v-cta__body", "{site.cta.body}" }
                        div { class: "v-btn-group",
                            a {
                                class: "v-btn v-btn--primary",
                                href: "mailto:{site.contact.email_general}",
                                span { "{site.cta.btn_primary}" }
                                span { class: "v-btn__arrow", "→" }
                            }
                            a {
                                class: "v-btn v-btn--outline",
                                href: "mailto:{site.contact.email_press}",
                                span { "{site.cta.btn_secondary}" }
                            }
                        }
                        div { class: "v-cta__contact-detail",
                            span { class: "v-cta__contact-cta", "{site.contact.cta}" }
                            a {
                                href: "mailto:{site.contact.email_general}",
                                class: "v-cta__contact-email",
                                "{site.contact.email_general}"
                            }
                        }
                    }
                }
            }
        }
    }
}
