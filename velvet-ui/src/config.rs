//! Compile-time content config. Editable content lives at `content/site.md`:
//! a markdown file with one `## Section` heading per `Site` field, each
//! followed by a fenced ```toml``` block holding that field's data.
//! `include_str!` embeds it in the wasm binary; `Site::load` strips the
//! markdown prose, concatenates the fenced TOML blocks in order, and parses
//! the result with `toml::from_str` on first use.

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Site {
    pub brand: Brand,
    pub meta: Meta,
    #[serde(default)]
    pub nav: Vec<NavItem>,
    pub hero: Hero,
    #[serde(default)]
    pub services: Services,
    #[serde(default)]
    pub story: Story,
    #[serde(default)]
    pub analytics: Analytics,
    #[serde(default)]
    pub process: Process,
    #[serde(default)]
    pub cases: Cases,
    #[serde(default)]
    pub pr: PrSection,
    #[serde(default)]
    pub studio: Studio,
    #[serde(default)]
    pub ai: AiSection,
    #[serde(default)]
    pub social: Social,
    #[serde(default)]
    pub awards: Awards,
    #[serde(default)]
    pub cta: Cta,
    #[serde(default)]
    pub contact: Contact,
    pub footer: Footer,
    #[serde(default)]
    pub client_banner: ClientBanner,
    #[serde(default)]
    pub team: Vec<TeamMember>,
    #[serde(default)]
    pub background: Background,
}

fn default_true() -> bool {
    true
}

fn default_intensity() -> u8 {
    80
}

/// Whole-site procedural dynamic background (no video decode cost).
/// Layered brand spotlight gradients drift on the compositor thread;
/// `intensity` scales layer opacity 0..=100.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Background {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_intensity")]
    pub intensity: u8,
    /// Slow crimson spotlight meshes + stage beam (the gradient wash).
    #[serde(default = "default_true")]
    pub mesh: bool,
    /// Drifting service-icon glyphs — the "creative" half of the blend.
    #[serde(default = "default_true")]
    pub show_glyphs: bool,
    /// Glyph layer opacity as a percent of `intensity` (0..=100).
    #[serde(default = "default_glyph_opacity")]
    pub glyph_opacity: u8,
    /// Per-glyph drift cycle length in seconds. Larger = calmer.
    #[serde(default = "default_glyph_duration")]
    pub glyph_duration: u32,
    /// The floating service glyphs themselves. Empty list = mesh only.
    #[serde(default)]
    pub glyphs: Vec<BackgroundGlyph>,
}

/// One floating service glyph in the ambient field.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct BackgroundGlyph {
    /// Icon name resolved by `Icon { name }` — see `components::icons`.
    pub icon: Box<str>,
    /// Palette slot: "crimson" | "violet" | "sky" | "citron" (bluish lime).
    #[serde(default)]
    pub tone: Box<str>,
    /// Horizontal placement as a percentage of viewport width (0..=100).
    #[serde(default)]
    pub x: u8,
    /// Vertical placement as a percentage of viewport height (0..=100).
    #[serde(default)]
    pub y: u8,
    /// Rendered size in px before per-glyph scaling.
    #[serde(default = "default_glyph_size")]
    pub size: u32,
    /// Multiplier on the base drift cycle; keeps neighbours out of lockstep.
    #[serde(default = "default_one_f32")]
    pub rate: f32,
    /// Negative start offset (seconds) so the field is not synchronised.
    #[serde(default)]
    pub delay: f32,
    /// Extra rotation amplitude in degrees.
    #[serde(default = "default_rot")]
    pub spin: f32,
}

fn default_glyph_opacity() -> u8 {
    30
}
fn default_glyph_duration() -> u32 {
    26
}
fn default_glyph_size() -> u32 {
    132
}
fn default_one_f32() -> f32 {
    1.0
}
fn default_rot() -> f32 {
    6.0
}

/// Opacity scalar 0.0..=1.0 for a 0..=100 intensity percent. Clamped so a
/// typo in `site.md` can never push a layer opaque or negative.
pub fn background_alpha(intensity: u8) -> f32 {
    (intensity.min(100) as f32 / 100.0).clamp(0.0, 1.0)
}

/// Glyph opacity: `intensity` sets the ceiling, `glyph_opacity` the share of
/// it. Clamped to 0.0..=1.0; the glyph layer is atmosphere, never content.
pub fn glyph_alpha(intensity: u8, glyph_opacity: u8) -> f32 {
    (background_alpha(intensity) * background_alpha(glyph_opacity)).clamp(0.0, 1.0)
}

/// Percentage clamp for glyph placement — keeps glyphs on screen even if
/// `site.md` carries a bad number.
pub fn glyph_percent(value: u8) -> f32 {
    (value.min(100) as f32).clamp(0.0, 100.0)
}

/// Drift cycle in seconds, floored at 4s so a fat-fingered `rate` can never
/// produce a strobing field.
pub fn glyph_duration(base: u32, rate: f32) -> f32 {
    let rate = if rate.is_finite() && rate > 0.0 {
        rate.clamp(0.25, 4.0)
    } else {
        1.0
    };
    (base.max(4) as f32 * rate).clamp(4.0, 240.0)
}

