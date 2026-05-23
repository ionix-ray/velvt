//! Design tokens — extracted from the Vaelvet logo (velvet red + gold lettering).
//! Single source of truth, mirrored in `assets/theme.css` as CSS custom properties.

#![allow(dead_code)]

/// Palette — five swatches lifted from the logo.
pub mod palette {
    /// Deep velvet shadow (background base, footer wash).
    pub const VELVET_DEEP: &str = "#5A0A0F";
    /// Mid-tone velvet crimson (hero ambient, dividers).
    pub const VELVET_CRIMSON: &str = "#8B1A1F";
    /// Warm ember highlight (cta hover, rim-light).
    pub const VELVET_EMBER: &str = "#B83C2B";

    /// Warm gold lettering (primary headings, brand mark).
    pub const GOLD_LEAF: &str = "#E8C892";
    /// Bright cream (high-contrast highlights).
    pub const GOLD_BRIGHT: &str = "#F5D9A0";

    /// Stage ink — near-black canvas.
    pub const INK: &str = "#0A0405";
    /// Soft ivory body text.
    pub const IVORY: &str = "#F8EFE0";
    /// Smoke — mid-grey for subdued type.
    pub const SMOKE: &str = "#9A8E84";
}

/// Type scale — Plex Serif Display for headings, Plex Sans for body.
pub mod fonts {
    pub const DISPLAY: &str =
        "\"IBM Plex Serif\", \"Plex Serif Display\", Georgia, \"Times New Roman\", serif";
    pub const SANS: &str = "\"IBM Plex Sans\", system-ui, -apple-system, sans-serif";
    pub const MONO: &str = "\"IBM Plex Mono\", ui-monospace, monospace";
}

/// Spacing — IBM Carbon 2× base (8px), exponential.
pub mod space {
    pub const S0: &str = "0";
    pub const S1: &str = "0.25rem"; // 4
    pub const S2: &str = "0.5rem"; // 8
    pub const S3: &str = "1rem"; // 16
    pub const S4: &str = "1.5rem"; // 24
    pub const S5: &str = "2.5rem"; // 40
    pub const S6: &str = "4rem"; // 64
    pub const S7: &str = "6rem"; // 96
    pub const S8: &str = "9rem"; // 144
}

/// Motion — easing curves and durations.
pub mod motion {
    pub const EASE_STAGE: &str = "cubic-bezier(0.16, 1, 0.3, 1)";
    pub const EASE_LINEAR: &str = "linear";
    pub const D_FAST: &str = "180ms";
    pub const D_MED: &str = "420ms";
    pub const D_SLOW: &str = "1200ms";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_hex_values_are_well_formed() {
        for c in [
            palette::VELVET_DEEP,
            palette::VELVET_CRIMSON,
            palette::VELVET_EMBER,
            palette::GOLD_LEAF,
            palette::GOLD_BRIGHT,
            palette::INK,
            palette::IVORY,
            palette::SMOKE,
        ] {
            assert_eq!(c.len(), 7, "hex must be #RRGGBB: {c}");
            assert!(c.starts_with('#'), "hex must start with #: {c}");
        }
    }
}
