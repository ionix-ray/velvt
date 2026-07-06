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
                            div {
                                class: "v-hero-3d-wrapper",
                                div { class: "v-glass-effect v-hero-3d-logo",
                                    img {
                                        class: "v-hero-3d-logo__img",
                                        src: "{site.hero.logo}",
                                        alt: "Velvt Logo"
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

