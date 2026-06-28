//! Brand mark asset — single source of truth.
//! Topbar badge, loader, and the index.html preload hint all resolve the
//! same square mark through this one function instead of repeating the
//! `asset!()` literal three times.

use dioxus::prelude::*;

/// The square brand mark (crimson card, "Velvt" wordmark + leaf glyph).
pub fn brand_mark() -> Asset {
    asset!("/assets/images/new-logo-1.png")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_mark_resolves_to_the_square_asset() {
        let resolved = format!("{}", brand_mark());
        assert!(resolved.contains("new-logo-1"), "got: {resolved}");
    }
}
