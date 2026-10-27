//! Loading screen — cinematic icon swarm that assembles the V logo.

use crate::components::icons::Icon;
use crate::theme::wordmark::Emblem;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

/// First visit in a session: full icon-swarm logo build, then curtain lift.
pub const INTRO_MS: u32 = 2_600;
/// Return visit in the same session: fast fade, swarm skipped entirely.
pub const REVISIT_MS: u32 = 700;

const SESSION_KEY: &str = "velvt-intro-seen";

/// Returns `true` when this session already played the intro, marking the
/// session as seen on first call. Browser-only (`sessionStorage`) — callers
/// must run inside `use_effect` so SSR/native builds never touch it.
pub fn intro_seen_and_mark() -> bool {
    let Some(win) = web_sys::window() else {
        return true;
    };
    let Ok(Some(storage)) = win.session_storage() else {
        return false; // storage unavailable — play the intro
    };
    match storage.get_item(SESSION_KEY) {
        Ok(Some(_)) => true,
        Ok(None) => {
            let _ = storage.set_item(SESSION_KEY, "1");
            false
        }
        Err(_) => false,
    }
}

const PHRASES: &[&str] = &[
    "Curating presence…",
    "Composing entrances…",
    "Seating the front row…",
    "Tuning the lights…",
    "Adjusting the lens…",
];

/// One icon in the logo-build swarm: scope-of-work glyph in a brand tone,
/// flying in from a viewport edge (`sx`/`sy`) to its spot on the V
/// formation (`fx`/`fy`, px relative to the emblem centre).
struct SwarmIcon {
    name: &'static str,
    tone: &'static str,
    fx: &'static str,
    fy: &'static str,
    sx: &'static str,
    sy: &'static str,
    delay: &'static str,
}

/// 13 official Carbon glyphs arranged as the letter V: two legs meeting at a
/// vertex that sits on the emblem axis. Left leg follows y = x + 84, right leg
/// y = 84 − x, vertex at (0, 84). Tones cycle the brand glyph palette.
#[rustfmt::skip]
const SWARM: &[SwarmIcon] = &[
    // Left leg — photography → podcast → video → events → marketing → gifting
    SwarmIcon { name: "camera", tone: "crimson", fx: "-168px", fy: "-84px", sx: "-75vw", sy: "-20vh", delay: "0ms" },
    SwarmIcon { name: "microphone", tone: "violet", fx: "-140px", fy: "-56px", sx: "-40vw", sy: "-65vh", delay: "50ms" },
    SwarmIcon { name: "video", tone: "sky", fx: "-112px", fy: "-28px", sx: "-10vw", sy: "-80vh", delay: "100ms" },
    SwarmIcon { name: "event", tone: "citron", fx: "-84px", fy: "0px", sx: "-80vw", sy: "25vh", delay: "150ms" },
    SwarmIcon { name: "search", tone: "violet", fx: "-56px", fy: "28px", sx: "-30vw", sy: "70vh", delay: "200ms" },
    SwarmIcon { name: "gift", tone: "crimson", fx: "-28px", fy: "56px", sx: "10vw", sy: "85vh", delay: "250ms" },
    // Vertex — the celebrity star where the V meets
    SwarmIcon { name: "star--filled", tone: "crimson", fx: "0px", fy: "84px", sx: "0vw", sy: "90vh", delay: "320ms" },
    // Right leg — venue → creators → branding → PR → optics → advertising
    SwarmIcon { name: "location--filled", tone: "sky", fx: "168px", fy: "-84px", sx: "75vw", sy: "-25vh", delay: "100ms" },
    SwarmIcon { name: "user--avatar", tone: "crimson", fx: "140px", fy: "-56px", sx: "40vw", sy: "-65vh", delay: "150ms" },
    SwarmIcon { name: "idea", tone: "citron", fx: "112px", fy: "-28px", sx: "10vw", sy: "-85vh", delay: "200ms" },
    SwarmIcon { name: "notification--filled", tone: "violet", fx: "84px", fy: "0px", sx: "80vw", sy: "20vh", delay: "250ms" },
    SwarmIcon { name: "aperture", tone: "sky", fx: "56px", fy: "28px", sx: "35vw", sy: "70vh", delay: "300ms" },
    SwarmIcon { name: "volume--up", tone: "citron", fx: "28px", fy: "56px", sx: "-5vw", sy: "85vh", delay: "350ms" },
];

