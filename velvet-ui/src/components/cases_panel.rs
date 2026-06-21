//! Cases panel — client case study cards (compact, fit within 100vh).
//! Cards use v-card-modern with smaller height image sections.

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

#[component]
pub fn CasesPanel(site: Site) -> Element {
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
                        for (i, case) in site.cases.items.iter().enumerate() {
                            div {
                                class: "v-card-modern v-reveal",
                                style: "transition-delay: {(i + 1) * 80}ms;",
                                div {
                                    class: "v-card-modern__image",
                                    style: "background-image: url('{case.bg_image}');"
                                }
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
                                    a {
                                        class: "v-btn-glow",
                                        href: "{case.button_link}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "View Case Study"
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
}
