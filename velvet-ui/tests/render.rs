//! End-to-end SSR render: load config, render Home, assert key copy survives
//! and every panel is present (100 % coverage per coverage-goals.md).

use vaelvet_ui::Site;
use vaelvet_ui::routes::Home;

use dioxus::prelude::*;

fn render_home() -> String {
    let mut dom = VirtualDom::new(Home);
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
    assert!(html.contains(r#"id="achivements""#), "panel #achivements must appear");
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
    for client in ["Be the first to be showcased here", "Luxe Beauty", "GreenFuture"] {
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
    // Note: next-hint component was removed per user request; no longer asserted.
}

#[test]
fn home_loader_uses_icon_mark_not_full_wordmark() {
    let html = render_home();
    // The loader must reference only-logo.png (the standalone icon) rather than
    // the full velvet-square wordmark card.
    assert!(
        html.contains("only-logo"),
        "loader must use only-logo.png as the loading symbol"
    );
}
