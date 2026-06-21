//! Studio panel — event showcase masonry, compact to fit 100vh.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn StudioPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "showcase",
            div { class: "v-section",
                div { class: "v-container",
                    div { class: "v-panel-header v-reveal",
                        span { class: "v-eyebrow", "Event Showcase" }
                        h2 { class: "v-display-2", "{site.studio.title}" }
                        p { class: "v-panel-header__sub", "{site.studio.sub}" }
                    }
                    div { class: "v-masonry",
                        for (i, item) in site.studio.items.iter().enumerate() {
                            div {
                                class: "{item_classes(i, site.studio.items.len())}",
                                style: "transition-delay: {(i + 1) * 60}ms;",
                                div { class: "v-masonry__content",
                                    span { class: "v-masonry__tag", "{item.tag}" }
                                    h4 { "{item.title}" }
                                    p { "{item.body}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Extra grid-span modifier for an item that would otherwise be left in an
/// incomplete trailing row of the 3-column masonry grid: a lone leftover
/// item spans the full row, two leftover items widen the last one — so the
/// grid never shows a dangling empty cell regardless of item count.
fn item_span_class(index: usize, total: usize) -> &'static str {
    if total == 0 || index != total - 1 {
        return "";
    }
    match total % 3 {
        1 => "v-masonry__item--full",
        2 => "v-masonry__item--wide",
        _ => "",
    }
}

/// Full class list for a masonry card at `index` out of `total` items.
fn item_classes(index: usize, total: usize) -> String {
    let extra = item_span_class(index, total);
    if extra.is_empty() {
        "v-masonry__item v-reveal".to_string()
    } else {
        format!("v-masonry__item v-reveal {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_row_counts_get_no_span_modifier() {
        for total in [0, 3, 6, 9] {
            for i in 0..total {
                assert_eq!(item_span_class(i, total), "");
            }
        }
    }

    #[test]
    fn lone_trailing_item_spans_the_full_row() {
        assert_eq!(item_span_class(3, 4), "v-masonry__item--full");
        assert_eq!(item_span_class(6, 7), "v-masonry__item--full");
    }

    #[test]
    fn two_trailing_items_widen_only_the_last() {
        assert_eq!(item_span_class(3, 5), "");
        assert_eq!(item_span_class(4, 5), "v-masonry__item--wide");
    }

    #[test]
    fn non_last_items_never_get_a_span_modifier() {
        assert_eq!(item_span_class(0, 5), "");
        assert_eq!(item_span_class(1, 5), "");
    }

    #[test]
    fn item_classes_appends_modifier_with_single_space() {
        assert_eq!(item_classes(0, 3), "v-masonry__item v-reveal");
        assert_eq!(
            item_classes(4, 5),
            "v-masonry__item v-reveal v-masonry__item--wide"
        );
    }
}
