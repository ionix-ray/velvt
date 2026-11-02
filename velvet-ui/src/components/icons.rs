//! Inline SVG icons. Official Carbon Design System geometry (32px grid)
//! for the scope-of-work glyph set; hand-drawn strokes for nav/social marks.
//! All inline — no font, no remote fetch, no lazy load.

use dioxus::prelude::*;

#[component]
pub fn Icon(name: String) -> Element {
    match name.as_str() {
        "film" => rsx!(IconFilm {}),
        "shield" => rsx!(IconShield {}),
        "spark" => rsx!(IconSpark {}),
        "stage" => rsx!(IconStage {}),
        "feather" => rsx!(IconFeather {}),
        "arrow" => rsx!(IconArrow {}),
        "email" => rsx!(IconEmail {}),
        "location" => rsx!(IconLocation {}),
        "facebook" => rsx!(IconFacebook {}),
        "instagram" => rsx!(IconInstagram {}),
        "youtube" => rsx!(IconYoutube {}),
        "linkedin" | "in" => rsx!(IconLinkedin {}),
        "twitter" | "x" => rsx!(IconTwitter {}),
        "chevron" => rsx!(IconChevron {}),
        // ── Carbon scope-of-work glyphs — official Carbon Design System ──
        "camera" => rsx!(IconCamera {}),
        "microphone" | "mic" => rsx!(IconMicrophone {}),
        "video" => rsx!(IconVideo {}),
        "event" => rsx!(IconEvent {}),
        "search" => rsx!(IconSearch {}),
        "gift" => rsx!(IconGift {}),
        "star--filled" => rsx!(IconStarFilled {}),
        "location--filled" => rsx!(IconLocationFilled {}),
        "user--avatar" => rsx!(IconUserAvatar {}),
        "idea" => rsx!(IconIdea {}),
        "notification--filled" => rsx!(IconNotificationFilled {}),
        "aperture" => rsx!(IconAperture {}),
        "volume--up" => rsx!(IconVolumeUp {}),
        "user--multiple" => rsx!(IconUserMultiple {}),
        "image" => rsx!(IconImage {}),
        "flash--filled" => rsx!(IconFlashFilled {}),
        // ── Legacy ambient-glyph aliases (background config surface) ─────
        "venue" => rsx!(IconLocationFilled {}),
        "talent" => rsx!(IconUserMultiple {}),
        "grand-party" => rsx!(IconEvent {}),
        "creator" => rsx!(IconUserAvatar {}),
        "clapper" => rsx!(IconVideo {}),
        "lens" => rsx!(IconImage {}),
        "spotlight" => rsx!(IconFlashFilled {}),
        _ => rsx!(IconDot {}),
    }
}

/// Capitalize the first character of a configured icon/social slug — turns
/// `twitter` into `Twitter` for a presentable aria-label without pulling in
/// a casing crate. Shared by `footer_panel` and `social_strip` so both use
/// the same icon-label mechanism.
#[must_use]
pub fn pretty_label(label: &str) -> String {
    let mut chars = label.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[component]
fn IconTwitter() -> Element {
    // X / Twitter glyph — two diagonal strokes meeting at the centre.
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M22.4 4h4.6L18.6 14.2 28.8 28h-8.2l-6.4-8.4L6.6 28H2l9.1-10.9L1.2 4h8.4l5.8 7.7L22.4 4zm-1.4 21.3h2.5L11.1 6.6H8.4l12.6 18.7z" }
        }
    )
}

