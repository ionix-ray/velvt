use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FadeInProps {
    #[props(default)]
    pub delay_ms: u32,
    #[props(default)]
    pub class: String,
    #[props(default = true)]
    pub visible: bool,
    pub children: Element,
}

#[component]
pub fn FadeIn(props: FadeInProps) -> Element {
    let base_class = if props.visible {
        "fade-in visible"
    } else {
        "fade-in"
    };

    rsx! {
        div {
            class: "{base_class} {props.class}",
            style: if props.delay_ms > 0 {
                format!("transition-delay: {}ms", props.delay_ms)
            } else {
                String::new()
            },
            {&props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fadein_renders_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    "Fade content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Fade content"));
        assert!(html.contains("<div"));
    }

    #[test]
    fn fadein_has_fade_in_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }

    #[test]
    fn fadein_visible_adds_visible_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    visible: true,
                    "Visible"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("visible"));
    }

    #[test]
    fn fadein_hidden_no_visible_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    visible: false,
                    "Hidden"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
        assert!(!html.contains(" visible"));
    }

    #[test]
    fn fadein_delay_adds_style() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    delay_ms: 300,
                    "Delayed"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("transition-delay: 300ms"));
    }

    #[test]
    fn fadein_no_delay_when_zero() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    delay_ms: 0,
                    "No Delay"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("transition-delay"));
    }

    #[test]
    fn fadein_accepts_custom_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                FadeIn {
                    class: "custom-fade",
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("custom-fade"));
    }
}
