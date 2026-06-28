//! Brand mark asset — single source of truth.
//! Topbar badge, footer, and the index.html preload hint all resolve the
//! same square mark through this one function instead of repeating the
//! `asset!()` literal three times.
//!
//! `loader_symbol()` is a separate, stripped logo mark used exclusively
//! during the loading screen so the iris ring animation overlays cleanly
//! on a transparent-background icon rather than the full wordmark card.

use dioxus::prelude::*;

/// The square brand mark (crimson card, "Velvt" wordmark + leaf glyph).
/// Used in the topbar, footer, and everywhere a full logo is needed.
pub fn brand_mark() -> Asset {
    asset!("/assets/images/velvet-square.png")
}

/// The standalone icon mark (transparent background, leaf glyph only).
/// Used exclusively on the loading screen so the iris rings overlay cleanly.
pub fn loader_symbol() -> Asset {
    asset!("/assets/images/only-logo.png")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_mark_resolves_to_the_square_asset() {
        let resolved = format!("{}", brand_mark());
        assert!(resolved.contains("velvet-square"), "got: {resolved}");
    }

    #[test]
    fn loader_symbol_resolves_to_only_logo_asset() {
        let resolved = format!("{}", loader_symbol());
        assert!(resolved.contains("only-logo"), "got: {resolved}");
    }
}
