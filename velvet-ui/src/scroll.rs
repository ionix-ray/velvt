//! Scroll-linked depth. Updates the `--scroll-y` CSS custom property on <body>
//! once per scroll event so the hero parallax planes can read it cheaply.
//!
//! Returns `Some(())` on success and `None` if any browser handle is missing —
//! we never panic, the page just runs without parallax.

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

/// Reveal sections as they cross into view. Adds `in` class to any
/// element matching `.v-reveal`, `.v-reveal-left`, or `.v-reveal-right`.
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
                        let _ = el.class_list().add_1("in");
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

    let targets = doc
        .query_selector_all(".v-reveal, .v-reveal-left, .v-reveal-right")
        .ok()?;
    for i in 0..targets.length() {
        if let Some(node) = targets.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                observer.observe(&el);
            }
        }
    }

    Some(())
}
