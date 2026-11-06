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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_enum_variants_are_constructible() {
        let _home = Route::Home {};
        let _index = Route::CaseStudiesIndex {};
        let _tag = Route::CaseStudiesByTag {
            tag: "B2B".to_string(),
        };
        let _study = Route::CaseStudy {
            slug: "test-slug".to_string(),
        };
    }

    #[test]
    fn route_enum_derives_work() {
        let r1 = Route::Home {};
        let r2 = Route::Home {};
        assert_eq!(r1, r2);
        let _c = r1.clone();
        let _d = format!("{r2:?}");
    }
}
