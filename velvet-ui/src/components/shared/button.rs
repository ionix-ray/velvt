use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub size: ButtonSize,
    #[props(default = "button".to_string())]
    pub button_type: String,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
    };

    let size_class = match props.size {
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "",
        ButtonSize::Large => "btn-lg",
    };

    rsx! {
        button {
            class: "{variant_class} {size_class}",
            r#type: "{props.button_type}",
            disabled: props.disabled,
            onclick: move |e| props.onclick.call(e),
            {&props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_renders_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    "Click Me"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Click Me"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn button_primary_has_primary_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    variant: ButtonVariant::Primary,
                    "Primary"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("btn-primary"));
    }

    #[test]
    fn button_secondary_has_secondary_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    variant: ButtonVariant::Secondary,
                    "Secondary"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("btn-secondary"));
    }

    #[test]
    fn button_disabled_when_set() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    disabled: true,
                    "Disabled"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("disabled"));
    }

    #[test]
    fn button_default_variant_is_primary() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    "Default"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("btn-primary"));
    }

    #[test]
    fn button_type_defaults_to_button() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    "Submit"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"type="button""#));
    }

    #[test]
    fn button_size_small_adds_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    size: ButtonSize::Small,
                    "Small"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("btn-sm"));
    }

    #[test]
    fn button_size_large_adds_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Button {
                    size: ButtonSize::Large,
                    "Large"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("btn-lg"));
    }
}
