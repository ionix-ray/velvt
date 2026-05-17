use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SeoProps {
    #[props(default = "Velvet — Premium PR Agency".to_string())]
    pub title: String,
    #[props(default = "Premium public relations agency specializing in media relations, crisis communications, talent management, and event production.".to_string())]
    pub description: String,
    #[props(default = "/".to_string())]
    pub path: String,
    #[props(default)]
    pub json_ld: Option<String>,
}

#[component]
pub fn SeoMeta(props: SeoProps) -> Element {
    let image_url = "https://velvet.pr/og-image.jpg";
    let site_name = "Velvet PR Agency";
    let twitter_handle = "@velvetpr";

    rsx! {
        document::Title { "{props.title}" }
        document::Meta { name: "description", content: "{props.description}" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Link { rel: "canonical", href: "https://velvet.pr{props.path}" }

        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "{site_name}" }
        document::Meta { property: "og:title", content: "{props.title}" }
        document::Meta { property: "og:description", content: "{props.description}" }
        document::Meta { property: "og:url", content: "https://velvet.pr{props.path}" }
        document::Meta { property: "og:image", content: "{image_url}" }

        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:site", content: "{twitter_handle}" }
        document::Meta { name: "twitter:title", content: "{props.title}" }
        document::Meta { name: "twitter:description", content: "{props.description}" }
        document::Meta { name: "twitter:image", content: "{image_url}" }

        if let Some(json_ld) = &props.json_ld {
            script {
                r#type: "application/ld+json",
                dangerous_inner_html: "{json_ld}",
            }
        }
    }
}

pub fn organization_schema() -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": "Velvet PR Agency",
        "url": "https://velvet.pr",
        "logo": "https://velvet.pr/logo.png",
        "sameAs": [
            "https://twitter.com/velvetpr",
            "https://linkedin.com/company/velvet-pr",
            "https://instagram.com/velvetpr"
        ],
        "contactPoint": {
            "@type": "ContactPoint",
            "telephone": "+1-212-555-0100",
            "contactType": "customer service",
            "email": "hello@velvet.pr"
        }
    })
    .to_string()
}

pub fn service_schema() -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Service",
        "serviceType": "Public Relations",
        "provider": {
            "@type": "Organization",
            "name": "Velvet PR Agency"
        },
        "areaServed": "Global",
        "hasOfferCatalog": {
            "@type": "OfferCatalog",
            "name": "PR Services",
            "itemListElement": [
                {"@type": "Offer", "itemOffered": {"@type": "Service", "name": "Media Relations"}},
                {"@type": "Offer", "itemOffered": {"@type": "Service", "name": "Crisis Communications"}},
                {"@type": "Offer", "itemOffered": {"@type": "Service", "name": "Talent Management"}},
                {"@type": "Offer", "itemOffered": {"@type": "Service", "name": "Event Production"}}
            ]
        }
    }).to_string()
}

pub fn faq_schema() -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": [
            {
                "@type": "Question",
                "name": "What services does Velvet PR offer?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "Velvet PR offers public relations, media relations, crisis communications, talent management, and event production services."
                }
            },
            {
                "@type": "Question",
                "name": "How do I get started with Velvet PR?",
                "acceptedAnswer": {
                    "@type": "Answer",
                    "text": "Contact us through our website to schedule a consultation. We'll discuss your goals and create a tailored communications strategy."
                }
            }
        ]
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn organization_schema_is_valid_json() {
        let schema = organization_schema();
        let result: Result<serde_json::Value, _> = serde_json::from_str(&schema);
        assert!(result.is_ok());
    }

    #[test]
    fn service_schema_is_valid_json() {
        let schema = service_schema();
        let result: Result<serde_json::Value, _> = serde_json::from_str(&schema);
        assert!(result.is_ok());
    }

    #[test]
    fn faq_schema_is_valid_json() {
        let schema = faq_schema();
        let result: Result<serde_json::Value, _> = serde_json::from_str(&schema);
        assert!(result.is_ok());
    }

    #[test]
    fn organization_schema_has_correct_type() {
        let schema = organization_schema();
        assert!(schema.contains("\"Organization\""));
    }

    #[test]
    fn service_schema_has_services() {
        let schema = service_schema();
        assert!(schema.contains("Media Relations"));
        assert!(schema.contains("Crisis Communications"));
    }

    #[test]
    fn faq_schema_has_questions() {
        let schema = faq_schema();
        assert!(schema.contains("Question"));
        assert!(schema.contains("Answer"));
    }
}
