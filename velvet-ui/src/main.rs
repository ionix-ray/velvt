//! Vaelvet entry — launches the WASM app and mounts the router.

#![cfg_attr(not(test), forbid(clippy::unwrap_used, clippy::expect_used))]

use dioxus::prelude::*;
use dioxus_router::components::Router;
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
        document::Link { rel: "stylesheet",     href: asset!("/assets/theme.css") }
        document::Link { rel: "icon", r#type: "image/png",
                          href: asset!("/assets/images/favicon.png") }
        document::Link { rel: "apple-touch-icon",
                          href: asset!("/assets/images/favicon.png") }
        document::Link { rel: "preload", r#as: "image", fetchpriority: "high",
                  href: brand_mark() }
        Router::<Route> {}
    }
}
