//! Hero panel — headline, badge, hologram stat grid, CTA.
//! Social strip is rendered globally by the Home route, not here.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn HeroPanel(site: Site) -> Element {
    let mut mouse_x = use_signal(|| 0.5f64);
    let mut mouse_y = use_signal(|| 0.5f64);

    let handle_mousemove = move |evt: Event<dioxus::html::MouseData>| {
        if let Some(win) = web_sys::window() {
            let width = win.inner_width().unwrap().as_f64().unwrap_or(1920.0);
            let height = win.inner_height().unwrap().as_f64().unwrap_or(1080.0);
            let mx = (evt.client_coordinates().x as f64) / width;
            let my = (evt.client_coordinates().y as f64) / height;
            mouse_x.set(mx);
            mouse_y.set(my);
        }
    };

    rsx! {
        section { class: "v-panel", id: "home",
            div { class: "v-hero", onmousemove: handle_mousemove,
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
                                style: "--mouse-x: {mouse_x()}; --mouse-y: {mouse_y()};",
                                div { class: "v-glass-effect v-hero-3d-logo",
                                    div {
                                        class: "v-hero-3d-logo__img",
                                        "aria-label": "Velvt Logo",
                                        role: "img"
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
