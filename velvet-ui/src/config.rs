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
    pub name: String,
    pub tagline: String,
    pub short: String,
    pub copyright: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Meta {
    pub title: String,
    pub description: String,
    pub og_image: String,
    pub theme_color: String,
    pub twitter: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub href: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Hero {
    pub badge: String,
    pub headline1: String,
    pub headline2: String,
    pub headline3: String,
    pub sub: String,
    pub cta_primary: String,
    pub cta_primary_href: String,
    pub cta_secondary: String,
    pub cta_secondary_href: String,
    #[serde(default)]
    pub stats: Vec<HeroStat>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct HeroStat {
    pub value: String,
    pub label: String,
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
    pub num: String,
    pub title: String,
    pub body: String,
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
    pub year: String,
    pub title: String,
    pub body: String,
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
    pub value: String,
    pub label: String,
    pub change: String,
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
    pub num: String,
    pub title: String,
    pub body: String,
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
    pub source: String,
    pub title: String,
    pub body: String,
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
    pub tag: String,
    pub title: String,
    pub body: String,
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
    pub icon: String,
    pub title: String,
    pub body: String,
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
    pub icon: String,
    pub label: String,
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
    pub icon: String,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Cta {
    pub title: String,
    pub body: String,
    pub btn_primary: String,
    pub btn_secondary: String,
    pub btn_ghost: String,
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
    pub label: String,
    pub href: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FooterSocial {
    pub label: String,
    pub href: String,
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
}
