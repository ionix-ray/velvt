use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub route: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    #[props(default)]
    pub items: Option<Vec<NavItem>>,
    #[props(default = false)]
    pub scrolled: bool,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let default_items = vec![
        NavItem {
            label: "Services".to_string(),
            route: "/services".to_string(),
        },
        NavItem {
            label: "Talent".to_string(),
            route: "/talent".to_string(),
        },
        NavItem {
            label: "Portfolio".to_string(),
            route: "/portfolio".to_string(),
        },
        NavItem {
            label: "Podcast".to_string(),
            route: "/podcast".to_string(),
        },
        NavItem {
            label: "Contact".to_string(),
            route: "/contact".to_string(),
        },
    ];

    let items = props.items.unwrap_or(default_items);
    let nav_class = if props.scrolled {
        "navbar navbar-scrolled"
    } else {
        "navbar"
    };

    rsx! {
        nav {
            class: "{nav_class}",
            role: "navigation",
            "aria-label": "Main navigation",
            div {
                class: "navbar-brand",
                a {
                    href: "/",
                    class: "navbar-logo",
                    "Velvet"
                }
            }
            div {
                class: "navbar-links",
                for item in items {
                    a {
                        href: "{item.route}",
                        class: "navbar-link",
                        "{item.label}"
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
    fn navbar_renders_logo() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Velvet"));
    }

    #[test]
    fn navbar_renders_default_links() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Services"));
        assert!(html.contains("Talent"));
        assert!(html.contains("Portfolio"));
        assert!(html.contains("Podcast"));
        assert!(html.contains("Contact"));
    }

    #[test]
    fn navbar_renders_custom_items() {
        #[allow(dead_code)]
        fn custom_navbar() -> Element {
            let items = vec![NavItem {
                label: "Custom".to_string(),
                route: "/custom".to_string(),
            }];
            rsx! {
                Navbar {
                    items: items,
                }
            }
        }
        let mut dom = VirtualDom::new(custom_navbar);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom"));
    }

    #[test]
    fn navbar_has_nav_role() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"role="navigation""#));
    }

    #[test]
    fn navbar_has_aria_label() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"aria-label="Main navigation""#));
    }

    #[test]
    fn navbar_scrolled_adds_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {
                    scrolled: true,
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("navbar-scrolled"));
    }

    #[test]
    fn navbar_not_scrolled_no_extra_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {
                    scrolled: false,
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("navbar-scrolled"));
    }

    #[test]
    fn navbar_links_have_correct_routes() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Navbar {}
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
}