/// Rendered size in px, clamped so a glyph can never dominate the viewport.
pub fn glyph_size(size: u32) -> f32 {
    (size.clamp(40, 420)) as f32
}

/// Spin amplitude in degrees, clamped to keep the field calm.
pub fn glyph_spin(spin: f32) -> f32 {
    if spin.is_finite() {
        spin.clamp(-45.0, 45.0)
    } else {
        default_rot()
    }
}

/// Animation delay in seconds; non-finite input collapses to 0.
pub fn glyph_delay(delay: f32) -> f32 {
    if delay.is_finite() {
        delay.clamp(-240.0, 0.0)
    } else {
        0.0
    }
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TeamMember {
    #[serde(default)]
    pub name: Box<str>,
    #[serde(default)]
    pub eyebrow: Box<str>,
    #[serde(default)]
    pub bio: Box<str>,
    /// Optional path under `velvet-ui/assets/` to the team member portrait.
    /// Leave empty (or omit) to render the monogram placeholder tile.
    #[serde(default)]
    pub photo: Box<str>,
    /// Single-character fallback shown inside the placeholder tile when
    /// `photo` is empty (or 404s).
    #[serde(default)]
    pub monogram: Box<str>,
    #[serde(default)]
    pub handles: Vec<SocialHandle>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SocialHandle {
    /// Icon name resolved by `Icon { name }` — e.g. "instagram", "linkedin".
    pub icon: Box<str>,
    /// Display text rendered next to the icon — e.g. "@thearpitaparhi_official".
    pub label: Box<str>,
    pub href: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ClientBanner {
    pub title: String,
    #[serde(default)]
    pub logos: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Brand {
    pub name: Box<str>,
    pub tagline: Box<str>,
    pub short: Box<str>,
    pub copyright: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Meta {
    pub title: Box<str>,
    pub description: Box<str>,
    pub og_image: Box<str>,
    pub theme_color: Box<str>,
    pub twitter: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct NavItem {
    pub label: Box<str>,
    pub href: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Hero {
    #[serde(default)]
    pub logo: Box<str>,
    #[serde(default)]
    pub video: Option<Box<str>>,
    /// Transparent animated SVG wordmark (vendored from the master Draw.io
    /// logo). Rendered as the hero visual so the page background shows through.
    #[serde(default)]
    pub logo_svg: Option<Box<str>>,
    pub badge: Box<str>,
    pub headline1: Box<str>,
    pub headline2: Box<str>,
    pub headline3: Box<str>,
    pub sub: Box<str>,
    pub cta_primary: Box<str>,
    pub cta_primary_href: Box<str>,
    pub cta_secondary: Box<str>,
    pub cta_secondary_href: Box<str>,
    #[serde(default)]
    pub stats: Vec<HeroStat>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct HeroStat {
    pub value: Box<str>,
    pub label: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Services {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub items: Vec<ServiceItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ServiceItem {
    pub num: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Story {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub items: Vec<StoryItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct StoryItem {
    pub year: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Analytics {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub stats: Vec<AnalyticStat>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AnalyticStat {
    pub value: Box<str>,
    pub label: Box<str>,
    pub change: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Process {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub eyebrow: String,
    #[serde(default)]
    pub steps: Vec<ProcessStep>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ProcessStep {
    pub num: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
    #[serde(default)]
    pub image: Option<Box<str>>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Cases {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub eyebrow: String,
    #[serde(default)]
    pub view_case_study: String,
    #[serde(default)]
    pub items: Vec<CaseItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CaseItem {
    pub client: String,
    pub metric: String,
    pub desc: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub logo_image: String,
    #[serde(default)]
    pub button_link: String,
    #[serde(default)]
    pub footer_label: String,
    /// Slug of a markdown file under `docs/cse_studies/` — when set, "View
    /// Case Study" links to `/achivements/{slug}` instead of `button_link`.
    #[serde(default)]
    pub slug: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PrSection {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub items: Vec<PrItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PrItem {
    pub source: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Studio {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub eyebrow: String,
    #[serde(default)]
    pub items: Vec<StudioItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct StudioItem {
    pub tag: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
    #[serde(default)]
    pub image: Option<Box<str>>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AiSection {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub features: Vec<AiFeature>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AiFeature {
    pub icon: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Social {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub items: Vec<SocialItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SocialItem {
    pub icon: Box<str>,
    pub label: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Awards {
    pub title: String,
    pub sub: String,
    #[serde(default)]
    pub items: Vec<AwardItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AwardItem {
    pub icon: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Cta {
    pub title: Box<str>,
    pub body: Box<str>,
    pub btn_primary: Box<str>,
    pub btn_secondary: Box<str>,
    pub btn_ghost: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Contact {
    pub email_general: String,
    pub email_press: String,
    #[serde(default)]
    pub ateliers: Vec<String>,
    pub cta: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Footer {
    pub brand_desc: String,
    #[serde(default)]
    pub columns: Vec<FooterColumn>,
    #[serde(default)]
    pub socials: Vec<FooterSocial>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FooterColumn {
    pub title: String,
    #[serde(default)]
    pub links: Vec<FooterLink>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FooterLink {
    pub label: Box<str>,
    pub href: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FooterSocial {
    pub label: Box<str>,
    pub href: Box<str>,
}

const RAW: &str = include_str!("../../content/site.md");
static SITE: OnceLock<Site> = OnceLock::new();

/// Extracts and concatenates every fenced ` ```toml ` code block from a
/// markdown document, in document order, separated by newlines.
fn extract_toml_from_markdown(md: &str) -> String {
    let mut out = String::new();
    let mut lines = md.lines();
    while let Some(line) = lines.by_ref().next() {
        if line.trim_start() != "```toml" {
            continue;
        }
        for body_line in lines.by_ref() {
            if body_line.trim_start() == "```" {
                break;
            }
            out.push_str(body_line);
            out.push('\n');
        }
    }
    out
}

/// Parse extracted TOML into a [`Site`], falling back to [`Site::default`]
/// (with a logged error) if the content is malformed.
fn parse_or_default(toml_src: &str) -> Site {
    match toml::from_str::<Site>(toml_src) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(error = %e, "site.md parse failed; falling back to default");
            Site::default()
        }
    }
}

impl Site {
    #[must_use]
    pub fn load() -> &'static Self {
        SITE.get_or_init(|| parse_or_default(&extract_toml_from_markdown(RAW)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_toml_from_markdown_concatenates_fenced_blocks_in_order() {
        let md = "# Title\nprose\n```toml\n[a]\nx = 1\n```\nmore prose\n```toml\n[b]\ny = 2\n```\n";
        let extracted = extract_toml_from_markdown(md);
        assert_eq!(extracted, "[a]\nx = 1\n[b]\ny = 2\n");
    }

    #[test]
    fn extract_toml_from_markdown_ignores_non_toml_fences_and_prose() {
        let md = "intro\n```rust\nfn f() {}\n```\n```toml\n[a]\nx = 1\n```\ntrailing prose\n";
        let extracted = extract_toml_from_markdown(md);
        assert_eq!(extracted, "[a]\nx = 1\n");
    }

    #[test]
    fn extract_toml_from_markdown_empty_input_yields_empty_string() {
        assert_eq!(extract_toml_from_markdown(""), "");
    }

    #[test]
    fn site_md_parses() {
        let site = Site::load();
        assert!(!site.brand.name.is_empty());
        assert!(!site.hero.headline1.is_empty());
        assert!(!site.services.items.is_empty());
        assert!(!site.story.items.is_empty());
        assert!(!site.cases.items.is_empty());
    }

    #[test]
    fn site_load_is_idempotent() {
        let a = Site::load();
        let b = Site::load();
        assert_eq!(a as *const _, b as *const _);
    }

    #[test]
    fn site_md_all_required_fields_non_empty() {
        let site = Site::load();
        assert!(!site.brand.tagline.is_empty(), "brand.tagline is empty");
        assert!(!site.brand.copyright.is_empty(), "brand.copyright is empty");
        assert!(!site.meta.title.is_empty(), "meta.title is empty");
        assert!(
            !site.meta.description.is_empty(),
            "meta.description is empty"
        );
        assert!(
            !site.hero.cta_primary.is_empty(),
            "hero.cta_primary is empty"
        );
        assert!(!site.cta.title.is_empty(), "cta.title is empty");
    }

    #[test]
    fn site_md_nav_has_entries() {
        let site = Site::load();
        assert!(!site.nav.is_empty(), "nav must have at least one item");
        for item in &site.nav {
            assert!(!item.label.is_empty(), "nav item label is empty");
            assert!(!item.href.is_empty(), "nav item href is empty");
        }
    }

    #[test]
    fn parse_or_default_falls_back_on_malformed_toml() {
        let site = parse_or_default("this is not valid toml {{{");
        assert_eq!(site, Site::default());
    }

    #[test]
    fn site_md_footer_columns_non_empty() {
        let site = Site::load();
        assert!(!site.footer.columns.is_empty(), "footer must have columns");
        for col in &site.footer.columns {
            assert!(!col.title.is_empty(), "footer column title is empty");
        }
    }

    #[test]
    #[allow(clippy::collapsible_if, clippy::panic)]
    fn test_no_stray_toml_headers() {
        let mut in_toml_block = false;
        for (i, line) in RAW.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("```toml") {
                in_toml_block = true;
                continue;
            } else if in_toml_block && trimmed.starts_with("```") {
                in_toml_block = false;
                continue;
            }

            if !in_toml_block {
                if (trimmed.starts_with("[[") && trimmed.ends_with("]]"))
                    || (trimmed.starts_with("[")
                        && trimmed.ends_with("]")
                        && !trimmed.contains("](")
                        && !trimmed.contains(' '))
                {
                    panic!(
                        "Found stray TOML header outside of ```toml block on line {}: {}",
                        i + 1,
                        trimmed
                    );
                }
            }
        }
    }
}
