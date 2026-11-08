//! Top bar — fixed brand + menu toggle + theme toggle.

use crate::theme::wordmark::Wordmark;
use dioxus::prelude::*;

#[component]
pub fn TopBar(menu_open: Signal<bool>, theme: Signal<String>) -> Element {
    let theme_icon = theme_icon_for(&theme.read());

    rsx! {
        header { class: "v-topbar", id: "topbar",
            a {
                class: "v-topbar__brand",
                href: "/",
                "data-spa": "true",
                aria_label: "Velvt — home",
                div { class: "v-logo-wrap",
                    Wordmark { class: "v-logo-svg".to_string(), title: None }
                    div { class: "v-logo-sheen", "aria-hidden": "true" }
                }
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
    use dioxus::prelude::VirtualDom;

    #[component]
    fn WrapTopBarClosed() -> Element {
        let menu_open = use_signal(|| false);
        let theme = use_signal(|| "light".to_string());
        rsx! { TopBar { menu_open, theme } }
    }

    #[component]
    fn WrapTopBarOpen() -> Element {
        let menu_open = use_signal(|| true);
        let theme = use_signal(|| "dark".to_string());
        rsx! { TopBar { menu_open, theme } }
    }

    fn render(component: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(component);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn renders_topbar_with_brand_and_controls() {
        let html = render(WrapTopBarClosed);
        assert!(html.contains("v-topbar"));
        assert!(html.contains("v-logo-wrap"));
        assert!(html.contains("v-theme-toggle"));
        assert!(html.contains("v-topbar__menu-btn"));
        assert!(!html.contains("v-topbar__menu-btn active"));

        let open_html = render(WrapTopBarOpen);
        assert!(open_html.contains("v-topbar__menu-btn active"));
    }

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
