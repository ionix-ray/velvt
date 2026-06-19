---
name: graphify
description: Build a module dependency graph of vaelvet-ui on session start to enable selective, token-efficient context loading. Instead of loading all 29 components, load only the subgraph connected to the current task. Trigger on 'context', 'resume', 'session start', 'what to load', 'token budget'.
---

# Graphify — Velvet Module Dependency Graph

On session start: build a lightweight mental graph of `velvet-ui/src/` dependencies, then load only the nodes relevant to the current task.

## Module Graph

```
main.rs
  └─ App component
       ├─ scroll::install_reveal()     [scroll.rs]
       └─ Router<Route>
            └─ Home {}                [routes/home.rs]
                 ├─ config::Site      [config.rs]
                 ├─ Loader            [components/loader.rs]
                 ├─ ScrollProgress    [components/scroll_progress.rs]
                 ├─ TopBar            [components/topbar.rs]
                 ├─ StackedNav        [components/stacked_nav.rs]
                 ├─ SectionDots       [components/section_dots.rs]
                 ├─ SocialStrip       [components/social_strip.rs]
                 ├─ NextHint          [components/next_hint.rs]
                 └─ v-panels div
                      ├─ HeroPanel         [components/hero_panel.rs]
                      ├─ AboutAggregatedPanel [components/about_aggregated_panel.rs]
                      │    ├─ Services     [services.rs, services_panel.rs]
                      │    ├─ Story        [manifesto.rs]
                      │    └─ Analytics    [awards_panel.rs]
                      ├─ ProcessPanel      [components/process_panel.rs]
                      ├─ StudioPanel       [components/studio_panel.rs]
                      ├─ CasesPanel        [components/cases_panel.rs]
                      │    └─ CaseStudies  [case_studies.rs]
                      ├─ CtaPanel          [components/cta_panel.rs]
                      │    ├─ Contact      [contact.rs]
                      │    ├─ AiPanel      [ai_panel.rs]
                      │    ├─ PrPanel      [pr_panel.rs]
                      │    └─ WorkWithUs   [work_with_us.rs]
                      └─ FooterPanel       [components/footer_panel.rs]
                           └─ Nav          [nav.rs]

Shared by all:
  config::Site  ←── content/site.toml (compile-time)
  theme::tokens ←── assets/theme.css (CSS vars)
  prelude       ←── Site + dioxus::prelude::* + window()
```

## Selective Loading Protocol

| Task | Load |
|---|---|
| Edit hero text/layout | `config.rs` + `hero_panel.rs` + `site.toml` |
| Fix scroll/navigation | `scroll.rs` + `home.rs` |
| Add config field | `config.rs` + `site.toml` + relevant panel |
| CSS compaction | `theme.css` only (don't load Rust files) |
| Tofu infra change | `ops/tofu/` only — no Rust files |
| New test | `velvet-ui/tests/integration.rs` + the tested component |
| CI workflow | `.github/workflows/` only |

## Level 0 (always load — ~300 tokens)

```
STATE.md (current status)
.skills/samir-product-philosophy/SKILL.md (hard rules)
.skills/INDEX.md (trigger map)
```

## Anti-Patterns

- Loading all 29 component files when editing one → load 1 component + config.rs only
- Loading full theme.css (49KB) for a Rust change → don't load it
- Loading AGENT_DIRECTIVE.md mid-session → load once on boot only

## Cross-References

- `.skills/token-optimization/SKILL.md` — progressive disclosure strategy
- `.skills/token-frugal/SKILL.md` — per-turn minimization
