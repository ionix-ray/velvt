//! SocialStrip — fixed vertical social icon strip, visible on all panels
//! except the last (footer) panel where the footer itself has socials.
//!
//! Links come from the same single source as the footer: `content/site.md`
//! `[[footer.socials]]`, resolved through `Site::load()`. Same icon-label
//! mechanism as `footer_panel` (`pretty_label` + `Icon { name }`).

use crate::Site;
use crate::components::icons::{Icon, pretty_label};
use dioxus::prelude::*;

#[component]
pub fn SocialStrip(site: Site, is_last_panel: bool) -> Element {
    rsx! {
        div {
            class: if is_last_panel { "v-social-strip v-social-strip--hidden" } else { "v-social-strip" },
            "aria-hidden": if is_last_panel { "true" } else { "false" },
            for social in site.footer.socials.iter() {
                a {
                    key: "{social.href}",
                    href: "{social.href}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "v-social-strip__link",
                    "aria-label": "Velvt on {pretty_label(&social.label)}",
                    Icon { name: social.label.to_string() }
                }
            }
        }
    }
}
