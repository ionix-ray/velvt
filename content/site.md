# Velvt — Event, Celebrity & Influencer Agency

Content source for the site. Edit the TOML inside each section below —
headings are just for human navigation; each fenced ```toml``` block maps
straight onto the matching field of the `Site` struct in `velvet-ui/src/config.rs`.

## Brand

```toml
[brand]
name      = "Velvt"
tagline   = "elevate your Presence."
short     = "Velvt"
copyright = "© 2026 Velvt. All rights reserved."
```

## Meta

```toml
[meta]
title       = "Velvt — Elevate Your Brand Experiences"
description = "Leading event management, celebrity talent booking, influencer marketing & custom gifting agency. Transforming brands through immersive experiences from concept to execution."
og_image    = "/assets/images/logo.jpg"
theme_color = "#5A0A0F"
twitter     = "@velvt"
```

## Nav

```toml
[[nav]]
label = "Home"
href = "#home"
[[nav]]
label = "About"
href = "#about"
[[nav]]
label = "Stories"
href = "#stories"
[[nav]]
label = "Showcase"
href = "#showcase"
[[nav]]
label = "Portfolio"
href = "#portfolio"
[[nav]]
label = "Contact"
href = "#contact"
```

## Hero

```toml
[hero]
badge     = "Premium Experiences Since 2026"
headline1 = "Elevate Your Brand Through"
headline2 = "Unforgettable"
headline3 = "Experiences"
sub       = "We engineer immersive events, strategic celebrity partnerships, and data-driven influencer campaigns that captivate audiences and deliver measurable ROI. From concept to execution, your vision, flawlessly realized."
cta_primary   = "Start Your Campaign"
cta_primary_href = "#contact"
cta_secondary = "Explore Our Work"
cta_secondary_href = "#showcase"

[[hero.stats]]
value = "350+"
label = "High-Impact Events"
[[hero.stats]]
value = "92%"
label = "Client Retention Rate"
[[hero.stats]]
value = "4.2x"
label = "Average Campaign ROI"
[[hero.stats]]
value = "50+"
label = "Celebrity Partnerships"
```

## Services

```toml
[services]
title = "Specialized Capabilities. Scalable Solutions."
sub   = "End-to-end experiential marketing powered by creative strategy, talent connections, and operational precision."

[[services.items]]
num    = "01"
title  = "Event Management"
body   = "End-to-end production from concept to crowd control. Venue sourcing, technical staging, compliance, and seamless execution for corporate, public, and private activations."
[[services.items]]
num    = "02"
title  = "Celebrity Management"
body   = "Strategic talent booking, contract negotiation, itinerary coordination, and brand-aligned appearances that amplify visibility and credibility."
[[services.items]]
num    = "03"
title  = "Custom Branding"
body   = "Identity-driven spatial design, tactile activations, and immersive brand environments that turn passive audiences into active advocates."
[[services.items]]
num    = "04"
title  = "Influencer Management"
body   = "Data-backed creator matchmaking, campaign structuring, content governance, and performance tracking for authentic, high-converting reach."
[[services.items]]
num    = "05"
title  = "Seasonal Events"
body   = "Culturally resonant celebrations engineered for engagement, community impact, and strategic brand integration."
[[services.items]]
num    = "06"
title  = "Personalized Gifts"
body   = "Thoughtful, brand-enhanced gifting solutions. Scalable production, premium customization, and timely delivery that strengthen loyalty and recall."
[[services.items]]
num    = "07"
title  = "Gift Card Management"
body   = "Full-lifecycle digital and physical gift solutions. Secure code generation, dynamic loading, redemption tracking, fraud prevention, and actionable analytics."
```

## Story

```toml
[story]
title = "Where Strategy Meets Spectacle"
sub   = "A results-driven experiential agency specializing in end-to-end event production, talent curation, and brand activation. Backed by industry veterans and creative innovators, we turn moments into movements."

[[story.items]]
year  = "01"
title = "Precision Planning"
body  = "End-to-end orchestration of every detail, timeline, and stakeholder for flawless, on-budget delivery."
[[story.items]]
year  = "02"
title = "Talent Curation"
body  = "Strategic selection of celebrities and influencers whose presence authentically amplifies your brand."
[[story.items]]
year  = "03"
title = "Brand Alignment"
body  = "Every activation reinforces your core identity — from spatial design to talent partnerships."
[[story.items]]
year  = "04"
title = "Seamless Execution"
body  = "Backed by creative innovators, we deliver with precision, compliance, and relentless ROI focus."
```

## Analytics

```toml
[analytics]
title = "Proven Impact. Measurable Growth."
sub   = "Every milestone reflects a brand elevated, an audience engaged, and a vision realized."

[[analytics.stats]]
value = "350+"
label = "High-Impact Events"
change = "Executed"
[[analytics.stats]]
value = "92%"
label = "Client Retention"
change = "Repeat Engagement"
[[analytics.stats]]
value = "4.2x"
label = "Avg. Campaign ROI"
change = "Across Verticals"
[[analytics.stats]]
value = "50+"
label = "Celebrity Partnerships"
change = "Deployed"
```

## Process

