//! Loading screen — retro progress bar with cycling creative text.

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
            // Camera-iris aperture: concentric rings pulse behind the mark
            // while it pulls into focus, instead of a flat opacity fade.
            div { class: "v-loader__iris", "aria-hidden": "true",
                div { class: "v-loader__iris-ring" }
            }
            div { class: "v-loader__brand",
                img { class: "v-loader__logo",
                      src: asset!("/assets/images/velvet-sqare.png"),
                      alt: "VELVT" }
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
        assert!(html.contains("v-loader__iris-ring"));
    }

    #[test]
    fn hidden_loader_carries_the_hidden_modifier() {
        let html = render(WrapHidden);
        assert!(html.contains("v-loader hidden"));
    }
}
