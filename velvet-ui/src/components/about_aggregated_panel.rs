//! About Aggregated Panel — Story, Analytics, team cards. All copy and
//! the team's bio, photo path, and social handles read from
//! `content/site.md` so a non-engineer maintainer can swap them without
//! touching Rust.

use crate::Site;
use crate::components::icons::Icon;
use crate::config::TeamMember;
use dioxus::prelude::*;

#[component]
pub fn AboutAggregatedPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "about",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-about-layout",
                        // ── Top Row: Hero Stats, Story, Team ────────────────
                        div { class: "v-about-top-row",

                            // Story
                            div { class: "v-about-grid__story",
                                span { class: "v-eyebrow", "About Velvt" }
                                h2 { class: "v-display-2 v-about-grid__title",
                                    "{site.story.title}"
                                }
                                p { class: "v-about-grid__sub", "{site.story.sub}" }

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
                            }

                            // Team Grid
                            div { class: "v-about-grid__right",
                                div { class: "v-team-grid",
                                    for member in site.team.iter() {
                                        TeamCard { member: member.clone() }
                                    }
                                }
                            }
                        }

                        // ── Bottom Row: By the Numbers ────────────────────────
                        div { class: "v-about-bottom-row",
                            div { class: "v-about-grid__stats",
                                span { class: "v-eyebrow", "By the Numbers" }
                                h3 { class: "v-about-grid__stats-title",
                                    "{site.analytics.title}"
                                }

                                div { class: "v-heatmap-container v-reveal",
                                    // Stats cards
                                    div { class: "v-about-stats",
                                        for stat in site.analytics.stats.iter() {
                                            div { class: "v-about-stat",
                                                div { class: "v-sparkle-border" }
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
    }
}

#[component]
fn TeamCard(member: TeamMember) -> Element {
    // Empty name means the maintainer hasn't filled in the team block
    // yet — skip the section rather than render an empty card.
    if member.name.is_empty() {
        return rsx! {};
    }
    let has_photo = !member.photo.is_empty();
    let monogram = if member.monogram.is_empty() {
        member
            .name
            .chars()
            .next()
            .map_or("•".to_string(), |c| c.to_uppercase().collect::<String>())
    } else {
        member.monogram.to_string()
    };
    rsx! {
        div { class: "v-team-card v-card-modern v-reveal", id: "team-member",
            div { class: "v-sparkle-border" }
            if has_photo {
                div { class: "v-team-card__photo",
                    img {
                        src: "{member.photo}",
                        alt: "{member.name} — Velvt Team",
                        loading: "lazy",
                        width: "200",
                        height: "200",
                    }
                }
            } else {
                div {
                    class: "v-team-card__photo v-team-card__photo--placeholder",
                    role: "img",
                    aria_label: "{member.name} — Velvt Team (portrait placeholder)",
                    "{monogram}"
                }
            }
            div { class: "v-team-card__body",
                if !member.eyebrow.is_empty() {
                    span { class: "v-team-card__eyebrow", "{member.eyebrow}" }
                }
                h3 { class: "v-team-card__name",
                    if let Some((first, last)) = member.name.split_once(' ') {
                        span { style: "color: var(--text-primary);", "{first} " }
                        span { style: "color: var(--accent);", "{last}" }
                    } else {
                        span { style: "color: var(--text-primary);", "{member.name}" }
                    }
                }
                if !member.bio.is_empty() {
                    p { class: "v-team-card__bio", "{member.bio}" }
                }
                if !member.handles.is_empty() {
                    div { class: "v-team-card__handles",
                        for handle in member.handles.iter() {
                            a {
                                class: "v-team-card__handle",
                                href: "{handle.href}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                aria_label: "{member.name} on {handle.icon}",
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
