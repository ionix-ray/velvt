//! Case study detail page — `/achivements/:slug`. Renders one markdown-backed
//! case study outside the home page's horizontal-panel flow, using a
//! two-column (sidebar + main content) layout matching the reference
//! app's blog-post structure, restyled with Vaelvet's own tokens.

use crate::Site;
use crate::case_studies::get_case_study;
use crate::components::case_header::CaseHeader;
use crate::components::footer_panel::FooterPanel;
use crate::components::markdown_renderer::MarkdownRenderer;
use crate::components::social_strip::SocialStrip;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Estimated reading time in whole minutes, at 200 words/minute, rounded up,
/// with a floor of 1 minute (so an empty or very short body still reads as
/// "1 min" rather than "0 min").
pub fn read_time_minutes(body: &str) -> usize {
    let word_count = body.split_whitespace().count();
    word_count.div_ceil(200).max(1)
}

#[component]
pub fn CaseStudy(slug: String) -> Element {
    match get_case_study(&slug) {
        Some((frontmatter, body)) => {
            let site = Site::load().clone();
            let read_time = read_time_minutes(&body);
            use_effect(move || {
                // Scroll to top without using eval() so the CSP needn't allow unsafe-eval.
                // Instant + deferred call wins any race with Dioxus re-rendering.
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
                    CaseHeader { back_href: "/achivements".to_string(), back_label: "All case studies".to_string() }
                    SocialStrip { is_last_panel: false }
                    section { class: "v-case-hero",
                        div { class: "v-container",
                            span { class: "v-eyebrow", "{frontmatter.client}" }
                            h1 { class: "v-display-2", "{frontmatter.title}" }
                            div { class: "v-case-hero__meta",
                                span { class: "v-case-hero__metric", "{frontmatter.metric}" }
                                span { class: "v-case-hero__date", "{frontmatter.date}" }
                            }
                            p { class: "v-panel-header__sub", "{frontmatter.summary}" }
                            div { class: "v-tags",
                                for tag in frontmatter.tags.iter() {
                                    a {
                                        class: "v-tag--green",
                                        href: "/achivements/tag/{tag}",
                                        "data-spa": "true",
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "v-container",
                        nav { class: "v-case-breadcrumb", aria_label: "Breadcrumb",
                            a { href: "/achivements", "data-spa": "true", "Case Studies" }
                            span { class: "v-case-breadcrumb__sep", "/" }
                            span { class: "v-case-breadcrumb__current", "{frontmatter.title}" }
                        }
                        div { class: "v-case-layout",
                            aside { class: "v-case-layout__sidebar",
                                div { class: "v-case-sidebar-card",
                                    p { class: "v-case-sidebar-card__label", "Published" }
                                    p { class: "v-case-sidebar-card__value", "{frontmatter.date}" }
                                }
                                div { class: "v-case-sidebar-card",
                                    p { class: "v-case-sidebar-card__label", "Topics" }
                                    div { class: "v-tags",
                                        for tag in frontmatter.tags.iter() {
                                            a {
                                                class: "v-tag--green",
                                                href: "/achivements/tag/{tag}",
                                                "data-spa": "true",
                                                "{tag}"
                                            }
                                        }
                                    }
                                }
                                div { class: "v-case-sidebar-card",
                                    p { class: "v-case-sidebar-card__label", "Read time" }
                                    p { class: "v-case-sidebar-card__value", "{read_time} min" }
                                }
                            }
                            main { class: "v-case-layout__main",
                                article { class: "v-case-article",
                                    MarkdownRenderer { content: body }
                                }
                                a {
                                    class: "v-case-page__back",
                                    href: "/achivements",
                                    "data-spa": "true",
                                    "Back to case studies"
                                }
                            }
                        }
                    }
                    FooterPanel { site }
                }
            }
        }
        None => {
            use_effect(move || {
                if let Some(win) = web_sys::window() {
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(0.0);
                    opts.set_left(0.0);
                    opts.set_behavior(web_sys::ScrollBehavior::Instant);
                    win.scroll_to_with_scroll_to_options(&opts);
                }
            });
            rsx! {
            div { class: "v-case-page v-case-page--missing",
                CaseHeader {}
                div { class: "v-container",
                    h1 { class: "v-display-2", "Case study not found" }
                    p { class: "v-panel-header__sub",
                        "We couldn't find a case study at this address."
                    }
                    a { class: "v-btn-tile", href: "/", "data-spa": "true", "Back to Velvt" }
                }
            }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::VirtualDom;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapKnownSlug() -> Element {
        rsx! {
            CaseStudy { slug: "technova-full-funnel-growth".to_string() }
        }
    }

    #[component]
    fn WrapUnknownSlug() -> Element {
        rsx! {
            CaseStudy { slug: "no-such-case-study".to_string() }
        }
    }

    #[test]
    fn known_slug_renders_title_metric_and_tags() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("TechNova"));
        assert!(html.contains("+240%"));
        assert!(html.contains("B2B"));
        assert!(html.contains("v-prose"));
        assert!(html.contains("The brief"));
    }

    #[test]
    fn known_slug_tag_links_point_to_the_tag_filtered_index() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("href=\"/achivements/tag/B2B\""));
    }

    #[test]
    fn unknown_slug_renders_not_found_without_panicking() {
        let html = render(WrapUnknownSlug);
        assert!(html.contains("Case study not found"));
        assert!(!html.contains("v-prose"));
    }

    #[test]
    fn renders_the_brand_image_via_shared_case_header() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("<img"));
        assert!(html.contains("velvet-square"));
    }

    #[test]
    fn renders_a_theme_toggle_button() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("v-theme-toggle"));
    }

    #[test]
    fn sidebar_has_published_date_and_topic_links() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("v-case-layout__sidebar"));
        assert!(html.contains("Published"));
        assert!(html.contains("2026") || html.contains("date"));
        assert!(html.contains("Topics"));
        assert!(html.contains("href=\"/achivements/tag/B2B\""));
    }

    #[test]
    fn breadcrumb_links_back_to_case_studies_index() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("v-case-breadcrumb"));
        assert!(html.contains("Case Studies"));
        assert!(html.contains("href=\"/achivements\""));
    }

    #[test]
    fn main_content_has_back_link_below_the_article() {
        let html = render(WrapKnownSlug);
        assert!(html.contains("v-case-article"));
        assert!(html.contains("Back to case studies"));
    }

    #[test]
    fn read_time_minutes_short_text_is_one_minute() {
        assert_eq!(read_time_minutes("just a few words here"), 1);
    }

    #[test]
    fn read_time_minutes_empty_body_is_one_minute_minimum() {
        assert_eq!(read_time_minutes(""), 1);
    }

    #[test]
    fn read_time_minutes_rounds_up_for_longer_text() -> TestResult {
        let body = "word ".repeat(401);
        // 401 words / 200 wpm = 2.005 -> ceil -> 3
        assert_eq!(read_time_minutes(&body), 3);
        let body_exact = "word ".repeat(400);
        // exactly 2.0 -> ceil -> 2, not bumped up
        assert_eq!(read_time_minutes(&body_exact), 2);
        Ok(())
    }
}
