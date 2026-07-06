//! Studio panel — event showcase block grid.

use crate::components::card_step::CardStep;
use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn StudioPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel v-showcase-panel", id: "experience",
            div { class: "v-section v-showcase__section",
                div { class: "v-container",
                    div { class: "v-panel-header v-showcase__header v-reveal",
                        if !site.studio.eyebrow.is_empty() {
                            span { class: "v-eyebrow", "{site.studio.eyebrow}" }
                        }
                        h2 { class: "v-display-2", "{site.studio.title}" }
                        p { class: "v-panel-header__sub", "{site.studio.sub}" }
                    }
                    div { class: "v-showcase__grid",
                        for (i, item) in site.studio.items.iter().enumerate() {
                            CardStep {
                                logo: site.hero.logo.to_string(),
                                tag: Some(item.tag.to_string()),
                                title: item.title.to_string(),
                                body: item.body.to_string(),
                                class_extra: showcase_class_extra(i, site.studio.items.len()),
                                delay_ms: (i + 1) * 60,
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Extra CSS class for a showcase card, based on its position in the grid.
/// Returns "v-tile--showcase" always, plus a span modifier for orphaned
/// trailing items that would leave an incomplete last row in the 3-col grid.
fn showcase_class_extra(index: usize, total: usize) -> String {
    let span = showcase_span_class(index, total);
    if span.is_empty() {
        "v-tile--showcase".to_string()
    } else {
        format!("v-tile--showcase {span}")
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
    fn showcase_class_extra_always_includes_showcase_base() {
        assert!(showcase_class_extra(0, 3).contains("v-tile--showcase"));
        assert!(showcase_class_extra(4, 5).contains("v-tile--showcase"));
        assert!(showcase_class_extra(4, 5).contains("v-tile--wide"));
    }

    #[test]
    fn showcase_class_extra_is_just_base_for_non_orphan() {
        assert_eq!(showcase_class_extra(0, 3), "v-tile--showcase");
        assert_eq!(showcase_class_extra(1, 6), "v-tile--showcase");
    }
}
