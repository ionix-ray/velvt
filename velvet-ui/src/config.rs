//! Compile-time content config. The TOML lives at `content/site.toml`.
//! `include_str!` embeds it in the wasm binary; `Site::load` parses on first use.

use serde::Deserialize;
use std::sync::OnceLock;

/// The whole site config — single source of truth for content.
#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Site {
    pub brand:     Brand,
    pub meta:      Meta,
    #[serde(default)]
    pub nav:       Vec<NavItem>,
    pub hero:      Hero,
    #[serde(default)]
    pub services:  Vec<Service>,
    #[serde(default)]
    pub cases:     Vec<CaseStudy>,
    pub manifesto: Manifesto,
    pub contact:   Contact,
    pub footer:    Footer,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Brand {
    pub name:      String,
    pub tagline:   String,
    pub short:     String,
    pub copyright: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Meta {
    pub title:       String,
    pub description: String,
    pub og_image:    String,
    pub theme_color: String,
    pub twitter:     String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct NavItem {
    pub label: String,
    pub href:  String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Hero {
    pub eyebrow:  String,
    pub headline: String,
    pub sub:      String,
    pub cta:      String,
    pub cta_href: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Service {
    pub icon:  String,
    pub title: String,
    pub body:  String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CaseStudy {
    pub slug:    String,
    pub client:  String,
    pub metric:  String,
    pub summary: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifesto {
    pub title: String,
    pub body:  String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Contact {
    pub email_general: String,
    pub email_press:   String,
    #[serde(default)]
    pub ateliers:      Vec<String>,
    pub cta:           String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Footer {
    #[serde(default)]
    pub links:   Vec<FooterLink>,
    #[serde(default)]
    pub socials: Vec<FooterLink>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct FooterLink {
    pub label: String,
    pub href:  String,
}

const RAW: &str = include_str!("../../content/site.toml");

/// Process-wide cache so we parse at most once.
static SITE: OnceLock<Site> = OnceLock::new();

impl Site {
    /// Load the embedded site config. On parse failure, logs and returns
    /// a `Site::default()` so the page still renders (degraded but live).
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
    fn site_toml_parses_and_has_required_sections() {
        let site = Site::load();
        assert_eq!(site.brand.name, "Vaelvet");
        assert!(!site.brand.tagline.is_empty());
        assert!(!site.nav.is_empty(),       "nav must not be empty");
        assert!(!site.services.is_empty(),  "services must not be empty");
        assert!(!site.cases.is_empty(),     "cases must not be empty");
        assert!(!site.hero.headline.is_empty());
    }

    #[test]
    fn site_load_is_idempotent() {
        let a = Site::load();
        let b = Site::load();
        assert!(std::ptr::eq(a, b), "OnceLock must return the same reference");
    }
}
