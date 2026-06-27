//! Cases panel — client case study cards on the home page.
//! Cards use the unified `.v-tile` system (Carbon-inspired) shared with
//! the showcase grid and the case-studies index page.

use crate::Site;
use dioxus::prelude::*;

/// First letter of the client name, uppercased, for the monogram badge.
/// `logo_image` is a content-driven path that can't go through the
/// asset!() pipeline, so it can't be guaranteed to resolve — a monogram
/// never 404s.
fn monogram(client: &str) -> String {
    client
        .chars()
        .find(|c| c.is_alphanumeric())
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "•".to_string())
}

use crate::case_studies::get_all_case_studies;
use crate::config::CaseItem;

#[component]
pub fn CasesPanel(site: Site) -> Element {
    let mut all_cases = site.cases.items.clone();
    for (slug, fm, _) in get_all_case_studies() {
        if !all_cases.iter().any(|c| c.slug.as_ref() == slug) {
            all_cases.push(CaseItem {
                client: fm.client.clone(),
                metric: fm.metric.clone(),
                desc: fm.summary.clone(),
                tags: fm.tags.clone(),
                logo_image: "".into(),
                button_link: "".into(),
                footer_label: "Velvt Studio".into(),
                slug: slug.into(),
            });
        }
    }

    rsx! {
        section { class: "v-panel", id: "cases",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-panel-header v-reveal",
                        span { class: "v-eyebrow", "Showcase" }
                        h2 { class: "v-display-2", "{site.cases.title}" }
                        p { class: "v-panel-header__sub", "{site.cases.sub}" }
                    }
                    div { class: "v-cases-grid",
                        for (i, case) in all_cases.iter().enumerate() {
                            div {
                                class: "v-card-modern v-reveal",
                                style: "transition-delay: {(i + 1) * 80}ms;",
                                div { class: "v-card-modern__image" }
                                div { class: "v-card-modern__content",
                                    div {
                                        class: "v-card-modern__logo",
                                        "aria-hidden": "true",
                                        "{monogram(&case.client)}"
                                    }
                                    div { class: "v-card-modern__client", "{case.client}" }
                                    div { class: "v-card-modern__metric", "{case.metric}" }
                                    div { class: "v-tags",
                                        for tag in case.tags.iter() {
                                            span { class: "v-tag--green", "{tag}" }
                                        }
                                    }
                                    p { class: "v-card-modern__desc", "{case.desc}" }
                                    if case.slug.is_empty() {
                                        a {
                                            class: "v-btn-glow",
                                            href: "{case.button_link}",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            "View Case Study"
                                        }
                                    } else {
                                        a {
                                            class: "v-btn-glow",
                                            href: "/cases/{case.slug}",
                                            "data-spa": "true",
                                            "View Case Study"
                                        }
                                    }
                                }
                                div { class: "v-card-modern__footer", "{case.footer_label}" }
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
    use crate::config::{CaseItem, Cases};
    use dioxus::prelude::VirtualDom;

    #[test]
    fn monogram_takes_first_alphanumeric_char_uppercased() {
        assert_eq!(monogram("TechNova"), "T");
        assert_eq!(monogram("greenfuture"), "G");
        assert_eq!(monogram("  Luxe Beauty"), "L");
    }

    #[test]
    fn monogram_falls_back_when_no_alphanumeric_chars() {
        assert_eq!(monogram(""), "•");
        assert_eq!(monogram("   "), "•");
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    fn site_with_case(item: CaseItem) -> Site {
        Site {
            cases: Cases {
                items: vec![item],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[component]
    fn WrapWithSlug() -> Element {
        let site = site_with_case(CaseItem {
            client: "TechNova".to_string(),
            slug: "technova-full-funnel-growth".to_string(),
            button_link: "#".to_string(),
            ..Default::default()
        });
        rsx! {
            CasesPanel { site }
        }
    }

    #[component]
    fn WrapWithoutSlug() -> Element {
        let site = site_with_case(CaseItem {
            client: "Legacy Client".to_string(),
            button_link: "https://example.com/legacy".to_string(),
            ..Default::default()
        });
        rsx! {
            CasesPanel { site }
        }
    }

    #[test]
    fn case_with_slug_links_to_internal_case_study_page() {
        let html = render(WrapWithSlug);
        assert!(html.contains("href=\"/cases/technova-full-funnel-growth\""));
        assert!(!html.contains("target=\"_blank\""));
    }

    #[test]
    fn case_without_slug_keeps_legacy_external_link_behaviour() {
        let html = render(WrapWithoutSlug);
        assert!(html.contains("href=\"https://example.com/legacy\""));
        assert!(html.contains("target=\"_blank\""));
    }
}
