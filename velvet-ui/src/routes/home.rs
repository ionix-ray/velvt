//! Home — the only route. Composes the full horizontal-scroll page.
//! All navigation state lives here; child components are purely reactive.

use crate::Site;
use crate::components::{
    about_aggregated_panel::AboutAggregatedPanel, cases_panel::CasesPanel, cta_panel::CtaPanel,
    footer_panel::FooterPanel, hero_panel::HeroPanel, loader::Loader, mobile_nav::MobileNav,
    process_panel::ProcessPanel, scroll_progress::ScrollProgress,
    section_dots::SectionDots, social_strip::SocialStrip, stacked_nav::StackedNav,
    studio_panel::StudioPanel, topbar::TopBar,
};
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Panel labels — displayed in the spindle navigator.
/// The 7th panel is the footer; SocialStrip hides on it.
const PANEL_LABELS: &[&str] = &[
    "HOME",
    "ABOUT",
    "IDEOLOGY",
    "EXPERIENCE",
    "ACHIVEMENTS",
    "CONTACT",
    "FOOTER",
];

/// URL-hash anchors, one per panel, in render order. Mirrors each panel
/// section's own `id` attribute so `#showcase` etc. is shareable/bookmarkable.
const PANEL_ANCHORS: &[&str] = &[
    "home", "about", "ideology", "experience", "achivements", "contact", "footer",
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

/// Panel index implied by the window's vertical scroll position.
fn scroll_sync_index(scroll_top: f64, client_height: f64, len: usize) -> usize {
    let idx = (scroll_top / client_height).round() as usize;
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

/// Smooth-scroll to the panel at `idx` and sync the URL hash.
fn scroll_to_panel(idx: usize) {
    set_url_anchor(idx);
    if let Some(win) = web_sys::window() {
        if let Some(doc) = win.document() {
            let id = anchor_for(idx);
            if let Some(panel) = doc.get_element_by_id(id) {
                let rect = panel.get_bounding_client_rect();
                let top = rect.top() + win.scroll_y().unwrap_or(0.0);
                let opts = web_sys::ScrollToOptions::new();
                opts.set_top(top);
                opts.set_behavior(web_sys::ScrollBehavior::Smooth);
                win.scroll_to_with_scroll_to_options(&opts);
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

    // Window vertical scroll listener to update current_panel
    {
        let mut scroll_listener = use_signal(|| None::<wasm_bindgen::closure::Closure<dyn FnMut()>>);

        use_effect(move || {
            if scroll_listener.peek().is_none() {
                if let Some(win) = web_sys::window() {
                    let win_clone = win.clone();
                    let scroll = wasm_bindgen::closure::Closure::<dyn FnMut()>::new(move || {
                        // Guard: only update panel state when we are on the home route.
                        // Without this check, the listener fires on /achivements/:slug pages
                        // and races with window.scrollTo(0,0) causing a jump to the footer.
                        let on_home = win_clone
                            .location()
                            .pathname()
                            .map(|p| p == "/" || p.is_empty())
                            .unwrap_or(false);
                        if !on_home {
                            return;
                        }

                        let scroll_top = win_clone.scroll_y().unwrap_or(0.0);
                        let client_height = win_clone
                            .inner_height()
                            .ok()
                            .and_then(|h| h.as_f64())
                            .unwrap_or(800.0);
                        let scroll_height = win_clone
                            .document()
                            .and_then(|d| d.document_element())
                            .map(|e| e.scroll_height() as f64)
                            .unwrap_or(0.0);

                        let mut idx = scroll_sync_index(
                            scroll_top,
                            client_height,
                            PANEL_LABELS.len(),
                        );

                        // If we are at the very bottom, force the last panel (footer)
                        if scroll_height > 0.0
                            && scroll_top + client_height >= scroll_height - 150.0
                        {
                            idx = PANEL_LABELS.len().saturating_sub(1);
                        }
                        if idx != *current_panel.peek() {
                            current_panel.set(idx);
                            set_url_anchor(idx);
                        }
                    });

                    let scroll_js = scroll.as_ref().unchecked_ref();
                    let _ = win.add_event_listener_with_callback("scroll", scroll_js);
                    *scroll_listener.write() = Some(scroll);
                }
            }
        });

        use_drop(move || {
            #[cfg(target_arch = "wasm32")]
            if let Some(win) = web_sys::window() {
                if let Some(scroll) = scroll_listener.write().take() {
                    let _ = win.remove_event_listener_with_callback(
                        "scroll",
                        scroll.as_ref().unchecked_ref(),
                    );
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

        // Next and Previous hints removed as per user request

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
        assert_eq!(anchor_for(3), "experience");
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
