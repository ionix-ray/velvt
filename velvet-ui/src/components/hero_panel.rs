//! Hero panel — headline, badge, hologram stat grid, CTA.
//! Social strip is rendered globally by the Home route, not here.

use crate::Site;
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
                            div { class: "v-hologram",
                                div { class: "v-hologram__glow", "aria-hidden": "true" }
                                div { class: "v-hologram__scanlines", "aria-hidden": "true" }
                                div { class: "v-hologram__particles", "aria-hidden": "true",
                                    for i in 0..8 {
                                        div { class: "v-hologram__particle", key: "{i}" }
                                    }
                                }
                                div { class: "v-hologram__ring v-hologram__ring--1" }
                                div { class: "v-hologram__ring v-hologram__ring--2" }
                                div { class: "v-hero__stat-grid",
                                    for (i, stat) in site.hero.stats.iter().enumerate() {
                                        div {
                                            class: "v-stat-card",
                                            style: "--float-delay: {format_delay(i)}",
                                            div { class: "v-stat-card__glow", "aria-hidden": "true" }
                                            div { class: "v-stat-card__value", "{stat.value}" }
                                            div { class: "v-stat-card__label", "{stat.label}" }
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

fn format_delay(i: usize) -> String {
    format!("{}s", i as f64 * 0.3)
}
