use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

pub fn use_scroll_position() -> Signal<bool> {
    let scrolled = use_signal(|| false);

    use_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let mut scrolled = scrolled;
        let window_clone = window.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let scroll_y = window_clone.scroll_y().unwrap_or(0.0);
            scrolled.set(scroll_y > 50.0);
        });

        let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());

        closure.forget();
    });

    scrolled
}
