//! Route components.

pub mod case_studies_index;
pub mod case_study;
pub mod home;

pub use case_studies_index::{CaseStudiesByTag, CaseStudiesIndex};
pub use case_study::CaseStudy;
pub use home::Home;
