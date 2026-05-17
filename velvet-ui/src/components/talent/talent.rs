use crate::components::shared::{Card, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TalentItem {
    pub name: String,
    pub role: String,
    pub bio: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct TalentProps {
    #[props(default)]
    pub items: Option<Vec<TalentItem>>,
}

#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn Talent(props: TalentProps) -> Element {
    let default_items = vec![
        TalentItem {
            name: "Alexandra Chen".to_string(),
            role: "Chief Communications Officer".to_string(),
            bio: "20+ years shaping narratives for Fortune 500 CEOs and global brands.".to_string(),
        },
        TalentItem {
            name: "Marcus Rivera".to_string(),
            role: "Event Production Director".to_string(),
            bio: "Orchestrated 500+ high-profile events from intimate galas to stadium spectacles."
                .to_string(),
        },
        TalentItem {
            name: "Sophie Laurent".to_string(),
            role: "Talent Relations Lead".to_string(),
            bio: "Connects A-list talent with brand partnerships that resonate authentically."
                .to_string(),
        },
    ];

    let items = props.items.unwrap_or(default_items);

    rsx! {
        Section {
            id: "talent",
            class: "talent-section",
            div {
                class: "talent-header",
                FadeIn {
                    h2 { "Talent & Events" }
                }
                FadeIn {
                    delay_ms: 200,
                    p { class: "talent-intro", "World-class professionals who amplify your story." }
                }
            }
            div {
                class: "talent-grid",
                for (i, item) in items.iter().enumerate() {
                    FadeIn {
                        delay_ms: (i as u32) * 200,
                        Card {
                            title: item.name.clone(),
                            subtitle: item.role.clone(),
                            p { class: "talent-bio", "{item.bio}" }
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
    fn talent_renders_heading() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Talent {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Talent"));
    }

    #[test]
    fn talent_renders_default_items() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Talent {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Alexandra Chen"));
        assert!(html.contains("Marcus Rivera"));
        assert!(html.contains("Sophie Laurent"));
    }

    #[test]
    fn talent_renders_roles() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Talent {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Chief Communications Officer"));
        assert!(html.contains("Event Production Director"));
    }

    #[test]
    fn talent_accepts_custom_items() {
        #[allow(dead_code)]
        fn custom_talent() -> Element {
            let items = vec![TalentItem {
                name: "Custom Talent".to_string(),
                role: "Custom Role".to_string(),
                bio: "Custom bio".to_string(),
            }];
            rsx! {
                Talent {
                    items: items,
                }
            }
        }
        let mut dom = VirtualDom::new(custom_talent);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom Talent"));
        assert!(html.contains("Custom Role"));
    }

    #[test]
    fn talent_has_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Talent {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="talent""#));
    }

    #[test]
    fn talent_uses_fade_in() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Talent {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }
}
