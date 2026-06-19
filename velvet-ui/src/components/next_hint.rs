use dioxus::prelude::*;

#[component]
pub fn NextHint(
    label: String,
    hidden: bool,
    direction: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let base_class = if hidden {
        "v-next-hint hidden"
    } else {
        "v-next-hint"
    };
    let class = match direction.as_str() {
        "left" => format!("{} v-next-hint--left", base_class),
        "right" => format!("{} v-next-hint--right", base_class),
        _ => base_class.to_string(),
    };
    let arrow_class = if direction == "left" {
        "v-next-hint__arrow v-next-hint__arrow--left"
    } else {
        "v-next-hint__arrow v-next-hint__arrow--right"
    };
    rsx! {
        div {
            class: "{class}",
            onclick: move |evt| onclick.call(evt),
            span { "{label}" }
            div { class: "{arrow_class}" }
        }
    }
}
