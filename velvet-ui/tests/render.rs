//! End-to-end SSR render: load config, render Home, assert key copy survives.

use vaelvet_ui::Site;
use vaelvet_ui::routes::Home;

use dioxus::prelude::*;

#[test]
fn site_config_parses() {
    let s = Site::load();
    assert_eq!(s.brand.name, "Vaelvet");
    assert_eq!(s.brand.tagline, "elevate your Presence.");
    assert!(!s.nav.is_empty());
    assert!(!s.services.is_empty());
    assert!(!s.cases.is_empty());
}

#[test]
fn home_renders_brand_and_hero() {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("Vaelvet"),               "brand name must appear");
    assert!(html.contains("compose entrances"),     "hero headline must appear");
    assert!(html.contains("v-hero"),                "hero section class must appear");
    assert!(html.contains("v-services"),            "services grid class must appear");
    assert!(html.contains("v-manifesto"),           "manifesto class must appear");
    assert!(html.contains("v-footer"),              "footer class must appear");
}

#[test]
fn home_includes_all_services() {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    for name in [
        "Cinematic PR",
        "Crisis & Counsel",
        "Talent Curation",
        "Event Direction",
        "Editorial Placement",
    ] {
        assert!(html.contains(name), "service {name} must render");
    }
}

#[test]
fn home_includes_all_case_studies() {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    for client in ["A24", "House of Marais", "Project Lumen"] {
        assert!(html.contains(client), "case study {client} must render");
    }
}

#[test]
fn home_has_no_raw_unwrap_panics_in_output() {
    // Indirect sanity: nothing escaped a Result with an error marker.
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    assert!(!html.contains("Err("));
    assert!(!html.contains("panic"));
}