#[component]
fn IconFilm() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            rect { x: "4", y: "6", width: "24", height: "20", rx: "1" }
            path { d: "M4 11h24M4 16h24M4 21h24" }
            path { d: "M9 6v20M23 6v20" }
        }
    )
}
#[component]
fn IconShield() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 4l10 4v8c0 6-4 10-10 12-6-2-10-6-10-12V8l10-4z" }
            path { d: "M12 16l3 3 6-6" }
        }
    )
}
#[component]
fn IconSpark() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 4v6M16 22v6M4 16h6M22 16h6M8 8l4 4M20 20l4 4M8 24l4-4M20 12l4-4" }
            circle { cx: "16", cy: "16", r: "3" }
        }
    )
}
#[component]
fn IconStage() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M4 8c4 2 8 2 12 0s8-2 12 0v16H4V8z" }
            path { d: "M12 24v-8M20 24v-8" }
        }
    )
}
#[component]
fn IconFeather() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M26 6c2 6-2 14-8 18l-6 2 2-6c4-6 12-10 18-8-2 2-6 4-12 6" }
            path { d: "M10 20l-4 6" }
        }
    )
}
#[component]
fn IconArrow() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M6 16h20M20 10l6 6-6 6" }
        }
    )
}
#[component]
fn IconEmail() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            rect { x: "4", y: "8", width: "24", height: "16", rx: "1" }
            path { d: "M4 10l12 8 12-8" }
        }
    )
}
#[component]
fn IconLocation() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 28c-6-8-10-12-10-18a10 10 0 0120 0c0 6-4 10-10 18z" }
            circle { cx: "16", cy: "10", r: "3" }
        }
    )
}
#[component]
fn IconFacebook() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16 2C8.268 2 2 8.268 2 16s6.268 14 14 14 14-6.268 14-14S23.732 2 16 2zm4 10h-2c-.6 0-1 .4-1 1v2h3l-.5 3H17v10h-4V18h-3v-3h3v-2c0-2.8 2.2-5 5-5h2v3z" }
        }
    )
}
#[component]
fn IconInstagram() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16 2c3.9 0 4.4 0 5.9.1 1.5.1 2.5.3 3.4.6 1 .3 1.8.7 2.6 1.4.7.7 1.1 1.6 1.4 2.6.3.9.5 1.9.6 3.4 0 1.5.1 2 .1 5.9s0 4.4-.1 5.9c-.1 1.5-.3 2.5-.6 3.4-.3 1-.7 1.8-1.4 2.6-.7.7-1.6 1.1-2.6 1.4-.9.3-1.9.5-3.4.6-1.5 0-2 .1-5.9.1s-4.4 0-5.9-.1c-1.5-.1-2.5-.3-3.4-.6-1-.3-1.8-.7-2.6-1.4-.7-.7-1.1-1.6-1.4-2.6-.3-.9-.5-1.9-.6-3.4 0-1.5-.1-2-.1-5.9s0-4.4.1-5.9c.1-1.5.3-2.5.6-3.4.3-1 .7-1.8 1.4-2.6.7-.7 1.6-1.1 2.6-1.4.9-.3 1.9-.5 3.4-.6C11.6 2 12.1 2 16 2zm0 2.5c-3.9 0-4.3 0-5.8.1-1.3 0-2.1.3-2.6.5-.6.2-1.1.5-1.5.9-.4.4-.7.9-.9 1.5-.2.5-.4 1.3-.5 2.6 0 1.5-.1 1.9-.1 5.8s0 4.3.1 5.8c0 1.3.3 2.1.5 2.6.2.6.5 1.1.9 1.5.4.4.9.7 1.5.9.5.2 1.3.4 2.6.5 1.5 0 1.9.1 5.8.1s4.3 0 5.8-.1c1.3 0 2.1-.3 2.6-.5.6-.2 1.1-.5 1.5-.9.4-.4.7-.9.9-1.5.2-.5.4-1.3.5-2.6 0-1.5.1-1.9.1-5.8s0-4.3-.1-5.8c0-1.3-.3-2.1-.5-2.6-.2-.6-.5-1.1-.9-1.5-.4-.4-.9-.7-1.5-.9-.5-.2-1.3-.4-2.6-.5-1.5-.1-1.9-.1-5.8-.1z" }
            circle { cx: "16", cy: "16", r: "3.5" }
            circle { cx: "23", cy: "9", r: "1", fill: "currentColor" }
        }
    )
}
#[component]
fn IconYoutube() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M30.2 7.5c-.4-1.4-1.5-2.5-2.9-2.9C24.9 4 16 4 16 4s-8.9 0-11.3.6C3.3 5 2.2 6.1 1.8 7.5 1.2 9.9 1.2 16 1.2 16s0 6.1.6 8.5c.4 1.4 1.5 2.5 2.9 2.9 2.4.6 11.3.6 11.3.6s8.9 0 11.3-.6c1.4-.4 2.5-1.5 2.9-2.9.6-2.4.6-8.5.6-8.5s0-6.1-.6-8.5zM12.8 21V11l7.5 5-7.5 5z" }
        }
    )
}
#[component]
fn IconLinkedin() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M26 2H6C3.8 2 2 3.8 2 6v20c0 2.2 1.8 4 4 4h20c2.2 0 4-1.8 4-4V6c0-2.2-1.8-4-4-4zM10.5 24H7V12h3.5v12zM8.8 10.5C7.5 10.5 6.5 9.5 6.5 8.3s1-2.2 2.3-2.2 2.3 1 2.3 2.2-1 2.2-2.3 2.2zM25 24h-3.5v-5.8c0-1.4-.5-2.4-1.8-2.4s-2 .9-2 2.3V24h-3.5V12H17v1.6c.5-.8 1.4-1.6 3-1.6 2.1 0 3.7 1.4 3.7 4.4V24h1.3z" }
        }
    )
}
#[component]
fn IconChevron() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M8 12l8 8 8-8" }
        }
    )
}
#[component]
fn IconDot() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            circle { cx: "16", cy: "16", r: "2" }
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// AMBIENT GLYPH SET
// 32×32 grid, 1.5 stroke, `currentColor` throughout so the ambient field can
// tint each glyph independently. Geometry only — no third-party artwork, so
// nothing here carries a licence obligation.
// ═══════════════════════════════════════════════════════════════════════════

