#[allow(dead_code)]
pub const BG_PRIMARY: &str = "#0A0A0A";
#[allow(dead_code)]
pub const BG_SECONDARY: &str = "#111111";
#[allow(dead_code)]
pub const BG_ELEVATED: &str = "#1A1A1A";

#[allow(dead_code)]
pub const TEXT_PRIMARY: &str = "#F5F5F5";
#[allow(dead_code)]
pub const TEXT_SECONDARY: &str = "#A0A0A0";
#[allow(dead_code)]
pub const TEXT_MUTED: &str = "#666666";

#[allow(dead_code)]
pub const VELVET_CRIMSON: &str = "#8B0000";
#[allow(dead_code)]
pub const VELVET_CRIMSON_LIGHT: &str = "#A52A2A";
#[allow(dead_code)]
pub const DEEP_PLUM: &str = "#4B0082";
#[allow(dead_code)]
pub const DEEP_PLUM_LIGHT: &str = "#6A0DAD";
#[allow(dead_code)]
pub const WARM_GOLD: &str = "#D4AF37";

#[allow(dead_code)]
pub const FONT_HEADING: &str = "'Playfair Display', Georgia, 'Times New Roman', serif";
#[allow(dead_code)]
pub const FONT_BODY: &str = "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif";
#[allow(dead_code)]
pub const FONT_ACCENT: &str = "'Manrope', 'Inter', sans-serif";

#[allow(dead_code)]
pub const MAX_WIDTH: &str = "1280px";
#[allow(dead_code)]
pub const GUTTER: &str = "2rem";

#[allow(dead_code)]
pub const TRANSITION_FAST: &str = "150ms cubic-bezier(0.4, 0, 0.2, 1)";
#[allow(dead_code)]
pub const TRANSITION_BASE: &str = "200ms cubic-bezier(0.4, 0, 0.2, 1)";
#[allow(dead_code)]
pub const TRANSITION_SLOW: &str = "300ms cubic-bezier(0.4, 0, 0.2, 1)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_valid_hex() {
        let colors = [
            BG_PRIMARY,
            BG_SECONDARY,
            BG_ELEVATED,
            TEXT_PRIMARY,
            TEXT_SECONDARY,
            TEXT_MUTED,
            VELVET_CRIMSON,
            VELVET_CRIMSON_LIGHT,
            DEEP_PLUM,
            DEEP_PLUM_LIGHT,
            WARM_GOLD,
        ];
        for color in colors {
            assert!(color.starts_with('#'), "Color {} must start with #", color);
            assert!(color.len() == 7, "Color {} must be 7 chars (#+6hex)", color);
        }
    }

    #[test]
    fn fonts_have_fallbacks() {
        let fonts = [FONT_HEADING, FONT_BODY, FONT_ACCENT];
        for font in fonts {
            assert!(font.contains(','), "Font {} must have fallbacks", font);
        }
    }

    #[test]
    fn transitions_use_easing() {
        let transitions = [TRANSITION_FAST, TRANSITION_BASE, TRANSITION_SLOW];
        for transition in transitions {
            assert!(
                transition.contains("cubic-bezier"),
                "Transition {} must use easing",
                transition
            );
        }
    }
}
