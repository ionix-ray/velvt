//! Hero panel — headline, badge, hologram stat grid, CTA.
//! Social strip is rendered globally by the Home route, not here.

use crate::Site;
use crate::theme::wordmark::Wordmark;
use dioxus::prelude::*;

#[component]
pub fn HeroPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "home",
            div { class: "v-hero",
                div { class: "v-container",
                    div { class: "v-hero__content",
                        div { class: "v-reveal-left",
                            div { class: "v-hero__badge",
                                span { class: "v-hero__badge-dot" }
                                span { "{site.hero.badge}" }
                            }
                            h1 { class: "v-display-1 v-hero__title",
                                span { "{site.hero.headline1}" }
                                br {}
                                span { "{site.hero.headline2}" }
                                " "
                                span { class: "v-accent", "{site.hero.headline3}" }
                            }
                            p { class: "v-hero__sub", "{site.hero.sub}" }
                            div { class: "v-btn-group",
                                a {
                                    class: "v-btn v-btn--primary",
                                    href: "{site.hero.cta_primary_href}",
                                    span { "{site.hero.cta_primary}" }
                                    span { class: "v-btn__arrow", "→" }
                                }
                                a {
                                    class: "v-btn v-btn--outline",
                                    href: "{site.hero.cta_secondary_href}",
                                    span { "{site.hero.cta_secondary}" }
                                }
                            }
                        }
                        div { class: "v-hero__visual v-reveal-right",
                            div {
                                class: "v-hero-3d-wrapper",
                                div { class: "v-glass-effect v-hero-3d-logo v-hero-3d-logo--clear",
                                    // Running sparkle border — child element so its
                                    // border-only mask never hides the video inside.
                                    div { class: "v-sparkle-border" }
                                    // Inline wordmark: real DOM, so CSS owns the wordmark
                                    // colour (`--wordmark`) and the ambient field shows
                                    // straight through the transparent tile. The configured
                                    // MP4/raster remain the fallback if motion is reduced.
                                    Wordmark {
                                        class: "v-logo-svg v-hero__logo-svg".to_string(),
                                        title: Some("Velvt".to_string()),
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
