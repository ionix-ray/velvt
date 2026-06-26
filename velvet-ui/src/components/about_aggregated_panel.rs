//! About Aggregated Panel — Story, Analytics, founder card. All copy and
//! the founder's bio, photo path, and social handles read from
//! `content/site.md` so a non-engineer maintainer can swap them without
//! touching Rust.

use crate::Site;
use crate::components::icons::Icon;
use crate::config::Founder;
use dioxus::prelude::*;

#[component]
pub fn AboutAggregatedPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "about",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-about-grid",
                        // ── Left: Story ────────────────────────────────────────
                        div { class: "v-about-grid__story",
                            span { class: "v-eyebrow", "About Velvt" }
                            h2 { class: "v-display-2 v-about-grid__title",
                                "{site.story.title}"
                            }
                            p { class: "v-about-grid__sub", "{site.story.sub}" }

                            // Pillar list (replaces complex timeline for compact fit)
                            div { class: "v-pillars",
                                for item in site.story.items.iter() {
                                    div { class: "v-pillar",
                                        span { class: "v-pillar__num", "{item.year}" }
                                        div { class: "v-pillar__content",
                                            h4 { class: "v-pillar__title", "{item.title}" }
                                            p { class: "v-pillar__body", "{item.body}" }
                                        }
                                    }
                                }
                            }

                            // Founder — config-driven (see content/site.md `[founder]`).
                            FounderCard { founder: site.founder.clone() }
                        }

                        // ── Right: Stats (bento) ──────────────────────────────
                        div { class: "v-about-grid__stats",
                            span { class: "v-eyebrow", "By the Numbers" }
                            h3 { class: "v-about-grid__stats-title",
                                "{site.analytics.title}"
                            }
                            div { class: "v-about-stats",
                                for stat in site.analytics.stats.iter() {
                                    div { class: "v-about-stat",
                                        div { class: "v-about-stat__value", "{stat.value}" }
                                        div { class: "v-about-stat__label", "{stat.label}" }
                                        span { class: "v-tag--green", "{stat.change}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FounderCard(founder: Founder) -> Element {
    // Empty name means the maintainer hasn't filled in the founder block
    // yet — skip the section rather than render an empty card.
    if founder.name.is_empty() {
        return rsx! {};
    }
    let has_photo = !founder.photo.is_empty();
    let monogram = if founder.monogram.is_empty() {
        founder
            .name
            .chars()
            .next()
            .map_or("•".to_string(), |c| c.to_uppercase().collect::<String>())
    } else {
        founder.monogram.to_string()
    };
    rsx! {
        div { class: "v-founder", id: "founder",
            if has_photo {
                img {
                    class: "v-founder__photo",
                    src: "{founder.photo}",
                    alt: "{founder.name} — founder of Velvt",
                    loading: "eager",
                    width: "200",
                    height: "200",
                }
            } else {
                div {
                    class: "v-founder__photo v-founder__photo--placeholder",
                    role: "img",
                    aria_label: "{founder.name} — founder of Velvt (portrait placeholder)",
                    "{monogram}"
                }
            }
            div { class: "v-founder__body",
                if !founder.eyebrow.is_empty() {
                    span { class: "v-founder__eyebrow", "{founder.eyebrow}" }
                }
                h3 { class: "v-founder__name", "{founder.name}" }
                if !founder.bio.is_empty() {
                    p { class: "v-founder__bio", "{founder.bio}" }
                }
                if !founder.handles.is_empty() {
                    div { class: "v-founder__handles",
                        for handle in founder.handles.iter() {
                            a {
                                class: "v-founder__handle",
                                href: "{handle.href}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                aria_label: "{founder.name} on {handle.icon}",
                                Icon { name: handle.icon.to_string() }
                                span { "{handle.label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
