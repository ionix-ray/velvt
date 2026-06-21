//! Home — the only route. Composes the full horizontal-scroll page.
//! All navigation state lives here; child components are purely reactive.

use crate::Site;
use crate::components::{
    about_aggregated_panel::AboutAggregatedPanel, cases_panel::CasesPanel, cta_panel::CtaPanel,
    footer_panel::FooterPanel, hero_panel::HeroPanel, loader::Loader, mobile_nav::MobileNav,
    next_hint::NextHint, process_panel::ProcessPanel, scroll_progress::ScrollProgress,
    section_dots::SectionDots, social_strip::SocialStrip, stacked_nav::StackedNav,
    studio_panel::StudioPanel, topbar::TopBar,
};
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Panel labels — displayed in the spindle navigator.
/// The 7th panel is the footer; SocialStrip hides on it.
const PANEL_LABELS: &[&str] = &[
    "01 HOME",
    "02 ABOUT",
    "03 STORIES",
    "04 SHOWCASE",
    "05 PORTFOLIO",
    "06 CONTACT",
    "07 FOOTER",
];

/// URL-hash anchors, one per panel, in render order. Mirrors each panel
/// section's own `id` attribute so `#showcase` etc. is shareable/bookmarkable.
const PANEL_ANCHORS: &[&str] = &[
    "home", "about", "stories", "showcase", "cases", "contact", "footer",
];

/// Anchor slug for a panel index. Falls back to the first anchor for an
/// out-of-range index rather than panicking.
fn anchor_for(idx: usize) -> &'static str {
    PANEL_ANCHORS.get(idx).copied().unwrap_or(PANEL_ANCHORS[0])
}

/// Panel index for an anchor slug (without the leading `#`), if recognized.
fn panel_index_for_anchor(anchor: &str) -> Option<usize> {
    PANEL_ANCHORS.iter().position(|a| *a == anchor)
}

/// Panel index implied by the current page's URL hash on load, or `0`.
fn initial_panel_index() -> usize {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .and_then(|h| panel_index_for_anchor(h.trim_start_matches('#')))
        .unwrap_or(0)
}

/// Reflect the active panel in the URL hash via `history.replaceState`,
/// so the address bar updates without triggering a browser scroll/jump.
fn set_url_anchor(idx: usize) {
    if let Some(win) = web_sys::window() {
        if let Ok(history) = win.history() {
            let hash = format!("#{}", anchor_for(idx));
            let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&hash));
        }
    }
}

