use crate::components::shared::{Button, ButtonVariant, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PodcastProps {
    #[props(default = "Coming Soon".to_string())]
    pub title: String,
    #[props(default = "The Velvet Podcast".to_string())]
    pub subtitle: String,
    #[props(default = "Conversations with the voices shaping culture, media, and influence.".to_string())]
    pub description: String,
}

#[component]
pub fn Podcast(props: PodcastProps) -> Element {
    rsx! {
        Section {
            id: "podcast",
            class: "podcast-section",
            div {
                class: "podcast-content",
                FadeIn {
                    div {
                        class: "podcast-badge",
                        "Coming Soon"
                    }
                }
                FadeIn {
                    delay_ms: 200,
                    h2 { "{props.subtitle}" }
                }
                FadeIn {
                    delay_ms: 400,
                    p { class: "podcast-description", "{props.description}" }
                }
                FadeIn {
                    delay_ms: 600,
                    div {
                        class: "podcast-notify",
                        form {
                            class: "notify-form",
                            onsubmit: |_| {},
                            input {
                                r#type: "email",
                                placeholder: "Enter your email",
                                class: "notify-input",
                                required: true,
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                button_type: "submit",
                                "Notify Me"
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
    fn podcast_renders_subtitle() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("The Velvet Podcast"));
    }

    #[test]
    fn podcast_renders_description() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Conversations with the voices"));
    }

    #[test]
    fn podcast_renders_coming_soon_badge() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Coming Soon"));
    }

    #[test]
    fn podcast_renders_email_form() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<form"));
        assert!(html.contains(r#"type="email""#));
    }

    #[test]
    fn podcast_renders_notify_button() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Notify Me"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn podcast_accepts_custom_title() {
        #[allow(dead_code)]
        fn custom_podcast() -> Element {
            rsx! {
                Podcast {
                    subtitle: "Custom Show",
                }
            }
        }
        let mut dom = VirtualDom::new(custom_podcast);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom Show"));
    }

    #[test]
    fn podcast_has_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="podcast""#));
    }

    #[test]
    fn podcast_uses_fade_in() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Podcast {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }
}
