//! Home — the only route. Composes the whole cinematic page.

use crate::Site;
use crate::components::{
    case_studies::CaseStudies, contact::Contact, footer::Footer, hero::Hero,
    manifesto::Manifesto, nav::Nav, services::Services,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let site = Site::load().clone();
    rsx! {
        Nav         { site: site.clone() }
        main {
            Hero        { site: site.clone() }
            Manifesto   { site: site.clone() }
            Services    { site: site.clone() }
            CaseStudies { site: site.clone() }
            Contact     { site: site.clone() }
        }
        Footer      { site: site }
    }
}
