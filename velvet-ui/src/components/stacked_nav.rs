//! Stacked right-side navigation panel.

use crate::Site;
use crate::theme::brand::brand_mark;
use dioxus::prelude::*;

#[component]
pub fn StackedNav(
    open: bool,
    current_panel: usize,
    site: Site,
    on_navigate: EventHandler<usize>,
) -> Element {
    let class = if open {
        "v-stack-nav open"
    } else {
        "v-stack-nav"
    };
    rsx! {
        nav { class: "{class}", aria_label: "Section navigation",
            div { class: "v-stack-nav__brand",
                img { src: brand_mark(), alt: "VELVT" }
            }
            for (i, item) in site.nav.iter().enumerate() {
                button {
                    class: if i == current_panel { "v-stack-nav__item active" } else { "v-stack-nav__item" },
                    onclick: move |_| on_navigate.call(i),
                    span { class: "v-stack-nav__text", "{item.label}" }

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {


}
