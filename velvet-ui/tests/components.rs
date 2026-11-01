//! SSR contract tests for the stateless components that consume `Site`
//! config. Each test asserts config-driven content, real hrefs, or aria
//! wiring — no snapshot noise.

use dioxus::prelude::*;
use vaelvet_ui::Site;
use vaelvet_ui::components::social_strip::SocialStrip;

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[component]
fn StripVisible() -> Element {
    rsx! {
        SocialStrip { site: Site::load().clone(), is_last_panel: false }
    }
}

#[component]
fn StripOnFooter() -> Element {
    rsx! {
        SocialStrip { site: Site::load().clone(), is_last_panel: true }
    }
}

// ── SocialStrip ─────────────────────────────────────────────────────────────

/// The strip must render exactly the configured footer socials — one source
/// of truth (`content/site.md` `[[footer.socials]]`), no hardcoded network.
#[test]
fn social_strip_hrefs_match_site_footer_socials() {
    let site = Site::load();
    assert!(
        !site.footer.socials.is_empty(),
        "footer.socials drives the strip — it must not be empty"
    );

    let html = render(StripVisible);
    for social in site.footer.socials.iter() {
        assert!(
            html.contains(&format!("href=\"{}\"", social.href)),
            "strip must link to configured href {}",
            social.href
        );
    }

    // Nothing left over from the old hardcoded set.
    for stale in ["facebook.com/velvt", "youtube.com/@velvt"] {
        assert!(
            !html.contains(stale),
            "hardcoded social URL {stale} must not survive"
        );
    }
}

#[test]
fn social_strip_labels_and_aria_match_the_icon_names() {
    let html = render(StripVisible);
    let site = Site::load();
    let links = site.footer.socials.len();
    assert_eq!(
        html.matches("v-social-strip__link").count(),
        links,
        "one strip link per configured social"
    );
    for social in site.footer.socials.iter() {
        // `Icon { name }` resolves from the same label string the aria uses.
        assert!(
            html.contains(&format!(
                "aria-label=\"Velvt on {}",
                capitalize(&social.label)
            )),
            "aria-label missing for {}",
            social.label
        );
    }
}

#[test]
fn social_strip_hides_and_marks_aria_hidden_on_the_footer_panel() {
    let html = render(StripOnFooter);
    assert!(html.contains("v-social-strip--hidden"));
    assert!(
        html.contains("aria-hidden=\"true\""),
        "a hidden strip must be aria-hidden"
    );
}

/// Small local helper mirroring `footer_panel::pretty_label` so the test can
/// state the expected label without importing implementation detail.
fn capitalize(label: &str) -> String {
    let mut chars = label.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}
