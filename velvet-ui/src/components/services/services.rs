use crate::components::shared::{Card, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ServiceItem {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ServicesProps {
    #[props(default)]
    pub items: Option<Vec<ServiceItem>>,
}

#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn Services(props: ServicesProps) -> Element {
    let default_items = vec![
        ServiceItem {
            title: "Public Relations".to_string(),
            description: "Strategic media placement, press releases, and brand positioning for maximum visibility.".to_string(),
            icon: "📰".to_string(),
        },
        ServiceItem {
            title: "Media Relations".to_string(),
            description: "Deep journalist networks, exclusive interviews, and earned media coverage.".to_string(),
            icon: "🎙️".to_string(),
        },
        ServiceItem {
            title: "Crisis Communications".to_string(),
            description: "Rapid response protocols, reputation management, and stakeholder alignment.".to_string(),
            icon: "🛡️".to_string(),
        },
    ];

    let items = props.items.unwrap_or(default_items);

    rsx! {
        Section {
            id: "services",
            class: "services-section",
            div {
                class: "services-header",
                FadeIn {
                    h2 { "Our Services" }
                }
                FadeIn {
                    delay_ms: 200,
                    p { class: "services-intro", "Strategic communications tailored to your narrative." }
                }
            }
            div {
                class: "services-grid",
                for (i, item) in items.iter().enumerate() {
                    FadeIn {
                        delay_ms: (i as u32) * 200,
                        Card {
                            title: item.title.clone(),
                            subtitle: item.description.clone(),
                            div {
                                class: "service-icon",
                                "{item.icon}"
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
    fn services_renders_heading() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Our Services"));
    }

    #[test]
    fn services_renders_default_items() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Public Relations"));
        assert!(html.contains("Media Relations"));
        assert!(html.contains("Crisis Communications"));
    }

    #[test]
    fn services_renders_descriptions() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Strategic media placement"));
        assert!(html.contains("Deep journalist networks"));
        assert!(html.contains("Rapid response protocols"));
    }

    #[test]
    fn services_accepts_custom_items() {
        #[allow(dead_code)]
        fn custom_services() -> Element {
            let items = vec![ServiceItem {
                title: "Custom Service".to_string(),
                description: "Custom description".to_string(),
                icon: "⭐".to_string(),
            }];
            rsx! {
                Services {
                    items: items,
                }
            }
        }
        let mut dom = VirtualDom::new(custom_services);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom Service"));
        assert!(html.contains("Custom description"));
    }

    #[test]
    fn services_has_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="services""#));
    }

    #[test]
    fn services_uses_fade_in() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }

    #[test]
    fn services_renders_three_cards_by_default() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Services {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Public Relations"));
        assert!(html.contains("Media Relations"));
        assert!(html.contains("Crisis Communications"));
    }
}
