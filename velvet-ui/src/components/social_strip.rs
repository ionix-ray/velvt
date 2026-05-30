//! SocialStrip — fixed vertical social icon strip, visible on all panels
//! except the last (footer) panel where the footer itself has socials.

use crate::components::icons::Icon;
use dioxus::prelude::*;

#[component]
pub fn SocialStrip(is_last_panel: bool) -> Element {
    rsx! {
        div {
            class: if is_last_panel { "v-social-strip v-social-strip--hidden" } else { "v-social-strip" },
            "aria-hidden": if is_last_panel { "true" } else { "false" },
            a {
                href: "https://facebook.com/velvt",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "v-social-strip__link",
                "aria-label": "Velvt on Facebook",
                Icon { name: "facebook".to_string() }
            }
            a {
                href: "https://instagram.com/velvt",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "v-social-strip__link",
                "aria-label": "Velvt on Instagram",
                Icon { name: "instagram".to_string() }
            }
            a {
                href: "https://youtube.com/@velvt",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "v-social-strip__link",
                "aria-label": "Velvt on YouTube",
                Icon { name: "youtube".to_string() }
            }
            a {
                href: "https://linkedin.com/company/velvt",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "v-social-strip__link",
                "aria-label": "Velvt on LinkedIn",
                Icon { name: "linkedin".to_string() }
            }
        }
    }
}