/// Carbon `camera` (camera) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconCamera() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M29,26H3a1,1,0,0,1-1-1V8A1,1,0,0,1,3,7H9.46l1.71-2.55A1,1,0,0,1,12,4h8a1,1,0,0,1,.83.45L22.54,7H29a1,1,0,0,1,1,1V25A1,1,0,0,1,29,26ZM4,24H28V9H22a1,1,0,0,1-.83-.45L19.46,6H12.54L10.83,8.55A1,1,0,0,1,10,9H4Z", }
            path { d: "M16,22a6,6,0,1,1,6-6A6,6,0,0,1,16,22Zm0-10a4,4,0,1,0,4,4A4,4,0,0,0,16,12Z", }
        }
    )
}

/// Carbon `microphone` (microphone) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconMicrophone() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M23,14v3A7,7,0,0,1,9,17V14H7v3a9,9,0,0,0,8,8.94V28H11v2H21V28H17V25.94A9,9,0,0,0,25,17V14Z", }
            path { d: "M16,22a5,5,0,0,0,5-5V7A5,5,0,0,0,11,7V17A5,5,0,0,0,16,22ZM13,7a3,3,0,0,1,6,0V17a3,3,0,0,1-6,0Z", }
        }
    )
}

/// Carbon `video` (video) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconVideo() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M21,26H4a2,2,0,0,1-2-2V8A2,2,0,0,1,4,6H21a2,2,0,0,1,2,2v4.06l5.42-3.87A1,1,0,0,1,30,9V23a1,1,0,0,1-1.58.81L23,19.94V24A2,2,0,0,1,21,26ZM4,8V24H21V18a1,1,0,0,1,1.58-.81L28,21.06V10.94l-5.42,3.87A1,1,0,0,1,21,14V8Z", }
        }
    )
}

/// Carbon `event` (event) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconEvent() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M28,6a2,2,0,0,0-2-2H22V2H20V4H12V2H10V4H6A2,2,0,0,0,4,6V26a2,2,0,0,0,2,2h4V26H6V6h4V8h2V6h8V8h2V6h4v6h2Z", }
            path { d: "M21 15 23.549 19.938 29 20.729 25 24.573 26 30 21 27.438 16 30 17 24.573 13 20.729 18.6 19.938 21 15z", }
        }
    )
}

/// Carbon `search` (search) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconSearch() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M29,27.5859l-7.5521-7.5521a11.0177,11.0177,0,1,0-1.4141,1.4141L27.5859,29ZM4,13a9,9,0,1,1,9,9A9.01,9.01,0,0,1,4,13Z", }
        }
    )
}

/// Carbon `gift` (gift) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconGift() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M26,10H23.2383A4.4867,4.4867,0,0,0,16,4.7065,4.4873,4.4873,0,0,0,8.7617,10H6a2.0025,2.0025,0,0,0-2,2v4a2.0025,2.0025,0,0,0,2,2V28a2.0025,2.0025,0,0,0,2,2H24a2.0025,2.0025,0,0,0,2-2V18a2.0025,2.0025,0,0,0,2-2V12A2.0025,2.0025,0,0,0,26,10ZM17,7.5A2.5,2.5,0,1,1,19.5,10H17ZM12.5,5A2.503,2.503,0,0,1,15,7.5V10H12.5a2.5,2.5,0,0,1,0-5ZM6,12h9v4H6Zm2,6h7V28H8ZM24.0012,28H17V18h7ZM17,16V12h9l.0012,4Z", }
        }
    )
}

