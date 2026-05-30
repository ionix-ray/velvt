//! Hero panel — headline, badge, stats grid, CTA, social strip.

use crate::Site;
use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn HeroPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "home",
            div { class: "v-hero",
                div { class: "v-hero__social",
                    a { href: "https://facebook.com/vaelvet", target: "_blank", rel: "noopener",
                        Icon { name: "facebook".to_string() }
                    }
                    a { href: "https://instagram.com/vaelvet", target: "_blank", rel: "noopener",
                        Icon { name: "instagram".to_string() }
                    }
                    a { href: "https://youtube.com/@vaelvet", target: "_blank", rel: "noopener",
                        Icon { name: "youtube".to_string() }
                    }
                    a { href: "https://linkedin.com/company/vaelvet", target: "_blank", rel: "noopener",
                        Icon { name: "linkedin".to_string() }
                    }
                }
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
                                br {}
                                span { class: "v-accent", "{site.hero.headline3}" }
                            }
                            p { class: "v-body v-hero__sub", "{site.hero.sub}" }
                            div { class: "v-btn-group",
                                a { class: "v-btn v-btn--primary", href: "{site.hero.cta_primary_href}",
                                    span { "{site.hero.cta_primary}" }
                                    span { class: "v-btn__arrow", "\u{2192}" }
                                }
                                a { class: "v-btn v-btn--outline", href: "{site.hero.cta_secondary_href}",
                                    span { "{site.hero.cta_secondary}" }
                                }
                            }
                        }
                        div { class: "v-hero__visual v-reveal-right",
                            div { class: "v-hero__stat-grid",
                                for stat in site.hero.stats.iter() {
                                    div { class: "v-stat-card",
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
