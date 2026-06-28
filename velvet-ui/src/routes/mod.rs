//! Route components + the typed `Route` enum that backs the SPA router.
//! Defined in the library crate (not `main.rs`) so any component can build
//! typed `Link { to: Route::… }` targets without forcing a full-page
//! reload via plain `<a href="…">`.

use dioxus::prelude::*;
use dioxus_router::Routable;

pub mod case_studies_index;
pub mod case_study;
pub mod home;

pub use case_studies_index::{CaseStudiesByTag, CaseStudiesIndex};
pub use case_study::CaseStudy;
pub use home::Home;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/achivements")]
    CaseStudiesIndex {},
    #[route("/achivements/tag/:tag")]
    CaseStudiesByTag { tag: String },
    #[route("/achivements/:slug")]
    CaseStudy { slug: String },
}
