//! Client banner — auto-scrolling marquee of trusted partner logos.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn ClientBanner(site: Site) -> Element {
    let logos = &site.client_banner.logos;
    let has_logos = !logos.is_empty();

    rsx! {
        section { class: "v-panel", id: "banner",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal",
                        h2 { class: "v-display-2",
                            "{site.client_banner.title}"
                        }
                    }
                    if has_logos {
                        div { class: "v-banner__marquee",
                            div { class: "v-banner__track",
                                for logo in logos.iter() {
                                    span { class: "v-banner__logo", "{logo}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
