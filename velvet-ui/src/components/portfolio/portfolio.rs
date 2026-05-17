use crate::components::shared::{Card, FadeIn, Section};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PortfolioItem {
    pub title: String,
    pub client: String,
    pub description: String,
    pub result: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct PortfolioProps {
    #[props(default)]
    pub items: Option<Vec<PortfolioItem>>,
}

#[allow(clippy::cast_possible_truncation)]
#[component]
pub fn Portfolio(props: PortfolioProps) -> Element {
    let default_items = vec![
        PortfolioItem {
            title: "Global Product Launch".to_string(),
            client: "TechCorp International".to_string(),
            description: "Orchestrated a 3-continent media campaign for a flagship product reveal.".to_string(),
            result: "2.5B impressions, 400% increase in brand awareness.".to_string(),
        },
        PortfolioItem {
            title: "Crisis Turnaround".to_string(),
            client: "Fortune 100 Retailer".to_string(),
            description: "Managed reputation recovery after a high-profile supply chain scandal.".to_string(),
            result: "Sentiment shifted from -65% to +22% in 90 days.".to_string(),
        },
        PortfolioItem {
            title: "Executive Visibility".to_string(),
            client: "Healthcare CEO".to_string(),
            description: "Built a thought leadership platform through op-eds, podcasts, and keynote placements.".to_string(),
            result: "CEO named in Top 50 Healthcare Leaders by industry press.".to_string(),
        },
    ];

    let items = props.items.unwrap_or(default_items);

    rsx! {
        Section {
            id: "portfolio",
            class: "portfolio-section",
            div {
                class: "portfolio-header",
                FadeIn {
                    h2 { "Portfolio" }
                }
                FadeIn {
                    delay_ms: 200,
                    p { class: "portfolio-intro", "Real results for real clients." }
                }
            }
            div {
                class: "portfolio-grid",
                for (i, item) in items.iter().enumerate() {
                    FadeIn {
                        delay_ms: (i as u32) * 200,
                        Card {
                            title: item.title.clone(),
                            subtitle: item.client.clone(),
                            div {
                                class: "portfolio-details",
                                p { "{item.description}" }
                                p { class: "portfolio-result", strong { "Result: " } "{item.result}" }
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
    fn portfolio_renders_heading() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Portfolio"));
    }

    #[test]
    fn portfolio_renders_default_items() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Global Product Launch"));
        assert!(html.contains("Crisis Turnaround"));
        assert!(html.contains("Executive Visibility"));
    }

    #[test]
    fn portfolio_renders_clients() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("TechCorp International"));
        assert!(html.contains("Fortune 100 Retailer"));
    }

    #[test]
    fn portfolio_renders_results() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("2.5B impressions"));
        assert!(html.contains("Sentiment shifted"));
    }

    #[test]
    fn portfolio_accepts_custom_items() {
        #[allow(dead_code)]
        fn custom_portfolio() -> Element {
            let items = vec![PortfolioItem {
                title: "Custom Case".to_string(),
                client: "Custom Client".to_string(),
                description: "Custom description".to_string(),
                result: "Custom result".to_string(),
            }];
            rsx! {
                Portfolio {
                    items: items,
                }
            }
        }
        let mut dom = VirtualDom::new(custom_portfolio);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Custom Case"));
    }

    #[test]
    fn portfolio_has_section_id() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(r#"id="portfolio""#));
    }

    #[test]
    fn portfolio_uses_fade_in() {
        let mut dom = VirtualDom::new(|| {
            rsx! {
                Portfolio {}
            }
        });
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("fade-in"));
    }
}
