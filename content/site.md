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
og_image    = "/assets/images/velvet-mark.png"
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
href = "#cases"
[[nav]]
label = "Contact"
href = "#contact"
```

## Hero

```toml
[hero]
logo      = "/assets/images/velvet-mark.webp"
video     = "/assets/images/velvt-animation-optimized.mp4"
logo_svg  = "/assets/images/velvt-logo.svg"
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

## Background

Whole-site ambient field. Two layers: a gradient mesh (`mesh`) and floating
service glyphs (`show_glyphs`). Everything is clamped in `config.rs`, so a
mistyped value here can never flood the page or strobe a visitor.
`intensity` is the master ceiling; `glyph_opacity` is the glyph layer's share
of it. All values are validated by unit tests in `velvet-ui/src/config.rs`.

```toml
[background]
enabled       = true
intensity     = 80    # 0..=100 — master ceiling for every layer
mesh          = true  # drifting crimson spotlights + stage beam
show_glyphs   = true  # floating service icons
glyph_opacity = 60    # glyph share of intensity (0..=100)
glyph_duration = 30   # base drift cycle, seconds; per-glyph `rate` scales it

# The glyphs name the work: photography, venue, talent, parties, creators.
# tone  → crimson | violet | sky | citron
# x, y  → viewport percentage placement
# size  → px (40..420), rate → cycle multiplier, delay → phase offset (s)
# spin  → rotation amplitude in degrees (clamped ±45)

[[background.glyphs]]
icon = "camera"
tone = "crimson"
x = 8
y = 30
size = 250
rate = 1.0
delay = 0.0
spin = 8.0

[[background.glyphs]]
icon = "venue"
tone = "violet"
x = 84
y = 22
size = 215
rate = 1.25
delay = -6.0
spin = -6.0

[[background.glyphs]]
icon = "talent"
tone = "sky"
x = 76
y = 68
size = 235
rate = 0.9
delay = -12.0
spin = 5.0

[[background.glyphs]]
icon = "grand-party"
tone = "citron"
x = 18
y = 74
size = 225
rate = 1.15
delay = -18.0
spin = -8.0

[[background.glyphs]]
icon = "creator"
tone = "violet"
x = 46
y = 12
size = 190
rate = 1.4
delay = -9.0
spin = 7.0

[[background.glyphs]]
icon = "clapper"
tone = "sky"
x = 58
y = 84
size = 205
rate = 1.05
delay = -15.0
spin = -5.0

[[background.glyphs]]
icon = "lens"
tone = "crimson"
x = 68
y = 46
size = 170
rate = 1.6
delay = -3.0
spin = 10.0

[[background.glyphs]]
icon = "spotlight"
tone = "citron"
x = 92
y = 52
size = 180
rate = 0.85
delay = -21.0
spin = -4.0

[[background.glyphs]]
icon = "aperture"
tone = "crimson"
x = 68
y = 40
size = 160
rate = 1.3
delay = -11.0
spin = 12.0
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
[[services.items]]
num    = "09"
title  = "Corporate Gifting"
body   = "End-to-end corporate gifting — custom coupons, gift cards and curated hampers, from design to lifecycle management."
[[services.items]]
num    = "10"
title  = "Video Editing"
body   = "Post-production craft — edits, colour, sound and motion graphics that turn raw footage into stories worth watching."
[[services.items]]
num    = "11"
title  = "Podcast Production"
body   = "Full-stack podcast production — studio, recording, editing and distribution under one roof."
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

## Studio

```toml
[studio]
eyebrow = "Experience"
title = "Experience Design. Flawlessly Delivered."
sub   = "Where creativity meets logistics. We orchestrate high-stakes events with precision engineering, immersive storytelling, and real-time adaptability."

[[studio.items]]
tag   = "Corporate"
title = "Summit Production"
body  = "Full-spectrum corporate event production for 500+ attendees with multi-stage setup."
image = "/assets/images/studio-01-summit.webp"
split_image = "/assets/images/studio-01-summit.webp"
split_tag = "Corporate"
split_stat = "500+ Attendees"
split_cta = "Explore"

[[studio.items]]
tag   = "Celebrity"
title = "Brand Ambassador Launch"
body  = "Orchestrated a high-profile celebrity partnership announcement with global media coverage."
image = "/assets/images/studio-02-ambassador.webp"
split_image = "/assets/images/studio-02-ambassador.webp"
split_tag = "Celebrity"
split_stat = "Global Reach"
split_cta = "Explore"

[[studio.items]]
tag   = "Cultural"
title = "Festival Activation"
body  = "Heritage-inspired festival with modern production values drawing 10,000+ attendees."
image = "/assets/images/studio-03-festival.webp"
split_image = "/assets/images/studio-03-festival.webp"
split_tag = "Cultural"
split_stat = "10K+ Attendees"
split_cta = "Explore"

[[studio.items]]
tag   = "Product"
title = "Interactive Launch"
body  = "AR-powered product launch event blending digital immersion with live attendee engagement."
image = "/assets/images/studio-04-interactive.webp"
split_image = "/assets/images/studio-04-interactive.webp"
split_tag = "Product"
split_stat = "AR Enabled"
split_cta = "Explore"

[[studio.items]]
tag   = "Venue"
title = "Velvt Venue"
body  = "Colocation space built for podcasts, co-working, intimate meetings and small film shoots."
image = "/assets/images/studio-05-venue.webp"
split_image = "/assets/images/studio-05-venue.webp"
split_tag = "Venue"
split_stat = "Multi-Purpose"
split_cta = "Explore"

