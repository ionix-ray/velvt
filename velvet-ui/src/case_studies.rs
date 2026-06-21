//! Case study content — TOML frontmatter + markdown body, embedded at
//! compile time from `docs/cse_studies/*.md` via `build.rs`. Adding a case
//! study is a content-only change: drop a new markdown file in that
//! directory and rebuild — no Rust code change required.

use serde::Deserialize;

include!("generated_case_studies.rs");

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CaseStudyFrontmatter {
    pub title: String,
    pub client: String,
    pub metric: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub summary: String,
}

/// Split TOML frontmatter (between the first two `---` delimiters) from the
/// markdown body that follows. A `splitn(3, ...)` keeps any further `---`
/// horizontal rules in the body intact rather than splitting on them too.
pub fn parse_case_study(markdown: &str) -> Option<(CaseStudyFrontmatter, String)> {
    let mut parts = markdown.splitn(3, "---");
    parts.next()?;
    let frontmatter_str = parts.next()?;
    let body = parts.next()?.trim_start().to_string();
    let frontmatter: CaseStudyFrontmatter = toml::from_str(frontmatter_str).ok()?;
    Some((frontmatter, body))
}

/// Look up one case study by slug.
pub fn get_case_study(slug: &str) -> Option<(CaseStudyFrontmatter, String)> {
    let markdown = load_case_study(slug)?;
    parse_case_study(markdown)
}

/// All case studies, newest first.
pub fn get_all_case_studies() -> Vec<(String, CaseStudyFrontmatter, String)> {
    let mut studies: Vec<(String, CaseStudyFrontmatter, String)> = list_case_study_slugs()
        .iter()
        .filter_map(|slug| {
            let (frontmatter, body) = get_case_study(slug)?;
            Some(((*slug).to_string(), frontmatter, body))
        })
        .collect();
    studies.sort_by(|a, b| b.1.date.cmp(&a.1.date));
    studies
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const SAMPLE: &str = "---\ntitle = \"T\"\nclient = \"C\"\nmetric = \"+1%\"\ndate = \"2026-01-01\"\ntags = [\"A\", \"B\"]\nsummary = \"S\"\n---\n# Body\n\nText with a --- in it.\n";

    #[test]
    fn parse_case_study_extracts_frontmatter_and_body() -> TestResult {
        let (frontmatter, body) = parse_case_study(SAMPLE).ok_or("sample markdown should parse")?;
        assert_eq!(frontmatter.title, "T");
        assert_eq!(frontmatter.client, "C");
        assert_eq!(frontmatter.metric, "+1%");
        assert_eq!(frontmatter.date, "2026-01-01");
        assert_eq!(frontmatter.tags, vec!["A".to_string(), "B".to_string()]);
        assert_eq!(frontmatter.summary, "S");
        assert!(body.starts_with("# Body"));
        assert!(body.contains("Text with a --- in it."));
        Ok(())
    }

    #[test]
    fn parse_case_study_rejects_markdown_without_frontmatter_fences() {
        assert_eq!(parse_case_study("# Just a heading, no frontmatter"), None);
    }

    #[test]
    fn parse_case_study_rejects_invalid_toml_frontmatter() {
        let bad = "---\nthis is not valid toml :::\n---\nbody\n";
        assert_eq!(parse_case_study(bad), None);
    }

    #[test]
    fn get_case_study_returns_none_for_unknown_slug() {
        assert_eq!(get_case_study("does-not-exist"), None);
    }

    #[test]
    fn get_case_study_loads_a_real_sample_file() -> TestResult {
        let (frontmatter, body) = get_case_study("technova-full-funnel-growth")
            .ok_or("technova-full-funnel-growth.md should be embedded by build.rs")?;
        assert_eq!(frontmatter.client, "TechNova");
        assert_eq!(frontmatter.metric, "+240%");
        assert!(frontmatter.tags.contains(&"B2B".to_string()));
        assert!(body.contains("## The brief"));
        Ok(())
    }

    #[test]
    fn get_all_case_studies_includes_every_sample_sorted_newest_first() {
        let studies = get_all_case_studies();
        let slugs: Vec<&str> = studies.iter().map(|(slug, _, _)| slug.as_str()).collect();
        assert!(slugs.contains(&"technova-full-funnel-growth"));
        assert!(slugs.contains(&"luxe-beauty-celebrity-launch"));
        assert!(slugs.contains(&"greenfuture-immersive-storytelling"));

        let dates: Vec<&str> = studies.iter().map(|(_, fm, _)| fm.date.as_str()).collect();
        let mut sorted_desc = dates.clone();
        sorted_desc.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(dates, sorted_desc);
    }
}
