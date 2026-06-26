//! Carbon-inspired inline SVG icons. 28×28 viewBox, currentColor strokes.
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
        _ => rsx!(IconDot {}),
    }
}

#[component]
fn IconTwitter() -> Element {
    // X / Twitter glyph — two diagonal strokes meeting at the centre.
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M22.4 4h4.6L18.6 14.2 28.8 28h-8.2l-6.4-8.4L6.6 28H2l9.1-10.9L1.2 4h8.4l5.8 7.7L22.4 4zm-1.4 21.3h2.5L11.1 6.6H8.4l12.6 18.7z" }
        }
    )
}

#[component]
fn IconFilm() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            rect { x: "4", y: "6", width: "24", height: "20", rx: "1" }
            path { d: "M4 11h24M4 16h24M4 21h24" }
            path { d: "M9 6v20M23 6v20" }
        }
    )
}
#[component]
fn IconShield() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 4l10 4v8c0 6-4 10-10 12-6-2-10-6-10-12V8l10-4z" }
            path { d: "M12 16l3 3 6-6" }
        }
    )
}
#[component]
fn IconSpark() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 4v6M16 22v6M4 16h6M22 16h6M8 8l4 4M20 20l4 4M8 24l4-4M20 12l4-4" }
            circle { cx: "16", cy: "16", r: "3" }
        }
    )
}
#[component]
fn IconStage() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M4 8c4 2 8 2 12 0s8-2 12 0v16H4V8z" }
            path { d: "M12 24v-8M20 24v-8" }
        }
    )
}
#[component]
fn IconFeather() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M26 6c2 6-2 14-8 18l-6 2 2-6c4-6 12-10 18-8-2 2-6 4-12 6" }
            path { d: "M10 20l-4 6" }
        }
    )
}
#[component]
fn IconArrow() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M6 16h20M20 10l6 6-6 6" }
        }
    )
}
#[component]
fn IconEmail() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            rect { x: "4", y: "8", width: "24", height: "16", rx: "1" }
            path { d: "M4 10l12 8 12-8" }
        }
    )
}
#[component]
fn IconLocation() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M16 28c-6-8-10-12-10-18a10 10 0 0120 0c0 6-4 10-10 18z" }
            circle { cx: "16", cy: "10", r: "3" }
        }
    )
}
#[component]
fn IconFacebook() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16 2C8.268 2 2 8.268 2 16s6.268 14 14 14 14-6.268 14-14S23.732 2 16 2zm4 10h-2c-.6 0-1 .4-1 1v2h3l-.5 3H17v10h-4V18h-3v-3h3v-2c0-2.8 2.2-5 5-5h2v3z" }
        }
    )
}
#[component]
fn IconInstagram() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M16 2c3.9 0 4.4 0 5.9.1 1.5.1 2.5.3 3.4.6 1 .3 1.8.7 2.6 1.4.7.7 1.1 1.6 1.4 2.6.3.9.5 1.9.6 3.4 0 1.5.1 2 .1 5.9s0 4.4-.1 5.9c-.1 1.5-.3 2.5-.6 3.4-.3 1-.7 1.8-1.4 2.6-.7.7-1.6 1.1-2.6 1.4-.9.3-1.9.5-3.4.6-1.5 0-2 .1-5.9.1s-4.4 0-5.9-.1c-1.5-.1-2.5-.3-3.4-.6-1-.3-1.8-.7-2.6-1.4-.7-.7-1.1-1.6-1.4-2.6-.3-.9-.5-1.9-.6-3.4 0-1.5-.1-2-.1-5.9s0-4.4.1-5.9c.1-1.5.3-2.5.6-3.4.3-1 .7-1.8 1.4-2.6.7-.7 1.6-1.1 2.6-1.4.9-.3 1.9-.5 3.4-.6C11.6 2 12.1 2 16 2zm0 2.5c-3.9 0-4.3 0-5.8.1-1.3 0-2.1.3-2.6.5-.6.2-1.1.5-1.5.9-.4.4-.7.9-.9 1.5-.2.5-.4 1.3-.5 2.6 0 1.5-.1 1.9-.1 5.8s0 4.3.1 5.8c0 1.3.3 2.1.5 2.6.2.6.5 1.1.9 1.5.4.4.9.7 1.5.9.5.2 1.3.4 2.6.5 1.5 0 1.9.1 5.8.1s4.3 0 5.8-.1c1.3 0 2.1-.3 2.6-.5.6-.2 1.1-.5 1.5-.9.4-.4.7-.9.9-1.5.2-.5.4-1.3.5-2.6 0-1.5.1-1.9.1-5.8s0-4.3-.1-5.8c0-1.3-.3-2.1-.5-2.6-.2-.6-.5-1.1-.9-1.5-.4-.4-.9-.7-1.5-.9-.5-.2-1.3-.4-2.6-.5-1.5-.1-1.9-.1-5.8-.1z" }
            circle { cx: "16", cy: "16", r: "3.5" }
            circle { cx: "23", cy: "9", r: "1", fill: "currentColor" }
        }
    )
}
#[component]
fn IconYoutube() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M30.2 7.5c-.4-1.4-1.5-2.5-2.9-2.9C24.9 4 16 4 16 4s-8.9 0-11.3.6C3.3 5 2.2 6.1 1.8 7.5 1.2 9.9 1.2 16 1.2 16s0 6.1.6 8.5c.4 1.4 1.5 2.5 2.9 2.9 2.4.6 11.3.6 11.3.6s8.9 0 11.3-.6c1.4-.4 2.5-1.5 2.9-2.9.6-2.4.6-8.5.6-8.5s0-6.1-.6-8.5zM12.8 21V11l7.5 5-7.5 5z" }
        }
    )
}
#[component]
fn IconLinkedin() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            path { d: "M26 2H6C3.8 2 2 3.8 2 6v20c0 2.2 1.8 4 4 4h20c2.2 0 4-1.8 4-4V6c0-2.2-1.8-4-4-4zM10.5 24H7V12h3.5v12zM8.8 10.5C7.5 10.5 6.5 9.5 6.5 8.3s1-2.2 2.3-2.2 2.3 1 2.3 2.2-1 2.2-2.3 2.2zM25 24h-3.5v-5.8c0-1.4-.5-2.4-1.8-2.4s-2 .9-2 2.3V24h-3.5V12H17v1.6c.5-.8 1.4-1.6 3-1.6 2.1 0 3.7 1.4 3.7 4.4V24h1.3z" }
        }
    )
}
#[component]
fn IconChevron() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
            path { d: "M8 12l8 8 8-8" }
        }
    )
}
#[component]
fn IconDot() -> Element {
    rsx!(
        svg { view_box: "0 0 32 32", fill: "currentColor",
            circle { cx: "16", cy: "16", r: "2" }
        }
    )
}