/// Carbon `star--filled` (star filled) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconStarFilled() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16,2l-4.55,9.22L1.28,12.69l7.36,7.18L6.9,30,16,25.22,25.1,30,23.36,19.87l7.36-7.17L20.55,11.22Z", }
        }
    )
}

/// Carbon `location--filled` (location filled) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconLocationFilled() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16,2A11.0134,11.0134,0,0,0,5,13a10.8885,10.8885,0,0,0,2.2163,6.6s.3.3945.3482.4517L16,30l8.439-9.9526c.0444-.0533.3447-.4478.3447-.4478l.0015-.0024A10.8846,10.8846,0,0,0,27,13,11.0134,11.0134,0,0,0,16,2Zm0,15a4,4,0,1,1,4-4A4.0045,4.0045,0,0,1,16,17Z", }
            circle { cx: "16",cy: "13",r: "4",fill: "none", }
        }
    )
}

/// Carbon `user--avatar` (user avatar) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconUserAvatar() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16,8a5,5,0,1,0,5,5A5,5,0,0,0,16,8Zm0,8a3,3,0,1,1,3-3A3.0034,3.0034,0,0,1,16,16Z", }
            path { d: "M16,2A14,14,0,1,0,30,16,14.0158,14.0158,0,0,0,16,2ZM10,26.3765V25a3.0033,3.0033,0,0,1,3-3h6a3.0033,3.0033,0,0,1,3,3v1.3765a11.8989,11.8989,0,0,1-12,0Zm13.9925-1.4507A5.0016,5.0016,0,0,0,19,20H13a5.0016,5.0016,0,0,0-4.9925,4.9258,12,12,0,1,1,15.985,0Z", }
        }
    )
}

/// Carbon `idea` (idea) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconIdea() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M11 24H21V26H11z", }
            path { d: "M13 28H19V30H13z", }
            path { d: "M16,2A10,10,0,0,0,6,12a9.19,9.19,0,0,0,3.46,7.62c1,.93,1.54,1.46,1.54,2.38h2c0-1.84-1.11-2.87-2.19-3.86A7.2,7.2,0,0,1,8,12a8,8,0,0,1,16,0,7.2,7.2,0,0,1-2.82,6.14c-1.07,1-2.18,2-2.18,3.86h2c0-.92.53-1.45,1.54-2.39A9.18,9.18,0,0,0,26,12,10,10,0,0,0,16,2Z", }
        }
    )
}

/// Carbon `notification--filled` (notification filled) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconNotificationFilled() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M28.707,19.293,26,16.5859V13a10.0136,10.0136,0,0,0-9-9.9492V1H15V3.0508A10.0136,10.0136,0,0,0,6,13v3.5859L3.293,19.293A.9993.9993,0,0,0,3,20v3a1,1,0,0,0,1,1h7v1a5,5,0,0,0,10,0V24h7a1,1,0,0,0,1-1V20A.9993.9993,0,0,0,28.707,19.293ZM19,25a3,3,0,0,1-6,0V24h6Z", }
        }
    )
}

/// Carbon `aperture` (aperture) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconAperture() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16,2A14,14,0,1,0,30,16,14,14,0,0,0,16,2ZM27.84,14.14,22,17.52V5.62A12,12,0,0,1,27.84,14.14ZM12,18.68V13.32L16,11l4,2.31v5.36L16,21Zm8-14V11L10.34,5.42A11.9,11.9,0,0,1,20,4.7Zm-11.52,2L14,9.85,4,15.62A12,12,0,0,1,8.48,6.66ZM4.16,17.85,10,14.47V26.38A12,12,0,0,1,4.16,17.85ZM12,27.3V21l9.67,5.58A11.92,11.92,0,0,1,16,28,12.05,12.05,0,0,1,12,27.3Zm11.52-2L18,22.14l10-5.77A12,12,0,0,1,23.52,25.34Z", }
        }
    )
}