/// Next panel index for an `ArrowRight`/`ArrowLeft` (etc.) keypress, or
/// `None` if the key is unrecognized or would move out of range.
fn keyboard_nav_index(key: &str, current: usize, len: usize) -> Option<usize> {
    match key {
        "ArrowRight" | "ArrowDown" => {
            if current < len.saturating_sub(1) {
                Some(current + 1)
            } else {
                None
            }
        }
        "ArrowLeft" | "ArrowUp" => {
            if current > 0 {
                Some(current - 1)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Next panel index for a wheel/trackpad scroll, or `None` if the delta is
/// too small to count as a deliberate gesture, or would move out of range.
fn wheel_nav_index(delta_y: f64, current: usize, len: usize) -> Option<usize> {
    if delta_y.abs() < 8.0 {
        return None;
    }
    if delta_y > 0.0 {
        if current < len.saturating_sub(1) {
            Some(current + 1)
        } else {
            None
        }
    } else if current > 0 {
        Some(current - 1)
    } else {
        None
    }
}

/// Panel index implied by the panels container's current scroll position.
fn scroll_sync_index(scroll_left: f64, client_width: f64, len: usize) -> usize {
    let idx = (scroll_left / client_width).round() as usize;
    if len == 0 { 0 } else { idx.min(len - 1) }
}

/// Spindle progress fraction (0.0..=1.0) for the current panel.
fn progress_for(current: usize, count: usize) -> f64 {
    if count > 1 {
        current as f64 / (count - 1) as f64
    } else {
        0.0
    }
}

/// Smooth-scroll `.v-panels` to the panel at `idx` and sync the URL hash.
fn scroll_to_panel(idx: usize) {
    set_url_anchor(idx);
    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            if let Some(panels) = doc.query_selector(".v-panels").ok().flatten() {
                let target = idx as f64 * panels.client_width() as f64;
                let opts = web_sys::ScrollToOptions::new();
                opts.set_left(target);
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                panels.scroll_to_with_scroll_to_options(&opts);
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    let site = Site::load().clone();

    let loader_hidden = use_signal(|| false);
    let mut menu_open = use_signal(|| false);
    let theme = use_signal(|| "dark".to_string());
    let mut current_panel = use_signal(|| 0usize);

    // Land on the panel named by the URL hash (e.g. `#showcase`) on first paint.
    // Deferred to an effect: `web_sys::window()` is browser-only and must
    // never run synchronously in the component body (it panics under SSR).
    {
        use_effect(move || {
            let initial = initial_panel_index();
            if initial != 0 {
                current_panel.set(initial);
                scroll_to_panel(initial);
            }
        });
    }

    // Apply theme attribute to <html>
    {
        use_effect(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(root) = doc.document_element() {
                        let _ = root.set_attribute("data-theme", &theme.read());
                    }
                }
            }
        });
    }

    // Hide loader after 2.2 s
    let mut timeout_handle = use_signal(|| Option::<gloo_timers::callback::Timeout>::None);
    {
        use_effect(move || {
            let mut loader = loader_hidden;
            let handle = gloo_timers::callback::Timeout::new(2200, move || {
                loader.set(true);
            });
            timeout_handle.set(Some(handle));
        });
    }

    // Navigation helper — shared by spindle clicks, menu clicks, key events
    let mut on_navigate = move |idx: usize| {
        current_panel.set(idx);
        menu_open.set(false);
        scroll_to_panel(idx);
    };

    // Keyboard arrow navigation + scroll-position sync
    {
        use_effect(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    let doc_scroll = doc.clone();

                    // ── Keyboard handler ─────────────────────────────────
                    let kbd =
                        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                            move |ev: web_sys::KeyboardEvent| {
                                let cur = *current_panel.read();
                                let len = PANEL_LABELS.len();
                                let Some(idx) = keyboard_nav_index(&ev.key(), cur, len) else {
                                    return;
                                };
                                ev.prevent_default();
                                current_panel.set(idx);
                                menu_open.set(false);
                                scroll_to_panel(idx);
                            },
                        );

                    // ── Wheel / trackpad handler ─────────────────────────
                    let wheel =
                        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::WheelEvent)>::new(
                            move |ev: web_sys::WheelEvent| {
                                let cur = *current_panel.read();
                                let len = PANEL_LABELS.len();
                                let Some(idx) = wheel_nav_index(ev.delta_y(), cur, len) else {
                                    return;
                                };
                                ev.prevent_default();
                                ev.stop_propagation();
                                current_panel.set(idx);
                                menu_open.set(false);
                                scroll_to_panel(idx);
                            },
                        );

                    // ── Scroll listener — keeps spindle in sync with touch/scroll ─
                    let scroll = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                        if let Some(panels) = doc_scroll.query_selector(".v-panels").ok().flatten()
                        {
                            let idx = scroll_sync_index(
                                panels.scroll_left() as f64,
                                panels.client_width() as f64,
                                PANEL_LABELS.len(),
                            );
                            if idx != *current_panel.read() {
                                current_panel.set(idx);
                                set_url_anchor(idx);
                            }
                        }
                    });

                    let kbd_js = kbd.as_ref().unchecked_ref();
                    let _ = win.add_event_listener_with_callback("keydown", kbd_js);
                    kbd.forget();

                    let wheel_js = wheel.as_ref().unchecked_ref();
                    let _ = win.add_event_listener_with_callback("wheel", wheel_js);
                    wheel.forget();

                    if let Some(panels) = doc.query_selector(".v-panels").ok().flatten() {
                        let scroll_js = scroll.as_ref().unchecked_ref();
                        let _ = panels.add_event_listener_with_callback("scroll", scroll_js);
                    }
                    scroll.forget();
                }
            }
        });
    }

    let panel_count = PANEL_LABELS.len();
    let labels: Vec<String> = PANEL_LABELS.iter().map(|s| s.to_string()).collect();
    let progress = progress_for(*current_panel.read(), panel_count);
    let is_footer_panel = *current_panel.read() >= panel_count - 1;

    rsx! {
        Loader { hidden: *loader_hidden.read() }

        ScrollProgress { progress }

        TopBar { menu_open, theme }

        StackedNav {
            open: *menu_open.read(),
            current_panel: *current_panel.read(),
            site: site.clone(),
            on_navigate,
        }

        SectionDots {
            count: panel_count,
            current: *current_panel.read(),
            labels,
            on_navigate,
        }

        // Social icons — visible on all panels except the footer
        SocialStrip { is_last_panel: is_footer_panel }

        // Next hint shows on panels 0..n-2, Previous shows on panels 1..n-1
        NextHint {
            label: "Next",
            hidden: *current_panel.read() >= panel_count - 1,
            direction: "right",
            onclick: move |_| {
                let cur = *current_panel.read();
                if cur < panel_count - 1 {
                    on_navigate(cur + 1);
                }
            }
        }
        NextHint {
            label: "Previous",
            hidden: *current_panel.read() == 0,
            direction: "left",
            onclick: move |_| {
                let cur = *current_panel.read();
                if cur > 0 {
                    on_navigate(cur - 1);
                }
            }
        }

        // ── Mobile bottom nav (≤768px) ───────────────────────────────────
        MobileNav {
            current_panel: *current_panel.read(),
            panel_count,
            on_navigate: EventHandler::new(move |idx: usize| {
                on_navigate(idx);
            }),
        }

        // ── Horizontal scroll panels ──────────────────────────────────────
        div { class: "v-panels",
            HeroPanel { site: site.clone() }
            AboutAggregatedPanel { site: site.clone() }
            ProcessPanel { site: site.clone() }
            StudioPanel { site: site.clone() }
            CasesPanel { site: site.clone() }
            CtaPanel { site: site.clone() }
            // 7th panel — footer rolls in after the last content panel
            FooterPanel { site }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_nav_advances_and_clamps_at_bounds() {
        assert_eq!(keyboard_nav_index("ArrowRight", 0, 3), Some(1));
        assert_eq!(keyboard_nav_index("ArrowDown", 1, 3), Some(2));
        assert_eq!(keyboard_nav_index("ArrowRight", 2, 3), None);
        assert_eq!(keyboard_nav_index("ArrowLeft", 2, 3), Some(1));
        assert_eq!(keyboard_nav_index("ArrowUp", 0, 3), None);
        assert_eq!(keyboard_nav_index("Enter", 1, 3), None);
    }

    #[test]
    fn wheel_nav_ignores_small_deltas_and_clamps_at_bounds() {
        assert_eq!(wheel_nav_index(2.0, 0, 3), None);
        assert_eq!(wheel_nav_index(10.0, 0, 3), Some(1));
        assert_eq!(wheel_nav_index(10.0, 2, 3), None);
        assert_eq!(wheel_nav_index(-10.0, 1, 3), Some(0));
        assert_eq!(wheel_nav_index(-10.0, 0, 3), None);
    }

    #[test]
    fn scroll_sync_index_rounds_and_caps_at_last_panel() {
        assert_eq!(scroll_sync_index(0.0, 1000.0, 7), 0);
        assert_eq!(scroll_sync_index(1000.0, 1000.0, 7), 1);
        assert_eq!(scroll_sync_index(6600.0, 1000.0, 7), 6);
        assert_eq!(scroll_sync_index(99_000.0, 1000.0, 7), 6);
        assert_eq!(scroll_sync_index(0.0, 1000.0, 0), 0);
    }

    #[test]
    fn progress_for_is_zero_at_start_and_one_at_end() {
        assert_eq!(progress_for(0, 7), 0.0);
        assert_eq!(progress_for(6, 7), 1.0);
        assert!((progress_for(3, 7) - 0.5).abs() < 1e-9);
        assert_eq!(progress_for(0, 1), 0.0);
        assert_eq!(progress_for(0, 0), 0.0);
    }

    #[test]
    fn anchor_for_matches_panel_render_order() {
        assert_eq!(anchor_for(0), "home");
        assert_eq!(anchor_for(3), "showcase");
        assert_eq!(anchor_for(6), "footer");
        assert_eq!(anchor_for(99), "home");
    }

    #[test]
    fn panel_index_for_anchor_round_trips_with_anchor_for() {
        for i in 0..PANEL_ANCHORS.len() {
            assert_eq!(panel_index_for_anchor(anchor_for(i)), Some(i));
        }
        assert_eq!(panel_index_for_anchor("not-a-panel"), None);
        assert_eq!(panel_index_for_anchor(""), None);
    }
}
