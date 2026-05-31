use dioxus::prelude::*;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

#[derive(Clone, PartialEq)]
pub struct CardData {
    pub card_type: String, // e.g. "showcase" or "event_showcase"
    pub title: String,
    pub content: String,
}

/// A very basic markdown parser that parses standard markdown into elements,
/// and intercepts ```card blocks to render a custom Card component.
#[component]
pub fn MarkdownRenderer(content: String) -> Element {
    let parser = Parser::new(&content);

    // In a real app, you'd build a full AST to VNode tree here.
    // For simplicity and avoiding breaking changes, we'll extract the custom blocks.
    let mut in_card = false;
    let mut current_card_content = String::new();
    let mut cards = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
                if lang.as_ref() == "card" {
                    in_card = true;
                    current_card_content.clear();
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_card {
                    in_card = false;
                    // Parse the card TOML/YAML
                    // Example format:
                    // type = "showcase"
                    // title = "My Case"
                    // desc = "Description"
                    let mut card_type = "showcase".to_string();
                    let mut title = "".to_string();
                    let mut desc = "".to_string();

                    for line in current_card_content.lines() {
                        if let Some((k, v)) = line.split_once('=') {
                            let key = k.trim();
                            let val = v.trim().trim_matches('"');
                            match key {
                                "type" => card_type = val.to_string(),
                                "title" => title = val.to_string(),
                                "desc" => desc = val.to_string(),
                                _ => {}
                            }
                        }
                    }
                    cards.push(CardData {
                        card_type,
                        title,
                        content: desc,
                    });
                }
            }
            Event::Text(text) => {
                if in_card {
                    current_card_content.push_str(&text);
                    current_card_content.push('\n');
                }
            }
            _ => {}
        }
    }

    rsx! {
        div { class: "v-markdown-content",
            for card in cards {
                if card.card_type == "showcase" {
                    div { class: "v-card-modern v-reveal",
                        div { class: "v-card-modern__content",
                            div { class: "v-card-modern__client", "{card.title}" }
                            p { class: "v-card-modern__desc", "{card.content}" }
                        }
                    }
                } else if card.card_type == "event_showcase" {
                    div { class: "v-card-modern v-reveal", style: "border: 1px solid var(--accent);",
                        div { class: "v-card-modern__content",
                            div { class: "v-eyebrow", "Event Showcase" }
                            div { class: "v-card-modern__client", "{card.title}" }
                            p { class: "v-card-modern__desc", "{card.content}" }
                        }
                    }
                }
            }
        }
    }
}