/// Carbon `volume--up` (volume up) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconVolumeUp() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M27.16,8.08,25.63,9.37a10,10,0,0,1-.29,13.23L26.81,24a12,12,0,0,0,.35-15.88Z", }
            path { d: "M21.58,12a6,6,0,0,1-.18,7.94l1.47,1.36a8,8,0,0,0,.23-10.59Z", }
            path { d: "M18,30a1,1,0,0,1-.71-.3L9.67,22H3a1,1,0,0,1-1-1H2V11a1,1,0,0,1,1-1H9.67l7.62-7.7a1,1,0,0,1,1.41,0A1,1,0,0,1,19,3V29A1,1,0,0,1,18,30ZM4,20h6.08a1,1,0,0,1,.71.3L17,26.57V5.43L10.79,11.7a1,1,0,0,1-.71.3H4Z", }
        }
    )
}

/// Carbon `user--multiple` (user multiple) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconUserMultiple() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M30,30H28V25a5.0057,5.0057,0,0,0-5-5V18a7.0078,7.0078,0,0,1,7,7Z", }
            path { d: "M22,30H20V25a5.0059,5.0059,0,0,0-5-5H9a5.0059,5.0059,0,0,0-5,5v5H2V25a7.0082,7.0082,0,0,1,7-7h6a7.0082,7.0082,0,0,1,7,7Z", }
            path { d: "M20,2V4a5,5,0,0,1,0,10v2A7,7,0,0,0,20,2Z", }
            path { d: "M12,4A5,5,0,1,1,7,9a5,5,0,0,1,5-5m0-2a7,7,0,1,0,7,7A7,7,0,0,0,12,2Z", }
        }
    )
}

/// Carbon `image` (image) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconImage() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M19,14a3,3,0,1,0-3-3A3,3,0,0,0,19,14Zm0-4a1,1,0,1,1-1,1A1,1,0,0,1,19,10Z", }
            path { d: "M26,4H6A2,2,0,0,0,4,6V26a2,2,0,0,0,2,2H26a2,2,0,0,0,2-2V6A2,2,0,0,0,26,4Zm0,22H6V20l5-5,5.59,5.59a2,2,0,0,0,2.82,0L21,19l5,5Zm0-4.83-3.59-3.59a2,2,0,0,0-2.82,0L18,19.17l-5.59-5.59a2,2,0,0,0-2.82,0L6,17.17V6H26Z", }
        }
    )
}

/// Carbon `flash--filled` (flash filled) — official Carbon Design System geometry, 32px grid.
#[component]
fn IconFlashFilled() -> Element {
    rsx!(
        svg { width: "1em", height: "1em", view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M11.61,29.92a1,1,0,0,1-.6-1.07L12.83,17H8a1,1,0,0,1-1-1.23l3-13A1,1,0,0,1,11,2H21a1,1,0,0,1,.78.37,1,1,0,0,1,.2.85L20.25,11H25a1,1,0,0,1,.9.56,1,1,0,0,1-.11,1l-13,17A1,1,0,0,1,12,30,1.09,1.09,0,0,1,11.61,29.92Z", }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn WrapIcon(icon_name: String) -> Element {
        rsx! {
            Icon { name: icon_name }
        }
    }

    /// Render one `Icon` by name to HTML. Uses `new_with_props` so the icon
    /// name can vary per call (a plain `fn() -> Element` root cannot).
    fn render_icon(icon_name: &str) -> String {
        let mut dom = VirtualDom::new_with_props(
            WrapIcon,
            WrapIconProps {
                icon_name: icon_name.to_string(),
            },
        );
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn every_ambient_glyph_resolves_to_real_geometry() {
        for name in [
            // Carbon scope set
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
            "user--multiple",
            "image",
            "flash--filled",
            // Legacy ambient aliases (background config surface)
            "lens",
            "venue",
            "talent",
            "grand-party",
            "creator",
            "clapper",
            "spotlight",
            "mic",
        ] {
            let html = render_icon(name);
            assert!(
                html.contains("<svg"),
                "{name} must render an svg, got: {html}"
            );
            assert!(
                !html.contains("<circle cx=\"16\" cy=\"16\" r=\"2\""),
                "{name} fell through to the IconDot fallback"
            );
        }
    }

    #[test]
    fn unknown_icon_falls_back_to_dot() {
        let html = render_icon("definitely-not-an-icon");
        assert!(html.contains("<circle"), "fallback dot missing");
    }

    #[test]
    fn pretty_label_capitalizes_first_char_or_empty() {
        assert_eq!(pretty_label("twitter"), "Twitter");
        assert_eq!(pretty_label("linkedin"), "Linkedin");
        assert_eq!(pretty_label(""), "");
    }
}
