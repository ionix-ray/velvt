//! Footer panel — the 7th horizontal scroll panel.
//! Contains the full Velvt footer: brand description, service columns,
//! contact details, social links, and copyright. Appears after the last
//! content panel, giving the "rolling reel" effect the user requested.

use crate::Site;
use crate::components::icons::{Icon, pretty_label};
use crate::theme::wordmark::Wordmark;
use dioxus::prelude::*;

#[component]
pub fn FooterPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel v-panel--footer", id: "footer",
            div { class: "v-footer-panel",
                // ── Main Top Row: Brand & Columns ────────────────────────
                div { class: "v-footer-panel__main",
                    div { class: "v-footer-panel__brand",
                    div { class: "v-logo-wrap",
                        Wordmark {
                            class: "v-logo-svg v-footer-panel__wordmark".to_string(),
                            title: Some(site.brand.name.to_string()),
                        }
                        div { class: "v-logo-sheen", "aria-hidden": "true" }
                    }
                    p { class: "v-footer-panel__tagline", "{site.brand.tagline}" }
                    p { class: "v-footer-panel__desc", "{site.footer.brand_desc}" }

                    // Social icons (SVG via shared Icon component)
                    div { class: "v-footer-panel__socials",
                        for social in site.footer.socials.iter() {
                            a {
                                href: "{social.href}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "v-footer-panel__social-btn",
                                aria_label: "Velvt on {pretty_label(&social.label)}",
                                Icon { name: social.label.to_string() }
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
                    }
                } // End v-footer-panel__main

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

                // ── Giant brand mark at the bottom ────────────────────────
                div { class: "v-footer-panel__giant-brand",
                    "VELVT"
                }
            }
        }
    }
}
