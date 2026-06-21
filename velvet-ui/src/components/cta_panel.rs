//! CTA panel — final call to action before the footer. Includes inquiry form.

use crate::Site;
use dioxus::prelude::*;

#[component]
pub fn CtaPanel(site: Site) -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut submitted = use_signal(|| false);
    let mut error = use_signal(String::new);

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

        match validate_inquiry(&n, &e, &m) {
            Err(msg) => error.set(msg.to_string()),
            Ok(()) => {
                let mailto = build_mailto(&email_general, &n, &e, &m);
                if let Some(win) = web_sys::window() {
                    let _ = win.location().set_href(&mailto);
                }
                submitted.set(true);
            }
        }
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

/// Validate the inquiry form fields. Returns the first failing field's
/// user-facing error message.
fn validate_inquiry(name: &str, email: &str, message: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Please enter your name.");
    }
    if email.is_empty() || !email.contains('@') || !email.contains('.') {
        return Err("Please enter a valid email address.");
    }
    if message.is_empty() {
        return Err("Please enter a message.");
    }
    Ok(())
}

/// Build a `mailto:` URL carrying the inquiry as subject/body.
fn build_mailto(to: &str, name: &str, email: &str, message: &str) -> String {
    let subject = format!("Inquiry from {name}");
    let body = format!("{message}\n\nFrom: {name} <{email}>");
    format!(
        "mailto:{to}?subject={}&body={}",
        encode_mailto(&subject),
        encode_mailto(&body)
    )
}

/// Minimal percent-encoding for mailto subject/body — spaces → %20, newlines → %0A.
fn encode_mailto(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '\n' => "%0A".to_string(),
            '\r' => "%0D".to_string(),
            '&' => "%26".to_string(),
            '?' => "%3F".to_string(),
            '=' => "%3D".to_string(),
            '%' => "%25".to_string(),
            c => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_inquiry_requires_name() {
        assert_eq!(
            validate_inquiry("", "a@b.com", "hi"),
            Err("Please enter your name.")
        );
    }

    #[test]
    fn validate_inquiry_requires_email_with_at_and_dot() {
        assert_eq!(
            validate_inquiry("Sam", "", "hi"),
            Err("Please enter a valid email address.")
        );
        assert_eq!(
            validate_inquiry("Sam", "no-at-sign.com", "hi"),
            Err("Please enter a valid email address.")
        );
        assert_eq!(
            validate_inquiry("Sam", "no-dot@com", "hi"),
            Err("Please enter a valid email address.")
        );
    }

    #[test]
    fn validate_inquiry_requires_message() {
        assert_eq!(
            validate_inquiry("Sam", "a@b.com", ""),
            Err("Please enter a message.")
        );
    }

    #[test]
    fn validate_inquiry_passes_with_all_fields_present() {
        assert_eq!(validate_inquiry("Sam", "a@b.com", "hi"), Ok(()));
    }

    #[test]
    fn build_mailto_embeds_subject_and_body() {
        let url = build_mailto("hello@vaelvet.com", "Sam", "sam@x.com", "Let's talk");
        assert!(url.starts_with("mailto:hello@vaelvet.com?subject=Inquiry%20from%20Sam&body="));
        assert!(url.contains("Let's%20talk"));
        assert!(url.contains("From:%20Sam%20<sam@x.com>"));
    }

    #[test]
    fn encode_mailto_escapes_reserved_characters() {
        assert_eq!(encode_mailto("a b"), "a%20b");
        assert_eq!(encode_mailto("a\nb"), "a%0Ab");
        assert_eq!(encode_mailto("a&b"), "a%26b");
        assert_eq!(encode_mailto("a?b"), "a%3Fb");
        assert_eq!(encode_mailto("a=b"), "a%3Db");
        assert_eq!(encode_mailto("a%b"), "a%25b");
        assert_eq!(encode_mailto("a\rb"), "a%0Db");
    }

    #[test]
    fn encode_mailto_leaves_plain_text_untouched() {
        assert_eq!(encode_mailto("hello"), "hello");
        assert_eq!(encode_mailto(""), "");
    }

    #[test]
    fn encode_mailto_composes_multiple_reserved_characters() {
        assert_eq!(
            encode_mailto("Hi there?\nFrom: a@b.com"),
            "Hi%20there%3F%0AFrom:%20a@b.com"
        );
    }
}
