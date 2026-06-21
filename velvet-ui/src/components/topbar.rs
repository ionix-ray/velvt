//! Top bar — fixed brand + menu toggle + theme toggle.

use crate::theme::brand::brand_mark;
use dioxus::prelude::*;

#[component]
pub fn TopBar(menu_open: Signal<bool>, theme: Signal<String>) -> Element {
    let theme_icon = theme_icon_for(&theme.read());

    rsx! {
        header { class: "v-topbar", id: "topbar",
            div { class: "v-topbar__brand",
                img { src: brand_mark(), alt: "VELVT" }
            }
            div { class: "v-topbar__actions",
                button {
                    class: "v-theme-toggle",
                    title: "Toggle theme",
                    onclick: move |_| {
                        let next = toggle_theme(&theme.read());
                        theme.set(next);
                    },
                    "{theme_icon}"
                }
                button {
                    class: if *menu_open.read() { "v-topbar__menu-btn active" } else { "v-topbar__menu-btn" },
                    title: "Toggle navigation",
                    onclick: move |_| menu_open.toggle(),
                    span {}
                    span {}
                    span {}
                }
            }
        }
    }
}

pub(crate) fn theme_icon_for(theme: &str) -> &'static str {
    if theme == "dark" {
        "\u{2600}"
    } else {
        "\u{263E}"
    }
}

pub(crate) fn toggle_theme(current: &str) -> String {
    if current == "dark" {
        "light".to_string()
    } else {
        "dark".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_icon_is_sun_for_dark_and_moon_otherwise() {
        assert_eq!(theme_icon_for("dark"), "\u{2600}");
        assert_eq!(theme_icon_for("light"), "\u{263E}");
        assert_eq!(theme_icon_for("anything-else"), "\u{263E}");
    }

    #[test]
    fn toggle_theme_flips_dark_and_light() {
        assert_eq!(toggle_theme("dark"), "light");
        assert_eq!(toggle_theme("light"), "dark");
        assert_eq!(toggle_theme("unknown"), "dark");
    }
}
