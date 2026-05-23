//! Footer — credits, links, socials.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn Footer(site: Site) -> Element {
    rsx! {
        footer { class: "v-footer",
            div { class: "v-container v-footer__row",
                p { class: "v-footer__copy", "{site.brand.copyright}" }
                nav { class: "v-footer__links", aria_label: "Footer",
                    for l in site.footer.links.iter() {
                        a { class: "v-nav__link", href: "{l.href}", "{l.label}" }
                    }
                    for s in site.footer.socials.iter() {
                        a {
                            class: "v-nav__link",
                            href: "{s.href}",
                            rel: "noopener noreferrer",
                            target: "_blank",
                            "{s.label}"
                        }
                    }
                }
            }
        }
    }
}
