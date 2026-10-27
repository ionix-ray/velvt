//! End-to-end SSR render: load config, render Home, assert key copy survives
//! and every panel is present (100 % coverage per coverage-goals.md).

use vaelvet_ui::Site;
use vaelvet_ui::components::dynamic_background::DynamicBackground;
use vaelvet_ui::config::{background_alpha, glyph_alpha};
use vaelvet_ui::routes::Home;

use dioxus::prelude::*;

fn render_home() -> String {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[component]
fn WrapBg() -> Element {
    rsx! {
        DynamicBackground { site: Site::load().clone() }
    }
}

fn render_bg() -> String {
    let mut dom = VirtualDom::new(WrapBg);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

// ── Config ─────────────────────────────────────────────────────────────────

#[test]
fn site_config_parses() {
    let s = Site::load();
    assert_eq!(s.brand.name.as_ref(), "Velvt");
    assert_eq!(
        s.brand.tagline.as_ref(),
        "We shape stories. You make history."
    );
    assert!(!s.nav.is_empty(), "nav must have items");
    assert!(!s.services.items.is_empty(), "services must have items");
    assert!(!s.cases.items.is_empty(), "case-studies must have items");
}

// ── Every one of the 6 panels must appear in the SSR output ────────────────

#[test]
fn panel_1_home_hero_renders() {
    let html = render_home();
    assert!(html.contains(r#"id="home""#), "panel #home must appear");
    assert!(html.contains("v-hero"), "hero class must appear");
    // The hero uses the cinematic 3-D logo wrapper (replaced the old v-hologram).
    assert!(
        html.contains("v-hero-3d-wrapper"),
        "hero 3d wrapper must appear"
    );
    assert!(
        html.contains("v-glass-effect"),
        "hero glass effect must appear"
    );
    assert!(
        html.contains("v-hero-3d-logo"),
        "hero 3d logo class must appear"
    );
    assert!(
        html.contains("v-hero__logo-svg")
            || html.contains("v-hero-3d-logo__img")
            || html.contains("v-hero-3d-logo__video"),
        "hero logo svg, img, or video must appear"
    );
}

#[test]
fn dynamic_background_renders_all_layers_with_config_alpha() {
    let html = render_bg();
    assert!(html.contains("v-dynamic-bg"), "bg root missing");
    assert!(
        html.contains("v-dynamic-bg__layer--slow"),
        "slow layer missing"
    );
    assert!(
        html.contains("v-dynamic-bg__layer--fast"),
        "fast layer missing"
    );
    assert!(html.contains("v-dynamic-bg__beam"), "beam missing");
    assert!(
        html.contains("aria-hidden=\"true\""),
        "decorative bg must be aria-hidden"
    );
    // Config drives the style contract: intensity → --v-bg-alpha and the
    // glyph share → --v-bg-glyph-alpha. Assert the wiring using the same
    // functions the component uses, not a magic number that drifts when the
    // brand tunes its background.
    let site = Site::load();
    let bg = &site.background;
    let alpha = background_alpha(bg.intensity);
    assert!(
        html.contains(&format!("--v-bg-alpha:{alpha}")),
        "config intensity must drive --v-bg-alpha, got: {html}"
    );
    if bg.show_glyphs {
        let glyph = glyph_alpha(bg.intensity, bg.glyph_opacity);
        assert!(
            html.contains(&format!("--v-bg-glyph-alpha:{glyph}")),
            "config glyph share must drive --v-bg-glyph-alpha, got: {html}"
        );
    }
}

#[test]
fn panel_2_about_renders() {
    let html = render_home();
    assert!(html.contains(r#"id="about""#), "panel #about must appear");
    assert!(html.contains("v-pillars"), "pillars class must appear");
}

#[test]
fn panel_3_process_renders() {
    let html = render_home();
    assert!(
        html.contains(r#"id="ideology""#),
        "panel #ideology must appear"
    );
    assert!(html.contains("v-process"), "process class must appear");
}

#[test]
fn panel_4_cases_renders() {
    let html = render_home();
    assert!(
        html.contains(r#"id="achivements""#),
        "panel #achivements must appear"
    );
    assert!(html.contains("v-cases"), "cases class must appear");
}

#[test]
fn panel_5_showcase_renders() {
    let html = render_home();
    assert!(
        html.contains(r#"id="experience""#)
            || html.contains(r#"id="studio""#)
            || html.contains("v-masonry"),
        "showcase or studio panel must appear"
    );
}

#[test]
fn panel_6_contact_renders() {
    let html = render_home();
    assert!(
        html.contains(r#"id="contact""#),
        "panel #contact (CTA) must appear"
    );
    assert!(html.contains("v-cta"), "cta class must appear");
}

#[test]
fn footer_renders() {
    let html = render_home();
    assert!(html.contains("v-footer"), "footer class must appear");
    assert!(html.contains("VELVT"), "brand name must appear in footer");
}

// ── Content completeness ────────────────────────────────────────────────────

#[test]
fn home_includes_all_case_studies() {
    let html = render_home();
    for client in [
        "Be the first to be showcased here",
        "Luxe Beauty",
        "GreenFuture",
    ] {
        assert!(html.contains(client), "case study {client} must render");
    }
}

// ── Safety ──────────────────────────────────────────────────────────────────

#[test]
fn home_has_no_raw_unwrap_panics_in_output() {
    let html = render_home();
    assert!(!html.contains("Err("), "no Err token in output");
    assert!(!html.contains("panic"), "no panic token in output");
}

#[test]
fn home_has_no_raw_unwrap_expect_in_output() {
    let html = render_home();
    assert!(!html.contains("unwrap"), "no unwrap token in output");
    assert!(!html.contains("expect"), "no expect token in output");
    assert!(!html.contains("todo!"), "no todo! token in output");
    assert!(
        !html.contains("unimplemented"),
        "no unimplemented token in output"
    );
}

// ── Structural ──────────────────────────────────────────────────────────────

// ── Reveal observer ⇄ CSS contract ─────────────────────────────────────────

/// `scroll::install_reveal` adds [`scroll::REVEALED_CLASS`] to every element
/// matching [`scroll::REVEAL_SELECTOR`]. The stylesheet must consume exactly
/// that class for both the entrance state (`.v-reveal*`) and the analytics
/// heatmap (`.v-heatmap-container`) — otherwise the observer is a no-op and
/// the heatmap cells stay at `opacity: 0` forever.
#[test]
fn theme_css_consumes_the_reveal_class_the_observer_adds() {
    use vaelvet_ui::scroll::{REVEAL_SELECTOR, REVEALED_CLASS};

    let css = include_str!("../assets/theme.css");
    assert_eq!(REVEALED_CLASS, "is-revealed");

    for hook in [
        ".v-reveal.is-revealed",
        ".v-reveal-left.is-revealed",
        ".v-reveal-right.is-revealed",
        ".v-heatmap-container.is-revealed",
    ] {
        assert!(
            css.contains(hook),
            "theme.css must style `{hook}` — the observer applies `{REVEALED_CLASS}`"
        );
    }

    // Every selector the observer watches must be a real selector the
    // stylesheet also keys off, and vice versa for the heatmap container.
    for token in REVEAL_SELECTOR.split(',') {
        let token = token.trim();
        assert!(
            token.starts_with('.'),
            "reveal selector `{token}` must be a class selector"
        );
        assert!(
            css.contains(token),
            "theme.css has no rule for observed selector `{token}`"
        );
    }
    assert!(
        REVEAL_SELECTOR.contains(".v-heatmap-container"),
        "the heatmap must be observed or its cells never leave opacity:0"
    );
}

/// The heatmap renders with the observer's hooks so `install_reveal` can
/// find it, and its cells carry the animation-delay stagger.
#[test]
fn heatmap_renders_with_observer_hooks_and_cells() {
    let html = render_home();
    assert!(
        html.contains("v-heatmap-container v-reveal"),
        "heatmap container must carry the reveal hook"
    );
    let cells = html.matches("v-heatmap-cell").count();
    assert!(
        cells >= 7 * 33,
        "7x33 heatmap grid must render, got {cells}"
    );
    assert!(
        html.contains("v-heatmap-cell--active"),
        "VELVT letter cells must be active"
    );
    assert!(
        html.contains("animation-delay:"),
        "heatmap cells stagger via animation-delay"
    );
}

#[test]
fn home_renders_loader_and_progress() {
    let html = render_home();
    assert!(html.contains("v-loader"), "loader element must exist");
    assert!(html.contains("v-progress"), "scroll progress must exist");
}

#[test]
fn home_renders_navigation_elements() {
    let html = render_home();
    assert!(html.contains("v-topbar"), "topbar must exist");
    assert!(html.contains("v-stack-nav"), "stacked nav must exist");
    assert!(html.contains("v-spindle"), "section spindle must exist");
}

#[test]
fn home_loader_uses_icon_mark_not_full_wordmark() {
    let html = render_home();
    // The loader must reference the velvet-mark glyph (the standalone icon)
    // rather than the full wordmark card.
    assert!(
        html.contains("velvet-mark"),
        "loader must use velvet-mark.webp as the loading symbol"
    );
}
