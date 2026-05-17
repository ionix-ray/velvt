use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionProps {
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub full_width: bool,
    pub children: Element,
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    let container_class = if props.full_width {
        "section-full".to_string()
    } else {
        "container".to_string()
    };

    if let Some(id) = &props.id {
        let id_clone = id.clone();
        rsx! {
            section {
                id: "{id_clone}",
                class: "section {props.class}",
                div {
                    class: "{container_class}",
                    {&props.children}
                }
            }
        }
    } else {
        rsx! {
            section {
                class: "section {props.class}",
                div {
                    class: "{container_class}",
                    {&props.children}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_renders_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    "Section content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Section content"));
        assert!(html.contains("<section"));
    }

    #[test]
    fn section_has_container_wrapper_by_default() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("container"));
    }

    #[test]
    fn section_full_width_uses_section_full_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    full_width: true,
                    "Full Width"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("section-full"));
        assert!(!html.contains("container"));
    }

    #[test]
    fn section_accepts_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    id: Some("hero-section".to_string()),
                    "Hero"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="hero-section""#));
    }

    #[test]
    fn section_accepts_custom_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    class: "custom-section",
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("custom-section"));
    }

    #[test]
    fn section_no_id_when_none() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Section {
                    "No ID"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("id="));
    }
}
