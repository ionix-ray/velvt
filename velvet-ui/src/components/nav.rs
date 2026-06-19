//! Fixed cinematic navbar. Brand mark + sparse links + soft backdrop blur.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn Nav(site: Site) -> Element {
    rsx! {
        nav { class: "v-nav", aria_label: "Primary",
            a { class: "v-nav__brand", href: "#top", aria_label: "Vaelvet — home",
                img {
                    src: asset!("/assets/images/logo-nav.jpg"),
                    alt: "{site.brand.name}",
                    width: "120",
                    height: "32",
                    decoding: "sync",
                    loading: "eager",
                }
            }
            div { class: "v-nav__links",
                for item in site.nav.iter() {
                    a { class: "v-nav__link", href: "{item.href}", "{item.label}" }
                }
            }
        }
    }
}
