//! Carbon-inspired inline SVG icons. 28×28 viewBox, currentColor strokes.
//! All inline — no font, no remote fetch, no lazy load.

use dioxus::prelude::*;

#[component]
pub fn Icon(name: String) -> Element {
    match name.as_str() {
        "film"     => rsx!(IconFilm {}),
        "shield"   => rsx!(IconShield {}),
        "spark"    => rsx!(IconSpark {}),
        "stage"    => rsx!(IconStage {}),
        "feather"  => rsx!(IconFeather {}),
        "arrow"    => rsx!(IconArrow {}),
        "email"    => rsx!(IconEmail {}),
        "location" => rsx!(IconLocation {}),
        "chevron"  => rsx!(IconChevron {}),
        _          => rsx!(IconDot {}),
    }
}

#[component] fn IconFilm() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        rect { x: "4", y: "6", width: "24", height: "20", rx: "1" }
        path { d: "M4 11h24M4 16h24M4 21h24" }
        path { d: "M9 6v20M23 6v20" }
    }
)}
#[component] fn IconShield() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M16 4l10 4v8c0 6-4 10-10 12-6-2-10-6-10-12V8l10-4z" }
        path { d: "M12 16l3 3 6-6" }
    }
)}
#[component] fn IconSpark() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M16 4v6M16 22v6M4 16h6M22 16h6M8 8l4 4M20 20l4 4M8 24l4-4M20 12l4-4" }
        circle { cx: "16", cy: "16", r: "3" }
    }
)}
#[component] fn IconStage() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M4 8c4 2 8 2 12 0s8-2 12 0v16H4V8z" }
        path { d: "M12 24v-8M20 24v-8" }
    }
)}
#[component] fn IconFeather() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M26 6c2 6-2 14-8 18l-6 2 2-6c4-6 12-10 18-8-2 2-6 4-12 6" }
        path { d: "M10 20l-4 6" }
    }
)}
#[component] fn IconArrow() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M6 16h20M20 10l6 6-6 6" }
    }
)}
#[component] fn IconEmail() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        rect { x: "4", y: "8", width: "24", height: "16", rx: "1" }
        path { d: "M4 10l12 8 12-8" }
    }
)}
#[component] fn IconLocation() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M16 28c-6-8-10-12-10-18a10 10 0 0120 0c0 6-4 10-10 18z" }
        circle { cx: "16", cy: "10", r: "3" }
    }
)}
#[component] fn IconChevron() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "none", stroke: "currentColor", stroke_width: "1.5",
        path { d: "M8 12l8 8 8-8" }
    }
)}
#[component] fn IconDot() -> Element { rsx!(
    svg { view_box: "0 0 32 32", fill: "currentColor",
        circle { cx: "16", cy: "16", r: "2" }
    }
)}
