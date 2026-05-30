//! Story panel — agency pillars / about.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn StoryPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "about",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "About Us" }
                        h2 { class: "v-display-2", "{site.story.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.story.sub}"
                        }
                    }
                    div { class: "v-timeline",
                        for (i, item) in site.story.items.iter().enumerate() {
                            div { class: "v-timeline__item v-reveal",
                                if i % 2 == 0 {
                                    div { class: "v-timeline__year", "{item.year}" }
                                    div { class: "v-timeline__content",
                                        h3 { "{item.title}" }
                                        p { "{item.body}" }
                                    }
                                } else {
                                    div { class: "v-timeline__content",
                                        h3 { "{item.title}" }
                                        p { "{item.body}" }
                                    }
                                    div { class: "v-timeline__year", "{item.year}" }
                                }
                                div { class: "v-timeline__dot" }
                            }
                        }
                    }
                }
            }
        }
    }
}
