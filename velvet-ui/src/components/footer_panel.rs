//! Footer panel — vertical layout with brand, columns, socials.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn FooterPanel(site: Site) -> Element {
    rsx! {
        footer { class: "v-panel",
            div { class: "v-footer",
                div { class: "v-container",
                    div { class: "v-footer__inner",
                        div { class: "v-footer__brand",
                            img { class: "v-footer__brand-name", src: asset!("/assets/images/velvt-logo.png"), alt: "VAELVET" }
                            p { "{site.footer.brand_desc}" }
                        }
                        div { class: "v-footer__cols",
                            for col in site.footer.columns.iter() {
                                div { class: "v-footer__col",
                                    h4 { "{col.title}" }
                                    for link in col.links.iter() {
                                        a { href: "{link.href}", "{link.label}" }
                                    }
                                }
                            }
                        }
                        div { class: "v-footer__socials",
                            for social in site.footer.socials.iter() {
                                a { href: "{social.href}", target: "_blank", rel: "noopener",
                                    "{social.label}"
                                }
                            }
                        }
                    }
                    div { class: "v-footer__bottom",
                        span { "{site.brand.copyright}" }
                    }
                }
            }
        }
    }
}
