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

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
export function init_reveal_js() {
    const observer = new IntersectionObserver((entries) => {
        for (const entry of entries) {
            if (entry.isIntersecting) {
                entry.target.classList.add("in");
            }
        }
    }, { threshold: 0.12 });
    
    const targets = document.querySelectorAll(".v-reveal, .v-reveal-left, .v-reveal-right");
    for (const target of targets) {
        observer.observe(target);
    }
}
"#)]
extern "C" {
    fn init_reveal_js();
}

/// Reveal sections as they cross into view. Adds `in` class to any
/// element with `.v-reveal`. Honours `prefers-reduced-motion`.
#[must_use]
pub fn install_reveal() -> Option<()> {
    init_reveal_js();
    Some(())
}
