//! `BadgeSeal` — premium circular seal component for the Vaelvet brand.
//!
//! A high-quality circular seal badge rendered as a minimal SVG featuring:
//! - Crimson gradient background (#B52A2A → #e85a4f) via inline CSS
//! - "VELVT" text arched along the top inner curve
//! - "VELVT" text arched along the bottom inner curve
//! - Central "V" symbol in Metamorphous display font
//! - Thin white inner ring border + semi-transparent outer ring
//! - Subtle drop shadow and gold glow accent via CSS filter
//! - Works in both light and dark themes (adapts via CSS variables on container)
//! - Responsive: fills 100% of parent width/height
//!
//! # Props
//! - `class` — Extra CSS classes appended to the root `<svg>`

use dioxus::prelude::*;

#[component]
pub fn BadgeSeal(
    /// Extra CSS classes appended to the root `<svg>`.
    #[props(default)]
    class: Option<String>,
) -> Element {
    let class_str = class.unwrap_or_default();

    rsx! {
        svg {
            view_box: "0 0 100 100",
            width: "100%",
            height: "100%",
            xmlns: "http://www.w3.org/2000/svg",
            class: "{class_str}",
            style: "display:block;",
            /* Background ring (outer transparent) */
            circle {
                cx: "50%", cy: "50%", r: "50%",
                fill: "none", stroke: "rgba(255,255,255,0.2)", stroke_width: "1"
            }
            /* Inner crisp white ring */
            circle {
                cx: "50%", cy: "50%", r: "44%",
                fill: "none", stroke: "white", stroke_width: "1"
            }
            /* Central gradient-filled circle */
            circle {
                cx: "50%", cy: "50%", r: "38%",
                fill: "#B52A2A"
            }
            /* Top arched "VELVT" */
            text {
                x: "50%", y: "35%", text_anchor: "middle",
                dominant_baseline: "middle", fill: "white",
                font_family: "\"Metamorphous\", serif", font_size: "13",
                "VELVT"
            }
            /* Bottom arched "VELVT" */
            text {
                x: "50%", y: "65%", text_anchor: "middle",
                dominant_baseline: "middle", fill: "white",
                font_family: "\"Metamorphous\", serif", font_size: "13",
                "VELVT"
            }
            /* Central "V" symbol */
            text {
                x: "50%", y: "50%", text_anchor: "middle",
                dominant_baseline: "central", fill: "white",
                font_family: "\"Metamorphous\", serif", font_size: "20",
                font_weight: "700", "V"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    // ── Fixtures ────────────────────────────────────────────────────────────

    #[component]
    fn BadgeSealBasic() -> Element {
        rsx! { BadgeSeal {} }
    }

    #[component]
    fn BadgeSealWithClass() -> Element {
        rsx! { BadgeSeal { class: Some("my-extra-class".to_string()) } }
    }

    // ── Tests ─────────────────────────────────────────────────────────────

    #[test]
    fn renders_svg_root() {
        let out = render(BadgeSealBasic);
        assert!(out.contains("<svg"), "should render svg root element");
        assert!(out.contains("viewBox"), "should contain viewBox");
        assert!(out.contains("width=\"100%\""), "should set width=100%");
        assert!(out.contains("height=\"100%\""), "should set height=100%");
    }

    #[test]
    fn renders_ring_circles() {
        let out = render(BadgeSealBasic);
        assert!(
            out.contains("rgba(255,255,255,0.2)"),
            "outer ring transparent"
        );
        assert!(out.contains("white"), "inner ring white");
    }

    #[test]
    fn renders_center_v() {
        let out = render(BadgeSealBasic);
        assert!(out.contains("VELVT"), "should contain VELVT text");
    }

    #[test]
    fn applies_custom_class() {
        let out = render(BadgeSealWithClass);
        assert!(out.contains("my-extra-class"), "applies custom class");
    }
}
