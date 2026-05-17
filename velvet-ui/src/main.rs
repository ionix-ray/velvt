mod components;
mod routes;
mod theme;

use dioxus::prelude::*;
use dioxus_router::Routable;
use dioxus_router::components::Router;
use routes::{Contact, Home, Podcast, Portfolio, Services, Talent};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta {
            "http-equiv": "Content-Security-Policy",
            content: "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' https://fonts.googleapis.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https:; connect-src 'self';",
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/theme.css"),
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Playfair+Display:wght@400;700&family=Inter:wght@400;500&family=Manrope:wght@600&display=swap",
        }
        document::Link { rel: "sitemap", r#type: "application/xml", href: "/sitemap.xml" }
        document::Title { "Velvet — Premium PR Agency" }
        document::Meta { name: "description", content: "Premium public relations agency specializing in media relations, crisis communications, talent management, and event production." }
        document::Meta { name: "theme-color", content: "#0A0A0A" }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/services")]
    Services {},
    #[route("/talent")]
    Talent {},
    #[route("/portfolio")]
    Portfolio {},
    #[route("/podcast")]
    Podcast {},
    #[route("/contact")]
    Contact {},
}
