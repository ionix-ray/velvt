//! Home — the only route. Composes the full horizontal-scroll page.

use crate::Site;
use crate::components::{
    analytics_panel::AnalyticsPanel, cases_panel::CasesPanel, client_banner::ClientBanner,
    cta_panel::CtaPanel, footer_panel::FooterPanel, hero_panel::HeroPanel, loader::Loader,
    next_hint::NextHint, process_panel::ProcessPanel, scroll_progress::ScrollProgress,
    section_dots::SectionDots, services_panel::ServicesPanel, stacked_nav::StackedNav,
    story_panel::StoryPanel, studio_panel::StudioPanel, topbar::TopBar, work_with_us::WorkWithUs,
};
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

const PANEL_LABELS: &[&str] = &[
    "Home",
    "About",
    "Stories",
    "Banner",
    "Showcase",
    "Portfolio",
    "Process",
    "Cases",
    "Inquiry",
    "CTA",
    "Footer",
];

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

    // Apply theme to document
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

    // Hide loader after mount
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

    // Close menu on navigation + scroll to target panel
    let on_navigate = move |idx: usize| {
        current_panel.set(idx);
        menu_open.set(false);
        scroll_to_panel(idx);
    };

    // Keyboard arrow navigation + scroll sync
    {
        use_effect(move || {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    let doc_scroll = doc.clone();

                    let kbd =
                        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                            move |ev: web_sys::KeyboardEvent| {
                                let idx = match ev.key().as_str() {
                                    "ArrowRight" | "ArrowDown" => {
                                        let cur = *current_panel.read();
                                        if cur < PANEL_LABELS.len() - 1 {
                                            cur + 1
                                        } else {
                                            return;
                                        }
                                    }
                                    "ArrowLeft" | "ArrowUp" => {
                                        let cur = *current_panel.read();
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

                    let wheel =
                        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::WheelEvent)>::new(
                            move |ev: web_sys::WheelEvent| {
                                let dy = ev.delta_y();
                                if dy.abs() < 8.0 {
                                    return;
                                }
                                let cur = *current_panel.read();
                                let idx = if dy > 0.0 {
                                    if cur < PANEL_LABELS.len() - 1 {
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
    let hint_hidden = *current_panel.read() >= panel_count - 1;

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

        NextHint { hidden: hint_hidden }

        div { class: "v-panels",
            HeroPanel { site: site.clone() }
            StoryPanel { site: site.clone() }
            AnalyticsPanel { site: site.clone() }
            ClientBanner { site: site.clone() }
            StudioPanel { site: site.clone() }
            ServicesPanel { site: site.clone() }
            ProcessPanel { site: site.clone() }
            CasesPanel { site: site.clone() }
            WorkWithUs { site: site.clone() }
            CtaPanel { site: site.clone() }
            FooterPanel { site }
        }
    }
}
