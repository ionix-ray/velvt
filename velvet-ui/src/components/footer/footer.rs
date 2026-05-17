use crate::components::shared::{Button, ButtonVariant, FadeIn};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    #[props(default = "Ready to elevate your narrative?".to_string())]
    pub cta_text: String,
    #[props(default = "Let's Talk".to_string())]
    pub cta_button: String,
}

#[component]
pub fn Footer(props: FooterProps) -> Element {
    rsx! {
        footer {
            class: "footer",
            div {
                class: "container",
                FadeIn {
                    div {
                        class: "footer-cta",
                        h3 { "{props.cta_text}" }
                        div {
                            class: "footer-cta-btn",
                            a {
                                href: "/contact",
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "{props.cta_button}"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "footer-bottom",
                    div {
                        class: "footer-links",
                        a { href: "/services", "Services" }
                        a { href: "/talent", "Talent" }
                        a { href: "/portfolio", "Portfolio" }
                        a { href: "/podcast", "Podcast" }
                        a { href: "/contact", "Contact" }
                    }
                    p { class: "footer-copyright",
                        "© 2026 Velvet PR Agency. All rights reserved."
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
    fn footer_renders_cta_text() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Ready to elevate your narrative"));
    }

    #[test]
    fn footer_renders_cta_button() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Talk"));
        assert!(html.contains("<button"));
    }

    #[test]
    fn footer_renders_navigation_links() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("/services"));
        assert!(html.contains("/talent"));
        assert!(html.contains("/portfolio"));
        assert!(html.contains("/podcast"));
        assert!(html.contains("/contact"));
    }

    #[test]
    fn footer_renders_copyright() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("2026 Velvet PR Agency"));
    }

    #[test]
    fn footer_accepts_custom_cta() {
        #[allow(dead_code)]
        fn custom_footer() -> Element {
            rsx! {
                Footer {
                    cta_text: "Custom CTA",
                    cta_button: "Click Here",
                }
            }
        }
        let mut dom = VirtualDom::new(custom_footer);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom CTA"));
        assert!(html.contains("Click Here"));
    }

    #[test]
    fn footer_uses_fade_in() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }

    #[test]
    fn footer_has_footer_element() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Footer {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("<footer"));
    }
}
