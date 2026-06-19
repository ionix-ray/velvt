//! Contact — two-column. CTA + lines (email, ateliers).

use crate::Site;
use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn Contact(site: Site) -> Element {
    rsx! {
        section { id: "contact", class: "v-section",
            div { class: "v-container",
                header { class: "v-section__head v-reveal",
                    span { class: "v-eyebrow", "Begin" }
                    h2 { class: "v-display-2", "Walk in already arrived." }
                    div { class: "v-divider" }
                }
                div { class: "v-contact v-reveal",
                    div {
                        p { class: "v-contact__cta", "{site.contact.cta}" }
                    }
                    div {
                        a { class: "v-contact__line", href: "mailto:{site.contact.email_general}",
                            Icon { name: "email".to_string() }
                            span { "{site.contact.email_general}" }
                        }
                        a { class: "v-contact__line", href: "mailto:{site.contact.email_press}",
                            Icon { name: "email".to_string() }
                            span { "Press · {site.contact.email_press}" }
                        }
                        for a in site.contact.ateliers.iter() {
                            div { class: "v-contact__line",
                                Icon { name: "location".to_string() }
                                span { "{a}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
