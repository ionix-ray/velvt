//! Vaelvet entry — launches the WASM app and mounts the router.

#![cfg_attr(not(test), forbid(clippy::unwrap_used, clippy::expect_used))]

use dioxus::prelude::*;
use dioxus_router::components::Router;
use vaelvet_ui::components::dynamic_background::DynamicBackground;
use vaelvet_ui::routes::Route;
use vaelvet_ui::scroll;
use vaelvet_ui::spa;
use vaelvet_ui::theme::brand::brand_mark;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Install reveal observer + SPA link interceptor after first render.
    use_effect(move || {
        let _ = scroll::install_reveal();
        let _ = spa::install_spa_link_interceptor();
    });

    rsx! {
        document::Link { rel: "preload", r#as: "image", fetchpriority: "high",
                  href: brand_mark() }
        // Whole-site dynamic background — one mount, every route.
        DynamicBackground { site: vaelvet_ui::Site::load().clone() }
        Router::<Route> {}
    }
}