#[component]
pub fn Loader(hidden: bool, #[props(default = false)] revisit: bool) -> Element {
    let mut phrase_index = use_signal(|| 0);

    use_effect(move || {
        let index = *phrase_index.read();
        if index >= PHRASES.len().saturating_sub(1) {
            return;
        }
        spawn(async move {
            TimeoutFuture::new(400).await;
            phrase_index.set(index + 1);
        });
    });

    let class = loader_class(hidden, revisit);
    let phrase = phrase_at(*phrase_index.read());

    rsx! {
        div { class: "{class}",
            // Brand stage: fixed-size container the icon swarm flies into
            // and collapses onto the mark at the centre.
            div { class: "v-loader__brand",
                // Scope icons fly in from every edge, assemble into the V
                // formation, then collapse into the emblem (behind it, z:0).
                div { class: "v-loader__swarm", "aria-hidden": "true",
                    for icon in SWARM {
                        span {
                            class: "v-loader__icon v-loader__icon--{icon.tone}",
                            "data-name": "{icon.name}",
                            style: "--fx: {icon.fx}; --fy: {icon.fy}; --sx: {icon.sx}; --sy: {icon.sy}; --d: {icon.delay};",
                            Icon { name: icon.name.to_string() }
                        }
                    }
                }
                Emblem {
                    class: "v-loader__logo".to_string(),
                    title: Some("Velvt".to_string()),
                }
                div { class: "v-logo-sheen", "aria-hidden": "true" }
            }
            div { class: "v-loader__bar",
                div { class: "v-loader__fill" }
            }
            div { class: "v-loader__phrase", key: "{phrase}", "{phrase}" }
        }
    }
}

fn loader_class(hidden: bool, revisit: bool) -> String {
    let mut class = String::from("v-loader");
    if revisit {
        class.push_str(" v-loader--revisit");
    }
    if hidden {
        class.push_str(" hidden");
    }
    class
}

fn phrase_at(index: usize) -> &'static str {
    PHRASES.get(index).copied().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_class_toggles_hidden_modifier() {
        assert_eq!(loader_class(false, false), "v-loader");
        assert_eq!(loader_class(true, false), "v-loader hidden");
    }

    #[test]
    fn loader_class_marks_revisit_visits() {
        assert_eq!(loader_class(false, true), "v-loader v-loader--revisit");
        assert_eq!(
            loader_class(true, true),
            "v-loader v-loader--revisit hidden"
        );
    }

    #[test]
    fn visible_loader_renders_icon_swarm_building_the_logo() {
        let html = render(WrapVisible);
        assert!(html.contains("v-loader__swarm"), "swarm container missing");
        // One tone modifier per swarm icon: 12 leg icons + 1 vertex icon.
        assert_eq!(
            html.matches("v-loader__icon--").count(),
            13,
            "expected 13 swarm icons, got: {html}"
        );
        for tone in ["crimson", "violet", "sky", "citron"] {
            assert!(
                html.contains(&format!("v-loader__icon--{tone}")),
                "tone {tone} missing from swarm"
            );
        }
        // V-formation final positions + entry offsets + stagger delays.
        assert!(html.contains("--fx"), "V-formation fx vars missing");
        assert!(html.contains("--sx"), "entry offsets missing");
        assert!(html.contains("--d:"), "stagger delays missing");
        // Vertex of the V sits on the emblem axis.
        assert!(html.contains("--fx: 0"), "vertex icon missing");
    }

    #[test]
    fn swarm_icons_carry_scope_glyph_names() {
        let html = render(WrapVisible);
        for name in [
            "camera",
            "microphone",
            "video",
            "event",
            "search",
            "gift",
            "star--filled",
            "location--filled",
            "user--avatar",
            "idea",
            "notification--filled",
            "aperture",
            "volume--up",
        ] {
            assert!(
                html.contains(&format!("data-name=\"{name}\"")),
                "swarm icon {name} missing"
            );
        }
    }

    #[test]
    fn swarm_is_aria_hidden_and_never_interactive() {
        let html = render(WrapVisible);
        let swarm_pos = html.find("v-loader__swarm").unwrap_or(html.len());
        let slice = &html[swarm_pos..swarm_pos.saturating_add(400).min(html.len())];
        assert!(
            slice.contains("aria-hidden=\"true\""),
            "swarm must be aria-hidden"
        );
    }

    #[test]
    fn phrase_at_returns_each_phrase_in_order_and_empty_when_out_of_range() {
        for (i, p) in PHRASES.iter().enumerate() {
            assert_eq!(phrase_at(i), *p);
        }
        assert_eq!(phrase_at(PHRASES.len()), "");
        assert_eq!(phrase_at(999), "");
    }

    #[component]
    fn WrapVisible() -> Element {
        rsx! {
            Loader { hidden: false }
        }
    }

    #[component]
    fn WrapHidden() -> Element {
        rsx! {
            Loader { hidden: true }
        }
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn visible_loader_renders_brand_bar_and_first_phrase() {
        let html = render(WrapVisible);
        assert!(html.contains("v-loader"));
        assert!(!html.contains("v-loader hidden"));
        assert!(html.contains("v-loader__bar"));
        assert!(html.contains(PHRASES[0]));
    }

    #[test]
    fn intro_has_no_dark_iris_rings() {
        let html = render(WrapVisible);
        assert!(
            !html.contains("v-loader__iris"),
            "iris rings removed by design — dark circles must not return: {html}"
        );
    }

    /// The loader must use the standalone icon mark, not the full wordmark card.
    #[test]
    fn loader_uses_only_logo_symbol_not_full_wordmark() {
        let html = render(WrapVisible);
        assert!(
            html.contains("velvet-mark"),
            "loader should reference velvet-mark.webp, got: {html}"
        );
    }

    #[test]
    fn hidden_loader_carries_the_hidden_modifier() {
        let html = render(WrapHidden);
        assert!(html.contains("v-loader hidden"));
    }
}
