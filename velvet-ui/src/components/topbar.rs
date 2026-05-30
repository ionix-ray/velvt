//! Top bar — fixed brand + menu toggle + theme toggle.

use dioxus::prelude::*;

#[component]
pub fn TopBar(menu_open: Signal<bool>, theme: Signal<String>) -> Element {
    let theme_icon = if *theme.read() == "dark" {
        "\u{2600}"
    } else {
        "\u{263E}"
    };

    rsx! {
        header { class: "v-topbar", id: "topbar",
            div { class: "v-topbar__brand",
                img { src: asset!("/assets/images/velvt-logo.png"), alt: "VELVT" }
            }
            div { class: "v-topbar__actions",
                button {
                    class: "v-theme-toggle",
                    title: "Toggle theme",
                    onclick: move |_| {
                        let current = theme.read().clone();
                        let next = if current == "dark" { "light".to_string() } else { "dark".to_string() };
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
