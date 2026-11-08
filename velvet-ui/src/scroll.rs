//! Scroll-linked depth. Updates the `--scroll-y` CSS custom property on <body>
//! once per scroll event so the hero parallax planes can read it cheaply.
//!
//! Returns `Some(())` on success and `None` if any browser handle is missing —
//! we never panic, the page just runs without parallax.

use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Install the scroll listener. Call once from `App` startup.
#[must_use]
pub fn install() -> Option<()> {
    let win = web_sys::window()?;
    let doc = win.document()?;
    let body = doc.body()?;

    let win_for_cb = win.clone();

    let cb = Closure::<dyn FnMut()>::new(move || {
        let y = win_for_cb.scroll_y().unwrap_or(0.0);
        let _ = body.style().set_property("--scroll-y", &y.to_string());
    });

    let listener = cb.as_ref().unchecked_ref();
    if win
        .add_event_listener_with_callback("scroll", listener)
        .is_err()
    {
        return None;
    }
    cb.forget(); // intentional: listener lives for the page lifetime
    Some(())
}

/// The one reveal mechanism: the observer stamps this class on a target the
/// moment it crosses the viewport threshold, and `theme.css` keys both the
/// entrance state (`.v-reveal*`) and the analytics heatmap
/// (`.v-heatmap-container`) off it.
pub const REVEALED_CLASS: &str = "is-revealed";

/// Class selector list the reveal observer watches. Contract-tested against
/// `theme.css` by this module's tests and `tests/render.rs`.
pub const REVEAL_SELECTOR: &str =
    ".v-reveal, .v-reveal-left, .v-reveal-right, .v-heatmap-container";

thread_local! {
    /// Page-lifetime observer, kept alive so SPA route changes can feed
    /// newly mounted panels into it — see [`rescan_reveals`].
    static OBSERVER: RefCell<Option<web_sys::IntersectionObserver>> =
        const { RefCell::new(None) };
}

/// Observe every reveal target in `doc` that the observer has not already
/// revealed. Never panics: a missing DOM or a borrowed slot just means no
/// new observations this pass.
fn observe_pending(observer: &web_sys::IntersectionObserver, doc: &web_sys::Document) {
    let Ok(targets) = doc.query_selector_all(REVEAL_SELECTOR) else {
        return;
    };
    for i in 0..targets.length() {
        let Some(node) = targets.item(i) else {
            continue;
        };
        let Ok(el) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        if el.class_list().contains(REVEALED_CLASS) {
            continue;
        }
        observer.observe(&el);
    }
}

/// Reveal sections as they cross into view. Adds [`REVEALED_CLASS`] to any
/// element matching [`REVEAL_SELECTOR`]. Call once from `App` startup.
#[must_use]
pub fn install_reveal() -> Option<()> {
    let win = web_sys::window()?;
    let doc = win.document()?;

    let callback =
        Closure::<dyn FnMut(web_sys::js_sys::Array)>::new(|entries: web_sys::js_sys::Array| {
            for entry in entries.iter() {
                let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() else {
                    continue;
                };
                if entry.is_intersecting() {
                    if let Some(el) = entry.target().dyn_ref::<web_sys::Element>() {
                        let _ = el.class_list().add_1(REVEALED_CLASS);
                    }
                }
            }
        });

    let options = web_sys::IntersectionObserverInit::new();
    options.set_threshold(&wasm_bindgen::JsValue::from_f64(0.12));

    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    )
    .ok()?;
    callback.forget(); // intentional: observer lives for the page lifetime

    observe_pending(&observer, &doc);
    OBSERVER.with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = Some(observer);
        }
    });

    Some(())
}

/// Re-scan the document for reveal targets mounted after [`install_reveal`]
/// — SPA route changes build new panels that no observer has seen yet.
/// No-op before the first install or when the browser handles are missing.
pub fn rescan_reveals() {
    OBSERVER.with(|slot| {
        let Ok(slot) = slot.try_borrow() else {
            return;
        };
        let Some(observer) = slot.as_ref() else {
            return;
        };
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        observe_pending(observer, &doc);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The observer and the stylesheet must agree on ONE class name.
    #[test]
    fn revealed_class_is_the_one_theme_css_consumes() {
        assert_eq!(REVEALED_CLASS, "is-revealed");
        let css = concat!(
            include_str!("../assets/theme.css"),
            include_str!("../assets/css/tokens.css"),
            include_str!("../assets/css/typography.css"),
            include_str!("../assets/css/layout.css"),
            include_str!("../assets/css/components.css"),
            include_str!("../assets/css/animations.css"),
            include_str!("../assets/css/responsive.css"),
        );
        assert!(css.contains(".v-reveal.is-revealed"));
        assert!(css.contains(".v-heatmap-container.is-revealed"));
    }

    /// Every observed target must exist in the stylesheet, and the heatmap
    /// container must be observed or its cells stay at `opacity: 0`.
    #[test]
    fn reveal_selector_covers_entrance_states_and_the_heatmap() {
        let tokens: Vec<&str> = REVEAL_SELECTOR.split(',').map(str::trim).collect();
        assert_eq!(
            tokens,
            [
                ".v-reveal",
                ".v-reveal-left",
                ".v-reveal-right",
                ".v-heatmap-container"
            ]
        );
        let css = concat!(
            include_str!("../assets/theme.css"),
            include_str!("../assets/css/tokens.css"),
            include_str!("../assets/css/typography.css"),
            include_str!("../assets/css/layout.css"),
            include_str!("../assets/css/components.css"),
            include_str!("../assets/css/animations.css"),
            include_str!("../assets/css/responsive.css"),
        );
        for token in &tokens {
            assert!(css.contains(token), "theme.css missing rule for {token}");
        }
    }
}
