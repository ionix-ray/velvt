//! `CardStep` — shared, config-driven transparent card with:
//!   - Sparkling running border (`.v-sparkle-border`)
//!   - Logo badge in the top-left corner (`.v-experience-badge`)
//!   - Optional step number, tag label, title, and body text
//!
//! Used by `ProcessPanel` (numbered steps) and `StudioPanel` (tagged tiles).
//! Single source of truth for the cinematic card design pattern so any
//! future visual change propagates everywhere automatically.

use dioxus::prelude::*;

/// Config-driven transparent cinematic card.
///
/// # Props
/// - `logo`        — URL of the brand mark to render as a badge (top-left).
/// - `num`         — Optional step number (e.g. "01"). Rendered in a circle.
/// - `tag`         — Optional eyebrow label (e.g. "Events"). Mutually useful with `num`.
/// - `title`       — Card heading text.
/// - `body`        — Card body paragraph text.
/// - `class_extra` — Additional CSS classes appended to the card root element.
/// - `delay_ms`    — CSS `animation-delay` / `transition-delay` in milliseconds.
#[component]
pub fn CardStep(
    /// Brand mark URL (single source of truth via `site.hero.logo`).
    logo: String,
    /// Optional step number (displayed in a circle above the title).
    #[props(default)]
    num: Option<String>,
    /// Optional eyebrow / tag label displayed above the title.
    #[props(default)]
    tag: Option<String>,
    /// Card headline.
    title: String,
    /// Card body paragraph.
    body: String,
    /// Extra CSS class(es) appended to the card root `<div>`.
    #[props(default)]
    class_extra: String,
    /// Animation / transition delay in milliseconds.
    #[props(default)]
    delay_ms: usize,
) -> Element {
    let base_class = if class_extra.is_empty() {
        "v-process__step v-reveal".to_string()
    } else {
        format!("v-process__step v-reveal {class_extra}")
    };

    let delay_style = if delay_ms > 0 {
        format!("animation-delay:{delay_ms}ms;transition-delay:{delay_ms}ms;")
    } else {
        String::new()
    };

    rsx! {
        div { class: "{base_class}", style: "{delay_style}",
            // Running sparkling border — pure CSS conic-gradient animation.
            div { class: "v-sparkle-border" }
            // Brand mark badge — top-left corner branding per design system.
            img { class: "v-experience-badge", src: "{logo}", alt: "Velvt" }
            // Step number (ProcessPanel) — renders inside a circle.
            if let Some(n) = &num {
                div { class: "v-process__num", "{n}" }
            }
            // Eyebrow tag (StudioPanel tiles) — rendered above the title.
            if let Some(t) = &tag {
                span { class: "v-eyebrow", "{t}" }
            }
            h4 { "{title}" }
            p { "{body}" }
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
    fn WithNum() -> Element {
        rsx! {
            CardStep {
                logo: "/assets/images/velvet-square.png".to_string(),
                num: Some("01".to_string()),
                title: "Discover".to_string(),
                body: "Deep research into brand, audience, and landscape.".to_string(),
                delay_ms: 120,
            }
        }
    }

    #[component]
    fn WithTag() -> Element {
        rsx! {
            CardStep {
                logo: "/assets/images/velvet-square.png".to_string(),
                tag: Some("Events".to_string()),
                title: "Grand Soirée".to_string(),
                body: "An unforgettable brand launch evening.".to_string(),
                class_extra: "v-tile--showcase".to_string(),
                delay_ms: 60,
            }
        }
    }

    #[component]
    fn Minimal() -> Element {
        rsx! {
            CardStep {
                logo: "/assets/images/velvet-square.png".to_string(),
                title: "Simple Card".to_string(),
                body: "Body text.".to_string(),
            }
        }
    }

    // ── Structural tests ────────────────────────────────────────────────────

    #[test]
    fn renders_sparkle_border_and_badge() {
        let html = render(WithNum);
        assert!(html.contains("v-sparkle-border"), "sparkle border missing");
        assert!(html.contains("v-experience-badge"), "logo badge missing");
        assert!(html.contains("velvet-square.png"), "logo src missing");
    }

    #[test]
    fn renders_step_number_when_provided() {
        let html = render(WithNum);
        assert!(
            html.contains("v-process__num"),
            "step number circle missing"
        );
        assert!(html.contains("01"), "step number text missing");
        // Tag eyebrow must NOT appear when only num is set.
        assert!(!html.contains("v-eyebrow"), "unexpected eyebrow tag");
    }

    #[test]
    fn renders_eyebrow_tag_when_provided() {
        let html = render(WithTag);
        assert!(html.contains("v-eyebrow"), "eyebrow tag missing");
        assert!(html.contains("Events"), "eyebrow text missing");
        // Step number must NOT appear when only tag is set.
        assert!(
            !html.contains("v-process__num"),
            "unexpected step number circle"
        );
    }

    #[test]
    fn appends_class_extra_to_root_div() {
        let html = render(WithTag);
        assert!(
            html.contains("v-tile--showcase"),
            "extra class not appended to root div"
        );
    }

    #[test]
    fn applies_delay_style_when_nonzero() {
        let html = render(WithNum);
        assert!(
            html.contains("animation-delay:120ms"),
            "animation-delay not set"
        );
        assert!(
            html.contains("transition-delay:120ms"),
            "transition-delay not set"
        );
    }

    #[test]
    fn no_delay_style_when_zero() {
        let html = render(Minimal);
        assert!(
            !html.contains("animation-delay"),
            "unexpected delay style on zero-delay card"
        );
    }

    #[test]
    fn renders_title_and_body() {
        let html = render(WithNum);
        assert!(html.contains("Discover"), "title missing");
        assert!(html.contains("Deep research"), "body text missing");
    }

    #[test]
    fn minimal_card_has_base_classes_only() {
        let html = render(Minimal);
        assert!(html.contains("v-process__step"), "base class missing");
        assert!(html.contains("v-reveal"), "reveal class missing");
        assert!(
            !html.contains("v-tile--showcase"),
            "unexpected extra class on minimal card"
        );
    }
}
