//! Next hint — bottom center arrow prompt.

use dioxus::prelude::*;

#[component]
pub fn NextHint(hidden: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let class = if hidden {
        "v-next-hint hidden"
    } else {
        "v-next-hint"
    };
    rsx! {
        div {
            class: "{class}",
            onclick: move |evt| onclick.call(evt),
            span { "Next" }
            div { class: "v-next-hint__arrow" }
        }
    }
}