```toml
[process]
title = "Our Proven Workflow."
sub   = "Every successful campaign starts with deep strategy and ends with measurable growth."

[[process.steps]]
num   = "01"
title = "Discovery & Strategy"
body  = "Deep-dive into your brand, audience, and goals to architect the perfect campaign."
[[process.steps]]
num   = "02"
title = "Talent & Partner Curation"
body  = "Strategic selection of celebrities, influencers, vendors, and venues aligned with your vision."
[[process.steps]]
num   = "03"
title = "Creative Production"
body  = "World-class visuals, immersive staging, and compelling narratives that captivate audiences."
[[process.steps]]
num   = "04"
title = "Execution & Management"
body  = "Flawless on-ground delivery with real-time adaptability and compliance at every turn."
[[process.steps]]
num   = "05"
title = "Performance Analysis"
body  = "Data-backed measurement, ROI reporting, and insights to optimize your next activation."
```

## Cases

```toml
[cases]
title = "Brands That Trusted Our Vision."
sub   = "We help startups, enterprises, and cultural pioneers build unforgettable market presence."

[[cases.items]]
client = "TechNova"
metric = "+240%"
desc   = "SaaS platform achieved 3x lead generation through our full-funnel experiential strategy."
tags   = ["B2B", "Experiential", "Lead Gen"]
logo_image = "/assets/images/logo.jpg"
button_link = "#"
footer_label = "Developed by 🍐 Community"
slug = "technova-full-funnel-growth"

[[cases.items]]
client = "Luxe Beauty"
metric = "+185%"
desc   = "Premium beauty brand broke sales records with our celebrity-driven launch activation."
tags   = ["Beauty", "Celebrity", "Launch"]
logo_image = "/assets/images/logo.jpg"
button_link = "#"
footer_label = "Developed by Velvt"
slug = "luxe-beauty-celebrity-launch"

[[cases.items]]
client = "GreenFuture"
metric = "+312%"
desc   = "Eco-tech startup dominated their market with our immersive brand storytelling campaign."
tags   = ["Tech", "Sustainability", "Immersive"]
logo_image = "/assets/images/logo.jpg"
button_link = "#"
footer_label = "Developed by 🍐 Community"
slug = "greenfuture-immersive-storytelling"
```

## Studio

```toml
[studio]
title = "Experience Design. Flawlessly Delivered."
sub   = "Where creativity meets logistics. We orchestrate high-stakes events with precision engineering, immersive storytelling, and real-time adaptability."

[[studio.items]]
tag   = "Corporate"
title = "Summit Production"
body  = "Full-spectrum corporate event production for 500+ attendees with multi-stage setup."
[[studio.items]]
tag   = "Celebrity"
title = "Brand Ambassador Launch"
body  = "Orchestrated a high-profile celebrity partnership announcement with global media coverage."
[[studio.items]]
tag   = "Cultural"
title = "Festival Activation"
body  = "Heritage-inspired festival with modern production values drawing 10,000+ attendees."
[[studio.items]]
tag   = "Product"
title = "Interactive Launch"
body  = "AR-powered product launch event blending digital immersion with live attendee engagement."
[[studio.items]]
tag   = "Gifting"
title = "Corporate Gifting Suite"
body  = "Scalable personalized gift program for 5,000+ employees across 12 locations globally."
```

## CTA

```toml
[cta]
title   = "Ready To Elevate Your Brand?"
body    = "Let's engineer immersive campaigns that capture attention, engage audiences, and grow your business."
btn_primary   = "Launch Your Campaign"
btn_secondary = "Book a Consultation"
btn_ghost     = "View Our Work"
```

## Contact

```toml
[contact]
email_general = "connect@velvt.live"
email_press   = "connect@velvt.live"
ateliers      = ["3rd floor, Plot No.756, G+2 Storied GA, Rev Plot No. 317, Saheed Nagar, Bhubaneswar, Khorda- 751007, Orissa, India", "Bandra West, Mumbai"]
cta           = "Tell us who you want to become."
```

## Client Banner

```toml
[client_banner]
title = "Trusted by Industry Leaders"
```

## Footer

```toml
[footer]
brand_desc = "A premium experiential agency that turns brands into movements. We engineer immersive events, forge celebrity partnerships, and craft data-driven influencer campaigns that captivate audiences and deliver measurable ROI."

[[footer.columns]]
title = "Services"
links = [
  { label = "Event Management", href = "#" },
  { label = "Celebrity Management", href = "#" },
  { label = "Influencer Campaigns", href = "#" },
  { label = "Custom Branding", href = "#" },
  { label = "Gift Solutions", href = "#" },
]
[[footer.columns]]
title = "Company"
links = [
  { label = "About Us", href = "#about" },
  { label = "Our Work", href = "#showcase" },
  { label = "Careers", href = "#" },
  { label = "Blog", href = "#" },
  { label = "Contact", href = "#contact" },
]
[[footer.columns]]
title = "Contact"
links = [
  { label = "connect@velvt.live", href = "mailto:connect@velvt.live" },
  { label = "+91 (555) 000-0000", href = "tel:+915550000000" },
  { label = "3rd floor, Plot No.756, G+2 Storied GA, Rev Plot No. 317, Saheed Nagar, Bhubaneswar, Khorda- 751007, Orissa, India", href = "#" },
  { label = "Bandra West, Mumbai", href = "#" },
]
[[footer.socials]]
label = "X"
href  = "https://x.com/velvt"
[[footer.socials]]
label = "in"
href  = "https://linkedin.com/company/velvt"
[[footer.socials]]
label = "IG"
href  = "https://instagram.com/velvt"
```
