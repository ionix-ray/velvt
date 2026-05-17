use crate::components::shared::{Button, ButtonVariant, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroProps {
    #[props(default = "Velvet".to_string())]
    pub title: String,
    #[props(default = "Premium PR Agency".to_string())]
    pub subtitle: String,
    #[props(default = "Editorial minimalism meets cinematic luxury".to_string())]
    pub description: String,
    #[props(default = "Get in Touch".to_string())]
    pub cta_text: String,
    #[props(default)]
    pub on_cta_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    rsx! {
        Section {
            id: "hero",
            full_width: true,
            class: "hero-section",
            div {
                class: "hero-content",
                FadeIn {
                    delay_ms: 0,
                    h1 { class: "hero-title", "{props.title}" }
                }
                FadeIn {
                    delay_ms: 200,
                    p { class: "hero-subtitle", "{props.subtitle}" }
                }
                FadeIn {
                    delay_ms: 400,
                    p { class: "hero-description", "{props.description}" }
                }
                FadeIn {
                    delay_ms: 600,
                    div {
                        class: "hero-cta",
                        if let Some(handler) = props.on_cta_click {
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |e| handler.call(e),
                                "{props.cta_text}"
                            }
                        } else {
                            Button {
                                variant: ButtonVariant::Primary,
                                "{props.cta_text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_renders_title() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Velvet"));
    }

    #[test]
    fn hero_renders_subtitle() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Premium PR Agency"));
    }

    #[test]
    fn hero_renders_description() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Editorial minimalism meets cinematic luxury"));
    }

    #[test]
    fn hero_renders_cta_button() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Get in Touch"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn hero_accepts_custom_title() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {
                    title: "Custom Title",
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom Title"));
    }

    #[test]
    fn hero_accepts_custom_cta_text() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {
                    cta_text: "Contact Us",
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Contact Us"));
    }

    #[test]
    fn hero_has_hero_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="hero""#));
    }

    #[test]
    fn hero_uses_fade_in_animation() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Hero {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }
}
