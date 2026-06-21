use dioxus::prelude::*;

#[component]
pub fn NextHint(
    label: String,
    hidden: bool,
    direction: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = hint_class(hidden, &direction);
    let arrow_class = arrow_class(&direction);
    rsx! {
        div {
            class: "{class}",
            onclick: move |evt| onclick.call(evt),
            span { "{label}" }
            div { class: "{arrow_class}" }
        }
    }
}

fn hint_class(hidden: bool, direction: &str) -> String {
    let base_class = if hidden {
        "v-next-hint hidden"
    } else {
        "v-next-hint"
    };
    match direction {
        "left" => format!("{base_class} v-next-hint--left"),
        "right" => format!("{base_class} v-next-hint--right"),
        _ => base_class.to_string(),
    }
}

fn arrow_class(direction: &str) -> &'static str {
    if direction == "left" {
        "v-next-hint__arrow v-next-hint__arrow--left"
    } else {
        "v-next-hint__arrow v-next-hint__arrow--right"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hint_class_adds_hidden_and_direction_modifiers() {
        assert_eq!(hint_class(false, "left"), "v-next-hint v-next-hint--left");
        assert_eq!(hint_class(false, "right"), "v-next-hint v-next-hint--right");
        assert_eq!(
            hint_class(true, "left"),
            "v-next-hint hidden v-next-hint--left"
        );
        assert_eq!(hint_class(false, "anything"), "v-next-hint");
    }

    #[test]
    fn arrow_class_defaults_to_right() {
        assert_eq!(
            arrow_class("left"),
            "v-next-hint__arrow v-next-hint__arrow--left"
        );
        assert_eq!(
            arrow_class("right"),
            "v-next-hint__arrow v-next-hint__arrow--right"
        );
        assert_eq!(
            arrow_class("anything"),
            "v-next-hint__arrow v-next-hint__arrow--right"
        );
    }
}
