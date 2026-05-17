use crate::components::shared::{Button, ButtonVariant, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContactProps {
    #[props(default = "Get in Touch".to_string())]
    pub title: String,
    #[props(default = "Let's start a conversation.".to_string())]
    pub subtitle: String,
}

#[component]
pub fn Contact(props: ContactProps) -> Element {
    rsx! {
        Section {
            id: "contact",
            class: "contact-section",
            div {
                class: "contact-header",
                FadeIn {
                    h2 { "{props.title}" }
                }
                FadeIn {
                    delay_ms: 200,
                    p { "{props.subtitle}" }
                }
            }
            div {
                class: "contact-grid",
                div {
                    class: "contact-form-wrapper",
                    FadeIn {
                        delay_ms: 400,
                        form {
                            class: "contact-form",
                            onsubmit: |_| {},
                            div {
                                class: "form-group",
                                label { "Name" }
                                input {
                                    r#type: "text",
                                    placeholder: "Your name",
                                    class: "form-input",
                                    required: true,
                                }
                            }
                            div {
                                class: "form-group",
                                label { "Email" }
                                input {
                                    r#type: "email",
                                    placeholder: "your@email.com",
                                    class: "form-input",
                                    required: true,
                                }
                            }
                            div {
                                class: "form-group",
                                label { "Message" }
                                textarea {
                                    placeholder: "Tell us about your project",
                                    class: "form-textarea",
                                    required: true,
                                    rows: 5,
                                }
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                button_type: "submit",
                                "Send Message"
                            }
                        }
                    }
                }
                div {
                    class: "contact-info",
                    FadeIn {
                        delay_ms: 600,
                        h3 { "Office" }
                        p { "123 Media Lane" }
                        p { "New York, NY 10001" }
                    }
                    FadeIn {
                        delay_ms: 800,
                        h3 { "Email" }
                        p { a { href: "mailto:hello@velvet.pr", "hello@velvet.pr" } }
                    }
                    FadeIn {
                        delay_ms: 1000,
                        h3 { "Follow" }
                        div { class: "social-links",
                            a { href: "#", "Twitter" }
                            a { href: "#", "LinkedIn" }
                            a { href: "#", "Instagram" }
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
    fn contact_renders_title() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Get in Touch"));
    }

    #[test]
    fn contact_renders_subtitle() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("start a conversation"));
    }

    #[test]
    fn contact_renders_name_field() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Your name"));
        assert!(html.contains(r#"type="text""#));
    }

    #[test]
    fn contact_renders_email_field() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("your@email.com"));
        assert!(html.contains(r#"type="email""#));
    }

    #[test]
    fn contact_renders_message_field() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Tell us about your project"));
        assert!(html.contains("<textarea"));
    }

    #[test]
    fn contact_renders_submit_button() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Send Message"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn contact_renders_office_info() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("123 Media Lane"));
        assert!(html.contains("hello@velvet.pr"));
    }

    #[test]
    fn contact_renders_social_links() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Twitter"));
        assert!(html.contains("LinkedIn"));
        assert!(html.contains("Instagram"));
    }

    #[test]
    fn contact_accepts_custom_title() {
        #[allow(dead_code)]
        fn custom_contact() -> Element {
            rsx! {
                Contact {
                    title: "Contact Us",
                }
            }
        }
        let mut dom = VirtualDom::new(custom_contact);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Contact Us"));
    }

    #[test]
    fn contact_has_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="contact""#));
    }

    #[test]
    fn contact_form_has_required_fields() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Contact {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("required"));
    }
}
