# Vaelvet — *elevate your Presence.*

A cinematic PR agency website. Built in Dioxus (Rust → WASM). Config-driven: edit this file or `content/site.toml` and rebuild — no code changes needed for copy.

---

## Quick start

```bash
just dev          # dev server with hot reload
just build        # production build → dist/
just test         # all tests
```

Requires: Rust 1.85+, `dx` CLI (`cargo install dioxus-cli --version 0.7.6`).

---

## To edit site content

The canonical content lives in **[`content/site.toml`](content/site.toml)**. Update strings there, rebuild. The Rust app parses the TOML at compile time via `include_str!`, so there is no runtime config fetch.

The sections below mirror that TOML so you can read the site at a glance.

---

## Brand

- **Name**: Vaelvet
- **Tagline**: *elevate your Presence.*
- **Promise**: We craft the room before you walk into it.

## Navigation

`Manifesto` · `Services` · `Case Studies` · `Roster` · `Contact`

## Hero

> *We don't write press releases. We compose entrances.*

Vaelvet is a premium public-relations house for film, music, fashion, and founders who refuse to be ordinary. Every campaign is staged like a third-act reveal.

CTA: **Book a private consultation →**

## Services

1. **Cinematic PR** — press tours, festival debuts, premiere choreography.
2. **Crisis & Counsel** — narrative repair, executive shielding, 72-hour response.
3. **Talent Curation** — actor, athlete, founder positioning across global press.
4. **Event Direction** — premieres, listening sessions, runway after-parties.
5. **Editorial Placement** — long-lead features in Vogue, GQ, Vanity Fair, Hollywood Reporter, Variety.

## Case Studies

- **Aurora's Last Light** — A24 indie debut → 3 Cannes invitations, $12M opening.
- **House of Marais** — couture relaunch → 14 cover stories in 9 weeks.
- **Project Lumen** — fintech founder → WSJ profile, TED keynote, $80M Series C.

## Manifesto

We believe presence is a craft. Most agencies measure reach. We measure resonance — the silence after the room turns. We are deliberately small, deliberately picky, and deliberately quiet about most of what we do.

## Contact

- **Email** — hello@vaelvet.com
- **Atelier** — Bandra West, Mumbai · Tribeca, New York
- **Press inquiries** — press@vaelvet.com

---

## Architecture (for engineers)

- `velvet-ui/` — Dioxus WASM app
- `velvet-ui/src/config.rs` — parses `content/site.toml` at compile time into a `Site` struct
- `velvet-ui/src/components/` — render that struct; no business logic in components
- `velvet-ui/assets/theme.css` — cinematic layer, hand-written, ≤40 KB
- `content/site.toml` — single source of truth for site content

See [`CLAUDE.md`](CLAUDE.md) for the engineering rulebook.

## License

© Vaelvet. All rights reserved. Brand marks are not licensed.
