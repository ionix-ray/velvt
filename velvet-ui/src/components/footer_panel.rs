//! Footer panel — the 7th horizontal scroll panel.
//! Contains the full Velvt footer: brand description, service columns,
//! contact details, social links, and copyright. Appears after the last
//! content panel, giving the "rolling reel" effect the user requested.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn FooterPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel v-panel--footer", id: "footer",
            div { class: "v-footer-panel",
                // ── Top: Brand + tagline ─────────────────────────────
                div { class: "v-footer-panel__brand",
                    span { class: "v-footer-panel__name", "{site.brand.name}" }
                    p { class: "v-footer-panel__tagline", "{site.brand.tagline}" }
                    p { class: "v-footer-panel__desc", "{site.footer.brand_desc}" }

                    // Social icons
                    div { class: "v-footer-panel__socials",
                        for social in site.footer.socials.iter() {
                            a {
                                href: "{social.href}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "v-footer-panel__social-btn",
                                aria_label: "Velvt on {social.label}",
                                "{social.label}"
                            }
                        }
                    }
                }

                // ── Middle: Link columns ─────────────────────────────
                div { class: "v-footer-panel__cols",
                    for col in site.footer.columns.iter() {
                        div { class: "v-footer-panel__col",
                            h4 { class: "v-footer-panel__col-title", "{col.title}" }
                            for link in col.links.iter() {
                                a {
                                    href: "{link.href}",
                                    class: "v-footer-panel__col-link",
                                    "{link.label}"
                                }
                            }
                        }
                    }

                    // Contact column from site.contact
                    div { class: "v-footer-panel__col",
                        h4 { class: "v-footer-panel__col-title", "Ateliers" }
                        for atelier in site.contact.ateliers.iter() {
                            span { class: "v-footer-panel__address", "{atelier}" }
                        }
                        a {
                            href: "mailto:{site.contact.email_general}",
                            class: "v-footer-panel__col-link",
                            "{site.contact.email_general}"
                        }
                        a {
                            href: "mailto:{site.contact.email_press}",
                            class: "v-footer-panel__col-link",
                            "{site.contact.email_press}"
                        }
                    }
                }

                // ── Bottom: Legal + copyright ────────────────────────
                div { class: "v-footer-panel__bottom",
                    p { class: "v-footer-panel__copyright", "{site.brand.copyright}" }
                    div { class: "v-footer-panel__legal",
                        a { href: "#", class: "v-footer-panel__legal-link", "Privacy Policy" }
                        span { class: "v-footer-panel__legal-sep", "·" }
                        a { href: "#", class: "v-footer-panel__legal-link", "Terms of Service" }
                        span { class: "v-footer-panel__legal-sep", "·" }
                        a { href: "#", class: "v-footer-panel__legal-link", "Cookie Policy" }
                    }
                }
            }
        }
    }
}
