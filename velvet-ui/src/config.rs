//! Compile-time content config. The TOML lives at `content/site.toml`.
//! `include_str!` embeds it in the wasm binary; `Site::load` parses on first use.

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
    pub steps: Vec<ProcessStep>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ProcessStep {
    pub num: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Cases {
    pub title: String,
    pub sub: String,
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
    pub bg_image: String,
    #[serde(default)]
    pub logo_image: String,
    #[serde(default)]
    pub button_link: String,
    #[serde(default)]
    pub footer_label: String,
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
    pub items: Vec<StudioItem>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct StudioItem {
    pub tag: Box<str>,
    pub title: Box<str>,
    pub body: Box<str>,
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

const RAW: &str = include_str!("../../content/site.toml");
static SITE: OnceLock<Site> = OnceLock::new();

impl Site {
    #[must_use]
    pub fn load() -> &'static Self {
        SITE.get_or_init(|| match toml::from_str::<Self>(RAW) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(error = %e, "site.toml parse failed; falling back to default");
                Self::default()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_toml_parses() {
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
    fn site_toml_all_required_fields_non_empty() {
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
    fn site_toml_nav_has_entries() {
        let site = Site::load();
        assert!(!site.nav.is_empty(), "nav must have at least one item");
        for item in &site.nav {
            assert!(!item.label.is_empty(), "nav item label is empty");
            assert!(!item.href.is_empty(), "nav item href is empty");
        }
    }

    #[test]
    fn site_toml_footer_columns_non_empty() {
        let site = Site::load();
        assert!(!site.footer.columns.is_empty(), "footer must have columns");
        for col in &site.footer.columns {
            assert!(!col.title.is_empty(), "footer column title is empty");
        }
    }
}
