//! Studio panel — event showcase masonry.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn StudioPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "showcase",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "Event Showcase" }
                        h2 { class: "v-display-2", "{site.studio.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.studio.sub}"
                        }
                    }
                    div { class: "v-masonry",
                        for (i, item) in site.studio.items.iter().enumerate() {
                            div { class: "v-masonry__item v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
                                div { class: "v-masonry__content",
                                    span { class: "v-masonry__tag", "{item.tag}" }
                                    h4 { "{item.title}" }
                                    p { "{item.body}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_delay(i: usize) -> String {
    format!("{}", (i + 1) * 80)
}
