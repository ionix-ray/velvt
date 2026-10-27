//! Brand wordmark — inline SVG, single source of truth.
//!
//! One lockup for every surface (topbar, stacked nav, hero, footer, loader,
//! case-study header):
//!   * emblem  → `velvet-mark.webp` (alpha-cropped crimson botanical "V")
//!   * VELVT   → vector `<text>` in `currentColor`
//!
//! Why inline instead of `<img src="…svg">`: `currentColor` cannot cross an
//! `<img>` document boundary, so a linked SVG can never follow the brand
//! palette or the active theme. Rendering the lockup as real DOM means CSS
//! owns the wordmark colour (`--wordmark`), and the emblem stays one shared,
//! cacheable raster. Result: the wordmark is always the same crimson family
//! as the emblem in both themes, with no second asset to keep in sync.

use dioxus::prelude::*;

use super::brand::brand_mark;

/// Aspect ratio of the lockup, kept in sync with the SVG `viewBox`.
/// 874 × 344 → 2.5407. Exposed so layout can reserve space without a magic
/// number in CSS.
pub const WORDMARK_VIEWBOX_W: u32 = 874;
/// Lockup height in `viewBox` units — see [`WORDMARK_VIEWBOX_W`].
pub const WORDMARK_VIEWBOX_H: u32 = 344;

/// Inline Velvt wordmark.
///
/// # Props
/// - `class`   — extra classes for the host `<svg>` (animation hooks, layout).
/// - `title`   — accessible name; the element is decorative when `None`.
#[component]
pub fn Wordmark(class: String, title: Option<String>) -> Element {
    let label = title.clone().unwrap_or_else(|| "Velvt".to_string());
    rsx! {
        svg {
            class: "{class}",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 {WORDMARK_VIEWBOX_W} {WORDMARK_VIEWBOX_H}",
            fill: "none",
            role: "img",
            "aria-label": "{label}",
            // Emblem: the crimson botanical V, letterboxed to a square that
            // optically matches the cap-height of the wordmark beside it.
            image {
                x: "10",
                y: "30",
                width: "284",
                height: "284",
                preserve_aspect_ratio: "xMidYMid meet",
                href: "{brand_mark()}",
            }
            // Wordmark: vector, inherits --wordmark from the theme.
            // textLength pins the advance width so VELVT can never clip the
            // 874-unit viewBox edge — the full word stays visible with any
            // font (even fallbacks). Anchored centre 600 keeps an even gap
            // from the emblem (ends at x=294) and the right edge (874).
            text {
                x: "600",
                y: "216",
                text_anchor: "middle",
                font_family: "'Cormorant Garamond', Georgia, serif",
                font_size: "172",
                letter_spacing: "9",
                text_length: "534",
                length_adjust: "spacingAndGlyphs",
                fill: "currentColor",
                "VELVT",
            }
        }
    }
}

/// Compact emblem-only mark, for square slots (favicon-like tiles, card
/// badges, the loader stage). Shares the same raster as the lockup.
#[component]
pub fn Emblem(class: String, title: Option<String>) -> Element {
    let label = title.clone().unwrap_or_else(|| "Velvt".to_string());
    rsx! {
        svg {
            class: "{class}",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 344 344",
            fill: "none",
            role: "img",
            "aria-label": "{label}",
            image {
                x: "0",
                y: "0",
                width: "344",
                height: "344",
                preserve_aspect_ratio: "xMidYMid meet",
                href: "{brand_mark()}",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(c: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(c);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[component]
    fn WrapWordmark() -> Element {
        rsx! {
            Wordmark {
                class: "v-wordmark--test".to_string(),
                title: Some("Velvt".to_string()),
            }
        }
    }

    #[component]
    fn WrapEmblem() -> Element {
        rsx! {
            Emblem {
                class: "v-emblem--test".to_string(),
                title: Some("Velvt".to_string()),
            }
        }
    }

    #[test]
    fn wordmark_is_inline_svg_with_currentcolor_text() {
        let html = render(WrapWordmark);
        assert!(html.contains("<svg"), "wordmark must be inline svg");
        assert!(
            html.contains("currentColor"),
            "wordmark text must inherit currentColor so CSS owns the colour"
        );
        assert!(html.contains("VELVT"), "wordmark text missing");
    }

    #[test]
    fn wordmark_reuses_the_single_emblem_asset() {
        let html = render(WrapWordmark);
        assert!(
            html.contains("velvet-mark"),
            "wordmark must reuse brand_mark() asset, got: {html}"
        );
    }

    #[test]
    fn wordmark_carries_viewbox_and_accessible_name() {
        let html = render(WrapWordmark);
        assert!(
            html.contains(&format!("0 0 {WORDMARK_VIEWBOX_W} {WORDMARK_VIEWBOX_H}")),
            "viewBox missing or wrong"
        );
        assert!(html.contains("aria-label"), "accessible name missing");
    }

    #[test]
    fn wordmark_applies_caller_class() {
        let html = render(WrapWordmark);
        assert!(html.contains("v-wordmark--test"), "class not applied");
    }

    #[test]
    fn emblem_is_square_viewbox_without_text() {
        let html = render(WrapEmblem);
        assert!(html.contains("0 0 344 344"), "emblem viewBox wrong");
        assert!(
            !html.contains("VELVT"),
            "emblem must not include the wordmark"
        );
    }

    /// The VELVT text must never clip against the 874-unit viewBox edge —
    /// pin its advance width with `textLength` so the word stays fully
    /// visible with any font (including fallbacks) in every theme.
    #[test]
    fn wordmark_text_guarantees_fit_inside_viewbox() {
        let html = render(WrapWordmark);
        assert!(
            html.contains("textLength"),
            "wordmark text must pin its advance width, got: {html}"
        );
    }

    #[test]
    fn lockup_aspect_ratio_is_stable() {
        // Guards the CSS/asset contract: the lockup is wide, never square.
        let ratio = WORDMARK_VIEWBOX_W as f32 / WORDMARK_VIEWBOX_H as f32;
        assert!(
            ratio > 2.4 && ratio < 2.7,
            "unexpected lockup ratio: {ratio}"
        );
    }
}
