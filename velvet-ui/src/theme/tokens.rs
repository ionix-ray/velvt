//! Design tokens — Vaelvet PR Agency.
//! Dark red + black + off-white palette. IBM Plex Sans body, display headings.
//! Single source of truth, mirrored in `assets/theme.css` as CSS custom properties.

#![allow(dead_code)]

/// Palette — dark red + black + off-white.
pub mod palette {
    pub const CRIMSON_DEEP: &str = "#5A0A0F";
    pub const CRIMSON: &str = "#8B1A1F";
    pub const CRIMSON_LIGHT: &str = "#B83C2B";
    pub const BLACK: &str = "#0A0405";
    pub const OFF_WHITE: &str = "#F8EFE0";
    pub const SMOKE: &str = "#9A8E84";
    pub const WARM_GRAY: &str = "#2A2226";
    pub const BORDER_LIGHT: &str = "rgba(10, 4, 5, 0.12)";
    pub const BORDER_DARK: &str = "rgba(248, 239, 224, 0.12)";
}

/// Type scale — display headings + IBM Plex Sans body.
pub mod fonts {
    pub const DISPLAY: &str = "\"Outfit\", system-ui, -apple-system, sans-serif";
    pub const SANS: &str = "\"IBM Plex Sans\", system-ui, -apple-system, sans-serif";
    pub const MONO: &str = "\"IBM Plex Sans\", system-ui, -apple-system, sans-serif";
}

/// Spacing — 8px base, exponential.
pub mod space {
    pub const S0: &str = "0";
    pub const S1: &str = "0.25rem";
    pub const S2: &str = "0.5rem";
    pub const S3: &str = "1rem";
    pub const S4: &str = "1.5rem";
    pub const S5: &str = "2.5rem";
    pub const S6: &str = "4rem";
    pub const S7: &str = "6rem";
    pub const S8: &str = "9rem";
}

/// Motion — subtle, precise.
pub mod motion {
    pub const EASE: &str = "cubic-bezier(0.25, 0.46, 0.45, 0.94)";
    pub const EASE_OUT: &str = "cubic-bezier(0.16, 1, 0.3, 1)";
    pub const D_FAST: &str = "180ms";
    pub const D_MED: &str = "320ms";
    pub const D_SLOW: &str = "600ms";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_hex_values_are_well_formed() {
        for c in [
            palette::CRIMSON_DEEP,
            palette::CRIMSON,
            palette::CRIMSON_LIGHT,
            palette::BLACK,
            palette::OFF_WHITE,
        ] {
            assert_eq!(c.len(), 7, "hex must be #RRGGBB: {c}");
            assert!(c.starts_with('#'), "hex must start with #: {c}");
        }
    }

    #[test]
    fn spacing_units_are_rem() {
        for s in [
            space::S1,
            space::S2,
            space::S3,
            space::S4,
            space::S5,
            space::S6,
            space::S7,
            space::S8,
        ] {
            assert!(s.ends_with("rem"), "spacing must use rem units: {s}");
        }
        assert_eq!(space::S0, "0");
    }

    #[test]
    fn motion_easings_are_cubic_bezier() {
        for m in [motion::EASE, motion::EASE_OUT] {
            assert!(
                m.starts_with("cubic-bezier("),
                "motion easing must be cubic-bezier: {m}"
            );
            assert!(m.ends_with(')'), "motion easing must end with ): {m}");
        }
    }
}
