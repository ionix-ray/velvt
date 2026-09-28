//! DynamicBackground — whole-site procedural ambient background.
//!
//! Fixed layered brand gradients (twin drifting spotlight meshes + one slow
//! stage beam) behind every route. No video element, no decode cost, no GPU
//! requirement: all motion is transform/opacity-only on oversized layers.
//! Driven by `[background]` in `content/site.md` (`enabled`, `intensity`).

use crate::Site;
use crate::config::background_alpha;
use dioxus::prelude::*;

#[component]
pub fn DynamicBackground(site: Site) -> Element {
    if !site.background.enabled {
        return rsx! {};
    }
    let alpha = background_alpha(site.background.intensity).to_string();
    rsx! {
        div {
            class: "v-dynamic-bg",
            "aria-hidden": "true",
            style: "--v-bg-alpha:{alpha}",
            div { class: "v-dynamic-bg__layer v-dynamic-bg__layer--slow" }
            div { class: "v-dynamic-bg__layer v-dynamic-bg__layer--fast" }
            div { class: "v-dynamic-bg__beam" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_scales_and_clamps_intensity() {
        assert_eq!(background_alpha(80), 0.8);
        assert_eq!(background_alpha(0), 0.0);
        assert_eq!(background_alpha(100), 1.0);
        assert_eq!(background_alpha(250), 1.0);
    }
}
