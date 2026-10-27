//! DynamicBackground — the whole-site ambient field.
//!
//! Two composited layers, mounted once in `App` so every route shares it:
//!
//! 1. **Mesh** — two drifting crimson spotlight gradients plus a slow stage
//!    beam. Pure `background-image`; nothing to decode.
//! 2. **Glyphs** — the service icons that state what the agency actually
//!    does (camera, lens, venue, talent, grand party, creator, clapper,
//!    spotlight, aperture), each floating in its own lane with an
//!    independent drift cycle.
//!
//! Cost discipline: every motion is `transform`/`opacity` on an oversized or
//! `will-change`-promoted layer, so the browser keeps it on the compositor
//! thread — no per-frame layout, no repaint, no video decode, no canvas.
//! It resolves at any DPI, so it stays sharp on 4K without shipping a 4K asset.
//!
//! Legibility: the field is atmosphere, never content. A radial readability
//! scrim sits between the glyphs and the page, and every knob is clamped in
//! `config` so a bad value in `site.md` cannot flood the text.
//!
//! Driven entirely by `[background]` in `content/site.md`.

use crate::Site;
use crate::components::icons::Icon;
use crate::config::{
    BackgroundGlyph, background_alpha, glyph_alpha, glyph_delay, glyph_duration, glyph_percent,
    glyph_size, glyph_spin,
};
use dioxus::prelude::*;

/// Normalised inline style for one glyph, derived from config and clamped.
///
/// Returns `left`/`top` percentages, `width` in px, the drift cycle, the
/// phase offset, and the spin amplitude — the exact set of custom properties
/// the stylesheet animates.
fn glyph_style(g: &BackgroundGlyph, base_duration: u32) -> String {
    let x = glyph_percent(g.x);
    let y = glyph_percent(g.y);
    let size = glyph_size(g.size);
    let duration = glyph_duration(base_duration, g.rate);
    let delay = glyph_delay(g.delay);
    let spin = glyph_spin(g.spin);
    format!(
        "--gx:{x}%;--gy:{y}%;--gw:{size}px;--gdur:{duration}s;--gdelay:{delay}s;--gspin:{spin}deg"
    )
}

/// Tone slot → CSS class. Unknown tones fall back to the brand crimson so a
/// typo degrades to on-brand rather than invisible.
fn tone_class(tone: &str) -> &str {
    match tone {
        "violet" => "v-bg-glyph--violet",
        "sky" => "v-bg-glyph--sky",
        "citron" => "v-bg-glyph--citron",
        _ => "v-bg-glyph--crimson",
    }
}

#[component]
pub fn DynamicBackground(site: Site) -> Element {
    let bg = &site.background;
    if !bg.enabled {
        return rsx! {};
    }

    let alpha = background_alpha(bg.intensity);
    let glyph_opacity = if bg.show_glyphs {
        glyph_alpha(bg.intensity, bg.glyph_opacity)
    } else {
        0.0
    };

    rsx! {
        div {
            class: "v-dynamic-bg",
            "aria-hidden": "true",
            style: "--v-bg-alpha:{alpha};--v-bg-glyph-alpha:{glyph_opacity}",
            // ── Layer 1: gradient mesh ────────────────────────────────
            if bg.mesh {
                div { class: "v-dynamic-bg__layer v-dynamic-bg__layer--slow" }
                div { class: "v-dynamic-bg__layer v-dynamic-bg__layer--fast" }
                div { class: "v-dynamic-bg__beam" }
            }
            // ── Layer 2: floating service glyphs ─────────────────────
            if bg.show_glyphs {
                div { class: "v-dynamic-bg__glyphs",
                    for g in bg.glyphs.iter() {
                        div {
                            class: "v-bg-glyph {tone_class(&g.tone)}",
                            style: "{glyph_style(g, bg.glyph_duration)}",
                            div { class: "v-bg-glyph__icon",
                                Icon { name: g.icon.to_string() }
                            }
                        }
                    }
                }
            }
            // ── Legibility: brand scrim between field and content ─────
            div { class: "v-dynamic-bg__scrim" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glyph(icon: &str, tone: &str, x: u8, y: u8) -> BackgroundGlyph {
        BackgroundGlyph {
            icon: icon.into(),
            tone: tone.into(),
            x,
            y,
            ..Default::default()
        }
    }

    #[test]
    fn glyph_style_emits_every_custom_property_css_animates() {
        let style = glyph_style(&glyph("camera", "sky", 12, 40), 26);
        for prop in [
            "--gx:",
            "--gy:",
            "--gw:",
            "--gdur:",
            "--gdelay:",
            "--gspin:",
        ] {
            assert!(style.contains(prop), "missing {prop} in {style}");
        }
    }

    #[test]
    fn glyph_style_clamps_bad_config_values() {
        // x/y over 100, zero size, non-finite rate, absurd spin.
        let mut g = glyph("camera", "sky", 250, 250);
        g.size = 0;
        g.rate = f32::NAN;
        g.spin = 400.0;
        g.delay = 900.0;
        let style = glyph_style(&g, 0);
        assert!(style.contains("--gx:100%"), "x not clamped: {style}");
        assert!(style.contains("--gy:100%"), "y not clamped: {style}");
        assert!(style.contains("--gw:40px"), "size not floored: {style}");
        assert!(style.contains("--gdur:4s"), "duration not floored: {style}");
        assert!(style.contains("--gspin:45deg"), "spin not clamped: {style}");
        assert!(style.contains("--gdelay:0s"), "delay not clamped: {style}");
    }

    #[test]
    fn tone_slots_map_to_css_classes_with_crimson_fallback() {
        assert_eq!(tone_class("violet"), "v-bg-glyph--violet");
        assert_eq!(tone_class("sky"), "v-bg-glyph--sky");
        assert_eq!(tone_class("citron"), "v-bg-glyph--citron");
        assert_eq!(tone_class("crimson"), "v-bg-glyph--crimson");
        assert_eq!(tone_class("chartreuse"), "v-bg-glyph--crimson");
    }

    #[component]
    fn WrapField() -> Element {
        let mut site = Site::default();
        site.background.enabled = true;
        site.background.show_glyphs = true;
        site.background.glyphs = vec![glyph("camera", "sky", 20, 30)];
        rsx! {
            DynamicBackground { site }
        }
    }

    #[test]
    fn glyphs_are_aria_hidden_and_never_interactive() {
        let mut dom = VirtualDom::new(WrapField);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("aria-hidden=\"true\""),
            "field must be decorative"
        );
        assert!(html.contains("v-bg-glyph--sky"), "tone class missing");
        assert!(
            html.contains("v-dynamic-bg__scrim"),
            "legibility scrim missing"
        );
    }
}
