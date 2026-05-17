use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub subtitle: Option<String>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        article {
            class: "card {props.class}",
            if let Some(title) = &props.title {
                h3 { class: "card-title", "{title}" }
            }
            if let Some(subtitle) = &props.subtitle {
                p { class: "card-subtitle", "{subtitle}" }
            }
            {&props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_renders_children() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    "Card content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Card content"));
        assert!(html.contains("<article"));
        assert!(html.contains("card"));
    }

    #[test]
    fn card_renders_title_when_provided() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    title: "Test Title",
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Test Title"));
        assert!(html.contains("card-title"));
    }

    #[test]
    fn card_renders_subtitle_when_provided() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    subtitle: "Test Subtitle",
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Test Subtitle"));
        assert!(html.contains("card-subtitle"));
    }

    #[test]
    fn card_has_no_title_when_none() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("card-title"));
    }

    #[test]
    fn card_has_no_subtitle_when_none() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(!html.contains("card-subtitle"));
    }

    #[test]
    fn card_accepts_custom_class() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Card {
                    class: "custom-class",
                    "Content"
                }
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("custom-class"));
    }
}
