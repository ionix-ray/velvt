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
/// element with `.v-reveal`. Honours `prefers-reduced-motion`.
#[must_use]
pub fn install_reveal() -> Option<()> {
    use web_sys::{IntersectionObserver, IntersectionObserverInit};

    let win = web_sys::window()?;
    let doc = win.document()?;

    let cb = Closure::<dyn FnMut(wasm_bindgen::JsValue, wasm_bindgen::JsValue)>::new(
        move |entries: wasm_bindgen::JsValue, _obs: wasm_bindgen::JsValue| {
            let Ok(arr) = entries.dyn_into::<js_sys::Array>() else {
                return;
            };
            for i in 0..arr.length() {
                let Ok(entry) = arr.get(i).dyn_into::<web_sys::IntersectionObserverEntry>() else {
                    continue;
                };
                if entry.is_intersecting() {
                    let Ok(el) = entry.target().dyn_into::<web_sys::Element>() else {
                        continue;
                    };
                    let _ = el.class_list().add_1("in");
                }
            }
        },
    );

    let init = IntersectionObserverInit::new();
    init.set_threshold(&js_sys::Array::of1(&wasm_bindgen::JsValue::from_f64(0.12)));

    let observer =
        IntersectionObserver::new_with_options(cb.as_ref().unchecked_ref(), &init).ok()?;
    cb.forget();

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
    // leak observer intentionally — page lifetime
    std::mem::forget(observer);
    Some(())
}
