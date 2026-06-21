//! Integration tests for vaelvet-ui
//! Performs Server-Side Rendering (SSR) smoke tests to ensure all primary
//! panels and the main router can render without panicking or missing critical content.

use dioxus::prelude::*;

use vaelvet_ui::Site;

// We need to bring in the Home component to test it.
use vaelvet_ui::routes::home::Home;

#[test]
fn ssr_home_page_renders() {
    let mut vdom = VirtualDom::new(Home);
    vdom.rebuild_in_place();

    let html = dioxus_ssr::render(&vdom);

    // Smoke test: The page should contain critical IDs
    assert!(html.contains(r#"id="home""#));
    assert!(html.contains(r#"id="about""#));
    assert!(html.contains(r#"id="cases""#));
    assert!(html.contains(r#"id="contact""#));
}

#[test]
fn ssr_hero_panel_content() {
    // We can also test individual components if we make them public,
    // but testing via the Home page is sufficient for a high-level smoke test.
    let site = Site::load();
    let mut vdom = VirtualDom::new(Home);
    vdom.rebuild_in_place();
    let html = dioxus_ssr::render(&vdom);

    // Verify dynamic content from config is present
    assert!(html.contains(site.hero.headline1.as_ref()));
    assert!(html.contains(site.brand.name.as_ref()));
}
