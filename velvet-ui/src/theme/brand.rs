//! Brand mark asset — single source of truth.
//! Topbar badge, loader, footer, and the index.html preload hint all
//! resolve the same mark through these functions instead of repeating the
//! `asset!()` literal everywhere.
//!
//! The mark is the crimson "V" botanical glyph (extracted from the master
//! drawio logo, trimmed to the alpha bbox, padded square, exported as
//! lossless WebP with a PNG fallback). `loader_symbol()` points at the
//! same glyph so the iris rings overlay the current brand mark.

use dioxus::prelude::*;

/// The square brand mark (crimson "V" + botanical branch glyph).
/// Used in the topbar, footer, stacked nav, and as the card badge source.
pub fn brand_mark() -> Asset {
    asset!("/assets/images/velvet-mark.webp")
}

/// The standalone icon mark (transparent background, "V" glyph).
/// Used exclusively on the loading screen so the iris rings overlay cleanly.
pub fn loader_symbol() -> Asset {
    asset!("/assets/images/velvet-mark.webp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_mark_resolves_to_the_square_asset() {
        let resolved = format!("{}", brand_mark());
        assert!(resolved.contains("velvet-mark"), "got: {resolved}");
    }

    #[test]
    fn loader_symbol_resolves_to_the_mark_asset() {
        let resolved = format!("{}", loader_symbol());
        assert!(resolved.contains("velvet-mark"), "got: {resolved}");
    }
}
