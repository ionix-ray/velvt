const CASE_STUDY_GREENFUTURE_IMMERSIVE_STORYTELLING: &str = include_str!("../../docs/cse_studies/greenfuture-immersive-storytelling.md");
const CASE_STUDY_LUXE_BEAUTY_CELEBRITY_LAUNCH: &str = include_str!("../../docs/cse_studies/luxe-beauty-celebrity-launch.md");
const CASE_STUDY_TECHNOVA_FULL_FUNNEL_GROWTH: &str = include_str!("../../docs/cse_studies/technova-full-funnel-growth.md");

/// Load a case study's raw markdown by slug.
pub fn load_case_study(slug: &str) -> Option<&'static str> {
    match slug {
        "greenfuture-immersive-storytelling" => Some(CASE_STUDY_GREENFUTURE_IMMERSIVE_STORYTELLING),
        "luxe-beauty-celebrity-launch" => Some(CASE_STUDY_LUXE_BEAUTY_CELEBRITY_LAUNCH),
        "technova-full-funnel-growth" => Some(CASE_STUDY_TECHNOVA_FULL_FUNNEL_GROWTH),
        _ => None,
    }
}

/// All known case study slugs.
pub fn list_case_study_slugs() -> &'static [&'static str] {
    &["greenfuture-immersive-storytelling", "luxe-beauty-celebrity-launch", "technova-full-funnel-growth", ]
}
