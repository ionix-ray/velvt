# Velvt — Event, Celebrity & Influencer Agency

Content source for the site. Edit the TOML inside each section below —
headings are just for human navigation; each fenced ```toml``` block maps
straight onto the matching field of the `Site` struct in `velvet-ui/src/config.rs`.

## Brand

```toml
[brand]
name      = "Velvt"
tagline   = "We shape stories. You make history."
short     = "Velvt"
copyright = "© 2026 Velvt. All rights reserved."
```

## Meta

```toml
[meta]
title       = "Velvt — We Shape Stories. You Make History."
description = "PR, event, celebrity, branding, advertising, digital and photography & videography agency. From powerful connections to unforgettable moments, we build brands that inspire, influence and leave a lasting impact."
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
badge     = "Building Brands. Creating Impact."
headline1 = "We shape"
headline2 = "stories."
headline3 = "You make history."
sub       = "From powerful connections to unforgettable moments, we build brands that inspire, influence and leave a lasting impact. Your vision. Our strategy. Unmatched impact."
cta_primary   = "Start Your Story"
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
title = "Our Services."
sub   = "End-to-end communications, talent and venue capabilities under one roof."

[[services.items]]
num    = "01"
title  = "PR Management"
body   = "Strategic communication that builds reputation, strengthens relationships and amplifies your voice."
[[services.items]]
num    = "02"
title  = "Event Management"
body   = "Creating memorable experiences that engage audiences and leave a lasting impression."
[[services.items]]
num    = "03"
title  = "Celebrity Management"
body   = "Building powerful associations and managing relationships that elevate your brand."
[[services.items]]
num    = "04"
title  = "Advertising"
body   = "Creative campaigns that capture attention, tell your story and drive results."
[[services.items]]
num    = "05"
title  = "Digital Marketing"
body   = "Data-driven strategies to grow your online presence and connect with the right audience."
[[services.items]]
num    = "06"
title  = "Branding"
body   = "Crafting unique identities that define your brand and set you apart in the competitive world."
[[services.items]]
num    = "07"
title  = "Photography & Videography"
body   = "Capturing moments that tell your story through powerful visuals."
[[services.items]]
num    = "08"
title  = "Venue"
body   = "Our colocation space for podcasts, with a full suite for hosting small meetings, co-working sessions and shooting short films."
```

## Story

```toml
[story]
title = "Where Strategy Meets Spectacle"
sub   = "A results-driven agency specialising in PR, events, celebrity and influencer work, brand identity and content production. Backed by industry veterans and creative innovators, we turn moments into movements."

[[story.items]]
year  = "01"
title = "Precision Planning"
body  = "End-to-end orchestration of every detail, timeline and stakeholder for flawless, on-budget delivery."
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
footer_label = "Velvt Studio"
slug = "technova-full-funnel-growth"

[[cases.items]]
client = "Luxe Beauty"
metric = "+185%"
desc   = "Premium beauty brand broke sales records with our celebrity-driven launch activation."
tags   = ["Beauty", "Celebrity", "Launch"]
logo_image = "/assets/images/logo.jpg"
button_link = "#"
footer_label = "Velvt Studio"
slug = "luxe-beauty-celebrity-launch"

[[cases.items]]
client = "GreenFuture"
metric = "+312%"
desc   = "Eco-tech startup dominated their market with our immersive brand storytelling campaign."
tags   = ["Tech", "Sustainability", "Immersive"]
logo_image = "/assets/images/logo.jpg"
button_link = "#"
footer_label = "Velvt Studio"
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
tag   = "Venue"
title = "Velvt Venue"
body  = "Colocation space built for podcasts, co-working, intimate meetings and small film shoots."
[[studio.items]]
tag   = "Celebrity"
title = "Celebrity & Influencer Management"
body  = "End-to-end celebrity and influencer partnerships — talent discovery, deal-making and on-brand activations that turn famous faces into measurable lift for your brand."
[[studio.items]]
tag   = "Creators"
title = "Creator Seeding"
body  = "Authentic product seeding to a vetted creator network. We match the right makers with your story so the conversation starts where your audience already lives."
```

## CTA

```toml
[cta]
title   = "Ready To Make History?"
body    = "Let's shape a story your audience won't forget — strategy, talent, production and venue under one roof."
btn_primary   = "Launch Your Campaign"
btn_secondary = "Book a Consultation"
btn_ghost     = "View Our Work"
```

## Contact

```toml
[contact]
email_general = "connect@velvt.live"
email_press   = "connect@velvt.live"
ateliers      = ["Plot No: 756, 3rd Floor, Saheed Nagar, BBSR, Odisha- 751007"]
cta           = "Tell us the story you want to tell."
```

## Client Banner

```toml
[client_banner]
title = "Trusted by Industry Leaders"
```

## Founder

```toml
[founder]
name     = "Arpita"
eyebrow  = "Founder · Star Gazer"
bio      = "An Odishi dancer with a quiet observer's eye, Arpita reads a room before she enters it. Her soft spot is the stardom her work moves around every day — and the warmth of hospitality she insists every guest leaves with. Velvt is built on both: an artist's attention to detail, and a host's instinct for people."
# Drop the founder portrait at velvet-ui/assets/images/arpita.jpg and
# point `photo` at it. Empty `photo` renders the monogram tile.
photo    = ""
monogram = "A"

[[founder.handles]]
icon  = "instagram"
label = "@thearpitaparhi_official"
href  = "https://instagram.com/thearpitaparhi_official"
```

## Footer

```toml
[footer]
brand_desc = "From powerful connections to unforgettable moments, we build brands that inspire, influence and leave a lasting impact. Building brands. Creating impact."

[[footer.columns]]
title = "Services"
links = [
  { label = "PR Management", href = "#" },
  { label = "Event Management", href = "#" },
  { label = "Celebrity Management", href = "#" },
  { label = "Advertising", href = "#" },
  { label = "Digital Marketing", href = "#" },
  { label = "Branding", href = "#" },
  { label = "Photography & Videography", href = "#" },
  { label = "Venue", href = "#" },
]
[[footer.columns]]
title = "Company"
links = [
  { label = "About Us", href = "#about" },
  { label = "Our Work", href = "#showcase" },
  { label = "Case Studies", href = "/cases" },
  { label = "Contact", href = "#contact" },
]
[[footer.columns]]
title = "Contact"
links = [
  { label = "connect@velvt.live", href = "mailto:connect@velvt.live" },
  { label = "+91 93484 04970", href = "tel:+919348404970" },
  { label = "www.velvt.live", href = "https://www.velvt.live" },
  { label = "Plot No: 756, 3rd Floor, Saheed Nagar, BBSR, Odisha- 751007", href = "#" },
]
[[footer.socials]]
label = "twitter"
href  = "https://x.com/velvt"
[[footer.socials]]
label = "linkedin"
href  = "https://linkedin.com/company/velvt"
[[footer.socials]]
label = "instagram"
href  = "https://instagram.com/velvt"
```
