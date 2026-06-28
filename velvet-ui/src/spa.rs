//! Document-level SPA click interceptor.
//!
//! Any plain `<a href="…" data-spa="true">` link gets its click intercepted
//! at the document level: we cancel the default browser navigation,
//! `history.pushState` the new URL, and dispatch a synthetic `popstate`
//! event so the dioxus-router history adapter picks the route up. That
//! keeps the SSR-rendered HTML test-friendly (plain `<a href>`) while the
//! running browser app navigates without a full page reload.
//!
//! All handles are optional; if any step fails the click falls through to
//! the browser's default behaviour, so the worst-case is a full reload —
//! never a panic, never a crash.

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, Event, HtmlAnchorElement};

/// Install a document-level click listener that intercepts SPA links and
/// routes them via `history.pushState` + a synthetic `popstate` event.
/// Returns `Some(())` on success.
#[must_use]
pub fn install_spa_link_interceptor() -> Option<()> {
    let win = web_sys::window()?;
    let doc = win.document()?;
    let history = win.history().ok()?;

    let cb = Closure::<dyn FnMut(Event)>::new(move |evt: Event| {
        // Drop modified-key clicks (cmd/ctrl/middle-click open-in-new-tab) so
        // the browser's native "open in new tab" behaviour is preserved.
        if let Ok(mouse) = evt.clone().dyn_into::<web_sys::MouseEvent>() {
            if mouse.meta_key() || mouse.ctrl_key() || mouse.shift_key() || mouse.button() != 0 {
                return;
            }
        }

        let Some(target) = evt.target() else { return };
        let Ok(node) = target.dyn_into::<Element>() else {
            return;
        };

        // Walk up to the nearest <a data-spa="true">.
        let Ok(Some(anchor_el)) = node.closest("a[data-spa='true']") else {
            return;
        };
        let Ok(anchor) = anchor_el.dyn_into::<HtmlAnchorElement>() else {
            return;
        };

        let href = anchor.get_attribute("href").unwrap_or_default();
        if !is_internal_path(&href) {
            return;
        }

        evt.prevent_default();
        if history
            .push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&href))
            .is_err()
        {
            return;
        }
        // Tell the router to re-read window.location.
        if let Ok(popstate) = Event::new("popstate") {
            let _ = web_sys::window()
                .map(|w| w.dispatch_event(&popstate))
                .transpose();
        }
        // Always scroll to the very top on every SPA route change.
        // Use both instant-scroll and a deferred scroll to win any race with
        // Dioxus re-rendering, which can briefly restore a previous scroll
        // position before the new route content paints.
        if let Some(win) = web_sys::window() {
            let opts = web_sys::ScrollToOptions::new();
            opts.set_top(0.0);
            opts.set_left(0.0);
            opts.set_behavior(web_sys::ScrollBehavior::Instant);
            win.scroll_to_with_scroll_to_options(&opts);

            // Defer a second scroll to override any post-render restoration.
            if let Some(win2) = web_sys::window() {
                let cb = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                    let opts2 = web_sys::ScrollToOptions::new();
                    opts2.set_top(0.0);
                    opts2.set_left(0.0);
                    opts2.set_behavior(web_sys::ScrollBehavior::Instant);
                    win2.scroll_to_with_scroll_to_options(&opts2);
                });
                let _ = win
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        50,
                    );
                cb.forget();
            }
        }
    });

    let listener = cb.as_ref().unchecked_ref();
    if doc
        .add_event_listener_with_callback("click", listener)
        .is_err()
    {
        return None;
    }
    cb.forget(); // listener lives for the page lifetime
    Some(())
}

/// Same-origin internal path: starts with `/`, no scheme, no host. Rules out
/// `http://`, `https://`, `mailto:`, `tel:`, `#anchor`, and protocol-relative
/// `//other-host/`. Keeps the interceptor scoped to in-app navigation.
fn is_internal_path(href: &str) -> bool {
    if href.is_empty() {
        return false;
    }
    if !href.starts_with('/') {
        return false;
    }
    if href.starts_with("//") {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_paths_match_the_in_app_subset() {
        assert!(is_internal_path("/"));
        assert!(is_internal_path("/achivements"));
        assert!(is_internal_path("/achivements/technova-full-funnel-growth"));
        assert!(is_internal_path("/achivements/tag/B2B"));
    }

    #[test]
    fn external_or_hash_or_scheme_links_are_left_alone() {
        assert!(!is_internal_path(""));
        assert!(!is_internal_path("#contact"));
        assert!(!is_internal_path("mailto:hello@velvt.live"));
        assert!(!is_internal_path("tel:+919348404970"));
        assert!(!is_internal_path("https://velvt.live"));
        assert!(!is_internal_path("http://x.com"));
        // Protocol-relative URL points off-origin in browsers.
        assert!(!is_internal_path("//external.example/path"));
    }
}
