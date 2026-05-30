//! CTA panel — final call to action.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CtaPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "contact",
            div { class: "v-cta",
                div { class: "v-container",
                    div { class: "v-cta__inner v-reveal",
                        h2 { class: "v-display-2",
                            "{site.cta.title}"
                        }
                        p { "{site.cta.body}" }
                        div { class: "v-btn-group",
                            a { class: "v-btn v-btn--primary", href: "#contact",
                                span { "{site.cta.btn_primary}" }
                                span { class: "v-btn__arrow", "\u{2192}" }
                            }
                            a { class: "v-btn v-btn--outline", href: "#contact",
                                span { "{site.cta.btn_secondary}" }
                            }
                            a { class: "v-btn v-btn--ghost", href: "#contact",
                                span { "{site.cta.btn_ghost}" }
                                span { class: "v-btn__arrow", "\u{2192}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
