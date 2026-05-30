//! Social panel — social media engagement grid.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn SocialPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "social",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-reveal", style: "text-align: center; margin-bottom: 2rem;",
                        span { class: "v-eyebrow", "Social Proof" }
                        h2 { class: "v-display-2", "{site.social.title}" }
                        p { class: "v-body", style: "margin-inline: auto;",
                            "{site.social.sub}"
                        }
                    }
                    div { class: "v-social",
                        for (i, item) in site.social.items.iter().enumerate() {
                            div { class: "v-social__post v-reveal",
                                style: "transition-delay: {format_delay(i)}ms;",
                                span { class: "v-social__icon",
                                    "{social_icon(&item.icon)}"
                                }
                                span { class: "v-social__label", "{item.label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn social_icon(icon: &str) -> &str {
    match icon {
        "reel" => "\u{25B6}",
        "heart" => "\u{2764}",
        "music" => "\u{1F3B5}",
        "comment" => "\u{1F4AC}",
        "chart" => "\u{1F4C8}",
        _ => "\u{25CF}",
    }
}

fn format_delay(i: usize) -> String {
    format!("{}", (i + 1) * 80)
}
