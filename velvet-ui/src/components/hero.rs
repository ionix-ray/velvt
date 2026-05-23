//! Hero — three-plane parallax. Back: velvet radial wash. Mid: faint V-mark.
//! Front: typography + CTA. All planes GPU-composited via translate3d.

use crate::Site;
use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn Hero(site: Site) -> Element {
    let mark = asset!("/assets/images/mark.jpg");
    let mid_style = format!("background-image: url('{mark}');");
    rsx! {
        section { id: "top", class: "v-hero",
            div { class: "v-hero__plane v-hero__plane--back",  aria_hidden: "true" }
            div { class: "v-hero__plane v-hero__plane--mid",   aria_hidden: "true", style: "{mid_style}" }
            div { class: "v-hero__plane v-hero__plane--front", aria_hidden: "true" }
            div { class: "v-hero__inner v-container",
                p { class: "v-eyebrow v-hero__eyebrow", "{site.hero.eyebrow}" }
                h1 { class: "v-display-1 v-hero__title", "{site.hero.headline}" }
                p  { class: "v-body v-hero__sub", "{site.hero.sub}" }
                a  { class: "v-hero__cta", href: "{site.hero.cta_href}",
                    span { "{site.hero.cta}" }
                    Icon { name: "arrow".to_string() }
                }
            }
            div { class: "v-hero__cue", aria_hidden: "true",
                Icon { name: "chevron".to_string() }
            }
        }
    }
}
