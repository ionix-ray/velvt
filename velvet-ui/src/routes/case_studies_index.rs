//! Case studies index — `/achivements` (all) and `/achivements/tag/:tag` (filtered).
//! Cards link into the detail page; tag chips on the detail page link back
//! into the filtered view here, so visitors can browse by category. Layout
//! follows the reference app's two-column (sidebar filter + main grid)
//! structure, restyled with Vaelvet's own tokens.

use crate::Site;
use crate::case_studies::get_all_case_studies;
use crate::components::case_header::CaseHeader;
use crate::components::footer_panel::FooterPanel;
use crate::components::social_strip::SocialStrip;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// All distinct tags across every case study, in first-seen order.
fn all_distinct_tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for (_, frontmatter, _) in get_all_case_studies() {
        for tag in frontmatter.tags {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
    }
    tags
}

fn case_studies_grid(filter_tag: Option<&str>) -> Element {
    let studies: Vec<_> = get_all_case_studies()
        .into_iter()
        .filter(|(_, frontmatter, _)| {
            filter_tag.is_none_or(|tag| frontmatter.tags.iter().any(|t| t == tag))
        })
        .collect();
    let tags = all_distinct_tags();
    let count = studies.len();
    let site = Site::load().clone();

    use_effect(move || {
        // Scroll to top without eval() — CSP-safe.
        if let Some(win) = web_sys::window() {
            let opts = web_sys::ScrollToOptions::new();
            opts.set_top(0.0);
            opts.set_left(0.0);
            opts.set_behavior(web_sys::ScrollBehavior::Instant);
            win.scroll_to_with_scroll_to_options(&opts);

            // Deferred second call to override any post-render scroll restoration.
            if let Some(win2) = web_sys::window() {
                let cb = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                    let opts2 = web_sys::ScrollToOptions::new();
                    opts2.set_top(0.0);
                    opts2.set_left(0.0);
                    opts2.set_behavior(web_sys::ScrollBehavior::Instant);
                    win2.scroll_to_with_scroll_to_options(&opts2);
                });
                let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    50,
                );
                cb.forget();
            }
        }
    });

    rsx! {
        div { class: "v-case-page",
            CaseHeader {}
            SocialStrip { is_last_panel: false }
            section { class: "v-case-hero",
                div { class: "v-container",
                    span { class: "v-eyebrow", "Showcase" }
                    h1 { class: "v-display-2",
                        if let Some(tag) = filter_tag {
                            "Tagged: {tag}"
                        } else {
                            "All case studies"
                        }
                    }
                }
            }
            div { class: "v-container",
                div { class: "v-case-layout",
                    aside { class: "v-case-layout__sidebar",
                        div { class: "v-case-sidebar-card",
                            p { class: "v-case-sidebar-card__label", "Filter by topic" }
                            nav { class: "v-case-tag-filter",
                                a {
                                    class: if filter_tag.is_none() { "v-case-tag-filter__item active" } else { "v-case-tag-filter__item" },
                                    href: "/achivements",
                                    "data-spa": "true",
                                    "All"
                                }
                                for tag in tags.iter() {
                                    a {
                                        class: if filter_tag == Some(tag.as_str()) { "v-case-tag-filter__item active" } else { "v-case-tag-filter__item" },
                                        href: "/achivements/tag/{tag}",
                                        "data-spa": "true",
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    }
                    main { class: "v-case-layout__main",
                        p { class: "v-case-results-count", "Showing {count} case studies" }
                        if studies.is_empty() {
                            p { class: "v-panel-header__sub", "No case studies match this tag yet." }
                        } else {
                            div { class: "v-cases-grid",
                                for (slug , frontmatter , _body) in studies.iter() {
                                    a {
                                        class: "v-tile v-tile--clickable v-reveal",
                                        href: "/achivements/{slug}",
                                        "data-spa": "true",
                                        span { class: "v-tile__eyebrow", "{frontmatter.client}" }
                                        div { class: "v-tile__metric", "{frontmatter.metric}" }
                                        p { class: "v-tile__desc", "{frontmatter.summary}" }
                                        div { class: "v-tags v-tile__tags",
                                            for tag in frontmatter.tags.iter() {
                                                span { class: "v-tag--green", "{tag}" }
                                            }
                                        }
                                        span { class: "v-tile__chevron", "→" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            FooterPanel { site }
        }
    }
}

#[component]
pub fn CaseStudiesIndex() -> Element {
    case_studies_grid(None)
}

#[component]
pub fn CaseStudiesByTag(tag: String) -> Element {
    case_studies_grid(Some(&tag))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::VirtualDom;

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapIndex() -> Element {
        rsx! {
            CaseStudiesIndex {}
        }
    }

    #[component]
    fn WrapTagFilter() -> Element {
        rsx! {
            CaseStudiesByTag { tag: "Beauty".to_string() }
        }
    }

    #[component]
    fn WrapUnknownTagFilter() -> Element {
        rsx! {
            CaseStudiesByTag { tag: "NoSuchTag".to_string() }
        }
    }

    #[test]
    fn index_lists_every_sample_case_study() {
        let html = render(WrapIndex);
        assert!(html.contains("TechNova"));
        assert!(html.contains("Luxe Beauty"));
        assert!(html.contains("GreenFuture"));
        assert!(html.contains("href=\"/achivements/technova-full-funnel-growth\""));
    }

    #[test]
    fn tag_filter_shows_only_matching_case_studies() {
        let html = render(WrapTagFilter);
        assert!(html.contains("Luxe Beauty"));
        assert!(!html.contains("TechNova"));
        assert!(!html.contains("GreenFuture"));
    }

    #[test]
    fn unknown_tag_filter_renders_empty_state_without_panicking() {
        let html = render(WrapUnknownTagFilter);
        assert!(html.contains("No case studies match this tag yet."));
    }

    #[test]
    fn renders_the_brand_image_via_shared_case_header() {
        let html = render(WrapIndex);
        assert!(html.contains("<img"));
        assert!(html.contains("velvet-square"));
    }

    #[test]
    fn renders_a_theme_toggle_button() {
        let html = render(WrapIndex);
        assert!(html.contains("v-theme-toggle"));
    }

    #[test]
    fn sidebar_lists_every_distinct_tag_with_an_all_link() {
        let html = render(WrapIndex);
        assert!(html.contains("v-case-layout__sidebar"));
        assert!(html.contains("href=\"/achivements\""));
        assert!(html.contains("href=\"/achivements/tag/B2B\""));
        assert!(html.contains("href=\"/achivements/tag/Beauty\""));
    }

    #[test]
    fn results_count_line_reflects_filtered_total() {
        let html = render(WrapTagFilter);
        assert!(html.contains("Showing 1 case studies"));
    }

    #[test]
    fn all_distinct_tags_deduplicates_across_all_sample_studies() {
        let tags = all_distinct_tags();
        let unique_count = tags.len();
        let mut sorted = tags.clone();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            unique_count,
            "tags must already be unique: {tags:?}"
        );
        assert!(tags.contains(&"B2B".to_string()));
    }
}
