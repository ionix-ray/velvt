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
                        loading: "eager",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AnalyticStat, Analytics, Story, StoryItem, TeamMember};
    use dioxus::prelude::VirtualDom;

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapPanel() -> Element {
        let site = Site {
            story: Story {
                title: "Our Heritage".into(),
                sub: "Built on culture".into(),
                items: vec![StoryItem {
                    year: "2024".into(),
                    title: "Foundation".into(),
                    body: "Started in Bhubaneswar".into(),
                }],
            },
            analytics: Analytics {
                title: "Impact Metrics".into(),
                sub: "Real growth numbers".into(),
                stats: vec![
                    AnalyticStat {
                        value: "350+".into(),
                        label: "Events".into(),
                        change: "+40%".into(),
                    },
                    AnalyticStat {
                        value: "92%".into(),
                        label: "Retention".into(),
                        change: "Top".into(),
                    },
                ],
            },
            team: vec![TeamMember {
                name: "Arpita Parhi".into(),
                eyebrow: "Founder".into(),
                monogram: "AP".into(),
                photo: "/assets/images/team/arpita.webp".into(),
                bio: "Creative Producer".into(),
                handles: vec![],
            }],
            ..Site::load().clone()
        };
        rsx! {
            AboutAggregatedPanel { site }
        }
    }

    #[component]
    fn WrapEmptyMember() -> Element {
        rsx! {
            TeamCard { member: TeamMember::default() }
        }
    }

    #[component]
    fn WrapMemberWithPhoto() -> Element {
        rsx! {
            TeamCard {
                member: TeamMember {
                    name: "Arpita Parhi".into(),
                    eyebrow: "Founder & Creative Director".into(),
                    monogram: "AP".into(),
                    photo: "/assets/images/team/arpita.webp".into(),
                    bio: "Leading creative production in Odisha.".into(),
                    handles: vec![crate::config::SocialHandle {
                        icon: "instagram".into(),
                        label: "@arpita".into(),
                        href: "https://instagram.com/arpita".into(),
                    }],
                }
            }
        }
    }

    #[component]
    fn WrapMemberMonogramFallback() -> Element {
        rsx! {
            TeamCard {
                member: TeamMember {
                    name: "Nihar".into(),
                    eyebrow: "".into(),
                    monogram: "".into(),
                    photo: "".into(),
                    bio: "".into(),
                    handles: vec![],
                }
            }
        }
    }

    #[component]
    fn WrapMemberCustomMonogram() -> Element {
        rsx! {
            TeamCard {
                member: TeamMember {
                    name: "Creative Team".into(),
                    eyebrow: "".into(),
                    monogram: "CT".into(),
                    photo: "".into(),
                    bio: "".into(),
                    handles: vec![],
                }
            }
        }
    }

    #[test]
    fn panel_renders_story_and_clean_stats_without_heatmap_grid() {
        let html = render(WrapPanel);
        assert!(html.contains("Our Heritage"));
        assert!(html.contains("Built on culture"));
        assert!(html.contains("Foundation"));
        assert!(html.contains("Started in Bhubaneswar"));
        assert!(html.contains("Impact Metrics"));
        assert!(html.contains("350+"));
        assert!(html.contains("Events"));
        assert!(html.contains("v-about-stats"));
        assert!(
            !html.contains("v-heatmap-grid"),
            "heatmap grid must remain deleted"
        );
    }

    #[test]
    fn team_card_empty_name_renders_nothing() {
        let html = render(WrapEmptyMember);
        assert_eq!(html.trim(), "");
    }

    #[test]
    fn team_card_renders_photo_and_two_word_name() {
        let html = render(WrapMemberWithPhoto);
        assert!(html.contains("Arpita "));
        assert!(html.contains("Parhi"));
        assert!(html.contains("Founder"));
        assert!(html.contains("Creative Director"));
        assert!(html.contains("Leading creative production"));
        assert!(html.contains("img"));
        assert!(html.contains("/assets/images/team/arpita.webp"));
        assert!(html.contains("@arpita"));
        assert!(html.contains("v-team-card__handles"));
    }

    #[test]
    fn team_card_placeholder_monogram_fallback() {
        // Without photo and without monogram -> derives monogram from first letter
        let html = render(WrapMemberMonogramFallback);
        assert!(html.contains("Nihar"));
        assert!(html.contains("v-team-card__photo--placeholder"));
        assert!(
            html.contains(">N<"),
            "placeholder should have derived monogram 'N'"
        );
        assert!(!html.contains("<img"));
    }

    #[test]
    fn team_card_placeholder_custom_monogram() {
        let html = render(WrapMemberCustomMonogram);
        assert!(html.contains("v-team-card__photo--placeholder"));
        assert!(html.contains("CT"));
    }
}
