use crate::components::contact::Contact as ContactSection;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navbar::Navbar;
use crate::components::podcast::Podcast as PodcastSection;
use crate::components::portfolio::Portfolio as PortfolioSection;
use crate::components::services::Services as ServicesSection;
use crate::components::shared::{
    SeoMeta, faq_schema, organization_schema, service_schema, use_scroll_position,
};
use crate::components::talent::Talent as TalentSection;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let scrolled = use_scroll_position();
    let json_ld = format!(
        "{}{}{}",
        organization_schema(),
        service_schema(),
        faq_schema()
    );

    rsx! {
        SeoMeta {
            title: "Velvet — Premium PR Agency",
            description: "Premium public relations agency specializing in media relations, crisis communications, talent management, and event production.",
            path: "/",
            json_ld: json_ld,
        }
        div { class: "page-wrapper" }
        Navbar { scrolled: scrolled() }
        main {
            Hero {}
            ServicesSection {}
            TalentSection {}
            PortfolioSection {}
            PodcastSection {}
            ContactSection {}
        }
        Footer {}
    }
}

#[component]
pub fn Services() -> Element {
    let scrolled = use_scroll_position();

    rsx! {
        SeoMeta {
            title: "Services — Velvet PR Agency",
            description: "Strategic PR, media relations, and crisis communications tailored to your narrative.",
            path: "/services",
            json_ld: service_schema(),
        }
        div { class: "page-wrapper page-enter" }
        Navbar { scrolled: scrolled() }
        main {
            ServicesSection {}
        }
        Footer {}
    }
}

#[component]
pub fn Talent() -> Element {
    let scrolled = use_scroll_position();

    rsx! {
        SeoMeta {
            title: "Talent & Events — Velvet PR Agency",
            description: "World-class professionals who amplify your story through talent management and event production.",
            path: "/talent",
        }
        div { class: "page-wrapper page-enter" }
        Navbar { scrolled: scrolled() }
        main {
            TalentSection {}
        }
        Footer {}
    }
}

#[component]
pub fn Portfolio() -> Element {
    let scrolled = use_scroll_position();

    rsx! {
        SeoMeta {
            title: "Portfolio — Velvet PR Agency",
            description: "Real results for real clients. Case studies in PR, crisis management, and executive visibility.",
            path: "/portfolio",
        }
        div { class: "page-wrapper page-enter" }
        Navbar { scrolled: scrolled() }
        main {
            PortfolioSection {}
        }
        Footer {}
    }
}

#[component]
pub fn Podcast() -> Element {
    let scrolled = use_scroll_position();

    rsx! {
        SeoMeta {
            title: "Podcast — Velvet PR Agency",
            description: "The Velvet Podcast — Coming Soon. Conversations with the voices shaping culture, media, and influence.",
            path: "/podcast",
        }
        div { class: "page-wrapper page-enter" }
        Navbar { scrolled: scrolled() }
        main {
            PodcastSection {}
        }
        Footer {}
    }
}

#[component]
pub fn Contact() -> Element {
    let scrolled = use_scroll_position();

    rsx! {
        SeoMeta {
            title: "Contact — Velvet PR Agency",
            description: "Get in touch with Velvet PR Agency. Let's start a conversation about your narrative.",
            path: "/contact",
        }
        div { class: "page-wrapper page-enter" }
        Navbar { scrolled: scrolled() }
        main {
            ContactSection {}
        }
        Footer {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_renders_heading() {
        let mut dom = VirtualDom::new(Home);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Velvet"));
    }

    #[test]
    fn services_renders_heading() {
        let mut dom = VirtualDom::new(Services);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Services"));
    }

    #[test]
    fn talent_renders_heading() {
        let mut dom = VirtualDom::new(Talent);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Talent"));
    }

    #[test]
    fn portfolio_renders_heading() {
        let mut dom = VirtualDom::new(Portfolio);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Portfolio"));
    }

    #[test]
    fn podcast_renders_heading() {
        let mut dom = VirtualDom::new(Podcast);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Podcast"));
    }

    #[test]
    fn contact_renders_heading() {
        let mut dom = VirtualDom::new(Contact);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Contact"));
    }
}
