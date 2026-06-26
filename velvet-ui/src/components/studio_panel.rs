//! Studio panel — event showcase block grid.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn StudioPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel v-showcase-panel", id: "showcase",
            div { class: "v-section v-showcase__section",
                div { class: "v-container",
                    div { class: "v-panel-header v-showcase__header v-reveal",
                        span { class: "v-eyebrow", "Event Showcase" }
                        h2 { class: "v-display-2", "{site.studio.title}" }
                        p { class: "v-panel-header__sub", "{site.studio.sub}" }
                    }
                    div { class: "v-showcase__grid",
                        for (i, item) in site.studio.items.iter().enumerate() {
                            div {
                                class: "{showcase_item_classes(i, site.studio.items.len())}",
                                style: "transition-delay: {(i + 1) * 60}ms;",
                                span { class: "v-tile__eyebrow", "{item.tag}" }
                                h4 { class: "v-tile__title", "{item.title}" }
                                p { class: "v-tile__desc", "{item.body}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Full class list for a showcase card at `index` out of `total` items.
/// Carbon-tile base + `--showcase` modifier so a single tile system styles
/// both the showcase grid and the cases grid.
fn showcase_item_classes(index: usize, total: usize) -> String {
    let extra = showcase_span_class(index, total);
    if extra.is_empty() {
        "v-tile v-tile--showcase v-reveal".to_string()
    } else {
        format!("v-tile v-tile--showcase v-reveal {extra}")
    }
}

/// Extra grid-span modifier for an item that would otherwise be left in an
/// incomplete trailing row of the 3-column showcase grid.
fn showcase_span_class(index: usize, total: usize) -> &'static str {
    if total == 0 || index != total - 1 {
        return "";
    }
    match total % 3 {
        1 => "v-tile--full",
        2 => "v-tile--wide",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_row_counts_get_no_span_modifier() {
        for total in [0, 3, 6, 9] {
            for i in 0..total {
                assert_eq!(showcase_span_class(i, total), "");
            }
        }
    }

    #[test]
    fn lone_trailing_item_spans_the_full_row() {
        assert_eq!(showcase_span_class(3, 4), "v-tile--full");
        assert_eq!(showcase_span_class(6, 7), "v-tile--full");
    }

    #[test]
    fn two_trailing_items_widen_only_the_last() {
        assert_eq!(showcase_span_class(3, 5), "");
        assert_eq!(showcase_span_class(4, 5), "v-tile--wide");
    }

    #[test]
    fn non_last_items_never_get_a_span_modifier() {
        assert_eq!(showcase_span_class(0, 5), "");
        assert_eq!(showcase_span_class(1, 5), "");
    }

    #[test]
    fn showcase_item_classes_appends_modifier_with_single_space() {
        assert_eq!(
            showcase_item_classes(0, 3),
            "v-tile v-tile--showcase v-reveal"
        );
        assert_eq!(
            showcase_item_classes(4, 5),
            "v-tile v-tile--showcase v-reveal v-tile--wide"
        );
    }
}
