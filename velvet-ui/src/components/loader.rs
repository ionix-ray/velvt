//! Loading screen — retro progress bar with cycling creative text.

use crate::theme::brand::loader_symbol;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

const PHRASES: &[&str] = &[
    "Curating presence…",
    "Composing entrances…",
    "Seating the front row…",
    "Tuning the lights…",
    "Adjusting the lens…",
];

#[component]
pub fn Loader(hidden: bool) -> Element {
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

    let class = loader_class(hidden);
    let phrase = phrase_at(*phrase_index.read());

    rsx! {
        div { class: "{class}",
            // Brand mark + iris rings stacked in the same container so the
            // rings animate ON TOP of the icon image.
            div { class: "v-loader__brand",
                img { class: "v-loader__logo",
                      src: loader_symbol(),
                      alt: "VELVT" }
                // Camera-iris aperture: three blade rings step down around the
                // mark like a lens stopping down, then a flash marks focus.
                // Placed after the <img> in DOM order so it paints above it
                // via the same stacking context (position:absolute + higher z).
                div { class: "v-loader__iris", "aria-hidden": "true",
                    div { class: "v-loader__iris-ring v-loader__iris-ring--1" }
                    div { class: "v-loader__iris-ring v-loader__iris-ring--2" }
                    div { class: "v-loader__iris-ring v-loader__iris-ring--3" }
                    div { class: "v-loader__iris-flash" }
                }
            }
            div { class: "v-loader__bar",
                div { class: "v-loader__fill" }
            }
            div { class: "v-loader__phrase", key: "{phrase}", "{phrase}" }
        }
    }
}

fn loader_class(hidden: bool) -> &'static str {
    if hidden {
        "v-loader hidden"
    } else {
        "v-loader"
    }
}

fn phrase_at(index: usize) -> &'static str {
    PHRASES.get(index).copied().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_class_toggles_hidden_modifier() {
        assert_eq!(loader_class(false), "v-loader");
        assert_eq!(loader_class(true), "v-loader hidden");
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
    fn visible_loader_renders_camera_iris_curtain() {
        let html = render(WrapVisible);
        assert!(html.contains("v-loader__iris"));
        assert!(html.contains("v-loader__iris-ring--1"));
        assert!(html.contains("v-loader__iris-ring--2"));
        assert!(html.contains("v-loader__iris-ring--3"));
        assert!(html.contains("v-loader__iris-flash"));
    }

    /// The iris rings must be rendered INSIDE .v-loader__brand so they
    /// stack visually on top of the logo image (same stacking context).
    #[test]
    fn iris_rings_are_nested_inside_brand_container() {
        let html = render(WrapVisible);
        // Find the brand container…
        let brand_start = html.find("v-loader__brand").expect("brand container missing");
        // …and confirm the iris div appears after it (inside it).
        let iris_pos = html.find("v-loader__iris").expect("iris container missing");
        assert!(
            iris_pos > brand_start,
            "iris rings must be inside the brand container to overlay the logo"
        );
    }

    /// The loader must use the standalone icon mark, not the full wordmark card.
    #[test]
    fn loader_uses_only_logo_symbol_not_full_wordmark() {
        let html = render(WrapVisible);
        assert!(
            html.contains("only-logo"),
            "loader should reference only-logo.png, got: {html}"
        );
    }

    #[test]
    fn hidden_loader_carries_the_hidden_modifier() {
        let html = render(WrapHidden);
        assert!(html.contains("v-loader hidden"));
    }
}
