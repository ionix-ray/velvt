//! CTA panel — final call to action before the footer. Includes inquiry form.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CtaPanel(site: Site) -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    let mut error = use_signal(|| String::new());

    let email_general = site.contact.email_general.clone();
    let email_press = site.contact.email_press.clone();
    let email_general_mailto = email_general.clone();
    let email_general_display = email_general.clone();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        error.set(String::new());

        let n = name.read().trim().to_string();
        let e = email.read().trim().to_string();
        let m = message.read().trim().to_string();

        if n.is_empty() {
            error.set("Please enter your name.".to_string());
            return;
        }
        if e.is_empty() || !e.contains('@') || !e.contains('.') {
            error.set("Please enter a valid email address.".to_string());
            return;
        }
        if m.is_empty() {
            error.set("Please enter a message.".to_string());
            return;
        }

        let subject = format!("Inquiry from {}", n);
        let body = format!("{m}\n\nFrom: {n} <{e}>", m = m, n = n, e = e);
        let mailto = format!(
            "mailto:{}?subject={}&body={}",
            email_general,
            encode_mailto(&subject),
            encode_mailto(&body)
        );

        if let Some(win) = web_sys::window() {
            let _ = win.location().set_href(&mailto);
        }
        submitted.set(true);
    };

    rsx! {
        section { class: "v-panel", id: "contact",
            div { class: "v-section",
                div { class: "v-container",
                    if *submitted.read() {
                        div { class: "v-cta__inner v-reveal",
                            span { class: "v-eyebrow", "Thank You" }
                            h2 { class: "v-display-2", "Your inquiry is on its way." }
                            p { class: "v-cta__body",
                                "We’ll be in touch within 24 hours. If your email client didn’t open, please reach us directly at "
                                a {
                                    href: "mailto:{email_general_mailto}",
                                    class: "v-cta__contact-email",
                                    "{email_general_mailto}"
                                }
                                "."
                            }
                        }
                    } else {
                        div { class: "v-cta__inner v-reveal",
                            span { class: "v-eyebrow", "Get Started" }
                            h2 { class: "v-display-2", "{site.cta.title}" }
                            p { class: "v-cta__body", "{site.cta.body}" }

                            form { class: "v-contact-form", onsubmit: on_submit,
                                div { class: "v-form-row",
                                    label { "Name" }
                                    input {
                                        r#type: "text",
                                        placeholder: "Your name",
                                        value: "{name}",
                                        oninput: move |evt| name.set(evt.value()),
                                    }
                                }
                                div { class: "v-form-row",
                                    label { "Email" }
                                    input {
                                        r#type: "email",
                                        placeholder: "your@email.com",
                                        value: "{email}",
                                        oninput: move |evt| email.set(evt.value()),
                                    }
                                }
                                div { class: "v-form-row",
                                    label { "Message" }
                                    textarea {
                                        placeholder: "Tell us about your vision…",
                                        rows: 4,
                                        value: "{message}",
                                        oninput: move |evt| message.set(evt.value()),
                                    }
                                }
                                if !error.read().is_empty() {
                                    div { class: "v-form-error", "{error}" }
                                }
                                div { class: "v-btn-group",
                                    button { class: "v-btn v-btn--primary", r#type: "submit",
                                        span { "Send Inquiry" }
                                        span { class: "v-btn__arrow", "→" }
                                    }
                                    a {
                                        class: "v-btn v-btn--outline",
                                        href: "mailto:{email_press}",
                                        span { "Press Inquiries" }
                                    }
                                }
                            }

                            div { class: "v-cta__contact-detail",
                                span { class: "v-cta__contact-cta", "{site.contact.cta}" }
                                a {
                                    href: "mailto:{email_general_display}",
                                    class: "v-cta__contact-email",
                                    "{email_general_display}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Minimal percent-encoding for mailto subject/body — spaces → %20, newlines → %0A.
fn encode_mailto(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '\n' => "%0A".to_string(),
            '&' => "%26".to_string(),
            '?' => "%3F".to_string(),
            '=' => "%3D".to_string(),
            '%' => "%25".to_string(),
            c => c.to_string(),
        })
        .collect()
}
