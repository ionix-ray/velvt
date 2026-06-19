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

/// Smooth-scroll `.v-panels` to the panel at `idx`.
fn scroll_to_panel(idx: usize) {
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
                                let idx = match ev.key().as_str() {
                                    "ArrowRight" | "ArrowDown" => {
                                        if cur < len - 1 {
                                            cur + 1
                                        } else {
                                            return;
                                        }
                                    }
                                    "ArrowLeft" | "ArrowUp" => {
                                        if cur > 0 {
                                            cur - 1
                                        } else {
                                            return;
                                        }
                                    }
                                    _ => return,
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
                                let dy = ev.delta_y();
                                if dy.abs() < 8.0 {
                                    return;
                                }
                                let cur = *current_panel.read();
                                let len = PANEL_LABELS.len();
                                let idx = if dy > 0.0 {
                                    if cur < len - 1 {
                                        cur + 1
                                    } else {
                                        return;
                                    }
                                } else if cur > 0 {
                                    cur - 1
                                } else {
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
                            let idx = (panels.scroll_left() as f64 / panels.client_width() as f64)
                                .round() as usize;
                            let capped = idx.min(PANEL_LABELS.len() - 1);
                            if capped != *current_panel.read() {
                                current_panel.set(capped);
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
    let progress = if panel_count > 1 {
        (*current_panel.read() as f64) / ((panel_count - 1) as f64)
    } else {
        0.0
    };

    // Remove unused hint_hidden variable
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