[[studio.items]]
tag   = "Celebrity"
title = "Celebrity & Influencer Management"
body  = "End-to-end celebrity and influencer partnerships — talent discovery, deal-making and on-brand activations that turn famous faces into measurable lift for your brand."
image = "/assets/images/studio-06-celebrity.webp"
split_image = "/assets/images/studio-06-celebrity.webp"
split_tag = "Celebrity"
split_stat = "End-to-End"
split_cta = "Explore"

[[studio.items]]
tag   = "Creators"
title = "Creator Seeding"
body  = "Authentic product seeding to a vetted creator network. We match the right makers with your story so the conversation starts where your audience already lives."
image = "/assets/images/studio-07-creators.webp"
split_image = "/assets/images/studio-07-creators.webp"
split_tag = "Creators"
split_stat = "Vetted Network"
split_cta = "Explore"
```

## Process

```toml
[process]
eyebrow = "How We Work"
title = "Our Proven Workflow."
sub   = "Every successful campaign starts with deep strategy and ends with measurable growth."

[[process.steps]]
num   = "01"
title = "Discovery & Strategy"
body  = "Deep-dive into your brand, audience, and goals to architect the perfect campaign."
image = "/assets/images/workflow-01-discovery.webp"
split_image = "/assets/images/workflow-01-discovery.webp"
split_tag = "Strategy"
split_stat = "Step 01"
split_cta = "Explore"

[[process.steps]]
num   = "02"
title = "Talent & Partner Curation"
body  = "Strategic selection of celebrities, influencers, vendors, and venues aligned with your vision."
image = "/assets/images/workflow-02-talent.webp"
split_image = "/assets/images/workflow-02-talent.webp"
split_tag = "Curation"
split_stat = "Step 02"
split_cta = "Explore"

[[process.steps]]
num   = "03"
title = "Creative Production"
body  = "World-class visuals, immersive staging, and compelling narratives that captivate audiences."
image = "/assets/images/workflow-03-production.webp"
split_image = "/assets/images/workflow-03-production.webp"
split_tag = "Production"
split_stat = "Step 03"
split_cta = "Explore"

[[process.steps]]
num   = "04"
title = "Execution & Management"
body  = "Flawless on-ground delivery with real-time adaptability and compliance at every turn."
image = "/assets/images/workflow-04-execution.webp"
split_image = "/assets/images/workflow-04-execution.webp"
split_tag = "Execution"
split_stat = "Step 04"
split_cta = "Explore"

[[process.steps]]
num   = "05"
title = "Performance Analysis"
body  = "Data-backed measurement, ROI reporting, and insights to optimize your next activation."
image = "/assets/images/workflow-05-analysis.webp"
split_image = "/assets/images/workflow-05-analysis.webp"
split_tag = "Analysis"
split_stat = "Step 05"
split_cta = "Explore"
```

## Cases

```toml
[cases]
eyebrow = "Showcase"
title = "Brands That Trusted Our Vision."
sub   = "We help startups, enterprises, and cultural pioneers build unforgettable market presence."
view_case_study = "View Case Study"

[[cases.items]]
client = "Be the first to be showcased here"
metric = "—"
desc   = "Partner with us to create history and have your brand featured here."
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

## Team

```toml
[[team]]
name     = "Arpita Parhi"
eyebrow  = "Founder · Star Gazer"
bio      = "An Odishi dancer with a quiet observer's eye, Arpita reads a room before she enters it. Her soft spot is the stardom her work moves around every day — and the warmth of hospitality she insists every guest leaves with. Velvt is built on both: an artist's attention to detail, and a host's instinct for people."
photo    = "/assets/images/arpita-recent.jpg"
monogram = "A"

[[team.handles]]
icon  = "instagram"
label = "@thearpitaparhi_official"
href  = "https://instagram.com/thearpitaparhi_official"

[[team]]
name     = "Adarsh Kumar Rout"
eyebrow  = "Creative Head"
bio      = "A veteran media professional and founder of Odisha Times, specializes in PR, media, and celebrity management. A dynamic on-screen personality and podcast host, he is passionate about crafting memorable events and bringing news to life."
photo    = "/assets/images/adarsh-kumar.jpg"
monogram = "A"
```

## Footer

```toml
[footer]
brand_desc = "From powerful connections to unforgettable moments, we build brands that inspire, influence and leave a lasting impact. Building brands. Creating impact."

[[footer.columns]]
title = "Services"
links = [
  { label = "Event Management", href = "#" },
  { label = "Celebrity Management", href = "#" },
  { label = "Influencer Campaigns", href = "#" },
  { label = "Custom Branding", href = "#" },
  { label = "Corporate Gifting", href = "#" },
]
[[footer.columns]]
title = "Company"
links = [
  { label = "About Us", href = "#about" },
  { label = "Our Work", href = "#showcase" },
  { label = "Case Studies", href = "/achivements" },
  { label = "Contact", href = "#contact" },
]

[[footer.socials]]
label = "twitter"
href  = "https://x.com/velvt"
[[footer.socials]]
label = "linkedin"
href  = "https://linkedin.com/company/velvt"
[[footer.socials]]
label = "instagram"
href  = "https://www.instagram.com/velvt_official/"
```