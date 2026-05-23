//! Vaelvet entry — launches the WASM app and mounts the router.

#![cfg_attr(not(test), forbid(clippy::unwrap_used, clippy::expect_used))]

use dioxus::prelude::*;
use dioxus_router::Routable;
use dioxus_router::components::Router;
use vaelvet_ui::routes::Home;
use vaelvet_ui::{Site, scroll};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let site = Site::load();

    // Install scroll-linked CSS variable + reveal observer after first render.
    use_effect(move || {
        let _ = scroll::install();
        let _ = scroll::install_reveal();
    });

    let _ = site;

    rsx! {
        document::Link { rel: "stylesheet",     href: asset!("/assets/theme.css") }
        document::Link { rel: "icon", r#type: "image/png",
                          href: asset!("/assets/images/favicon.png") }
        document::Link { rel: "apple-touch-icon",
                          href: asset!("/assets/images/favicon.png") }
        document::Link { rel: "preload", r#as: "image", fetchpriority: "high",
                          href: asset!("/assets/images/logo.jpg") }
        document::Link { rel: "preload", r#as: "image",
                          href: asset!("/assets/images/mark.jpg") }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}
