# Vaelvet — *elevate your Presence.*

A cinematic PR agency website. Built in Dioxus (Rust → WASM). Config-driven: edit `content/site.md` and rebuild — no code changes needed for copy.

---

## Quick start

```bash
just dev          # dev server with hot reload, opens browser
just build        # production WASM build
just test         # all tests
just container-up # build + run container + open browser (single command)
```

Requires: Rust 1.85+, `dx` CLI (`cargo install dioxus-cli --version 0.7.6`).

### Container deployment (production-grade Rust server)

```bash
just container-up    # Build WASM → build Rust server → container → browser
just container-stop  # Stop running container
just container-size  # Check image + WASM bundle size
```

The container runs a minimal **Rust static file server** (built with Axum + Tokio) inside a **distroless** runtime — no Python, no SWS, no bloated base images.

- **Security**: Content-Security-Policy, HSTS, X-Frame-Options, non-root user, read-only filesystem
- **SPA routing**: All non-file routes fall back to `index.html` for Dioxus client-side navigation
- **Static assets**: Hashed bundles cached forever (`immutable`), HTML served with `no-cache`
- **Fast builds**: WASM compiles locally with cargo incremental, container just copies files (~2-5s)

### Manual container steps

```bash
just build                              # WASM build → target/dx/.../public
cargo build --release -p velvet-server  # Rust server binary
cp -a target/dx/vaelvet-ui/release/web/public/. deployment/
cp target/release/velvet-server deployment/
podman build -t localhost/velvet:latest .
podman run --rm -p 8080:8080 localhost/velvet:latest
```

---

## To edit site content

The canonical content lives in **[`content/site.md`](content/site.md)**. It's a markdown file — one `## Section` heading per content field, each followed by a fenced `toml` block holding that field's data. Edit the values inside the fences, rebuild. The Rust app strips the markdown prose, concatenates the fenced blocks, and parses the result at compile time via `include_str!`, so there is no runtime config fetch.

The sections below mirror that file so you can read the site at a glance.

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
6. **Gift Card Solution** — End-to-end generation and lifecycle management of custom coupons and gift cards.

## Case Studies

- **Aurora's Last Light** — A24 indie debut → 3 Cannes invitations, $12M opening.
- **House of Marais** — couture relaunch → 14 cover stories in 9 weeks.
- **Project Lumen** — fintech founder → WSJ profile, TED keynote, $80M Series C.

## Manifesto

We believe presence is a craft. Most agencies measure reach. We measure resonance — the silence after the room turns. We are deliberately small, deliberately picky, and deliberately quiet about most of what we do.

## Contact

- **Email** — connect@velvt.live
- **Atelier** — Bandra West, Mumbai · Tribeca, New York
- **Press inquiries** — connect@velvt.live

---

## Architecture (for engineers)

- `velvet-ui/` — Dioxus WASM app
- `server/` — Production static file server (Axum + Tokio)
  - `src/handlers.rs` — health check, static file serving, SPA fallback
  - `src/middleware.rs` — security headers (CSP, HSTS, X-Frame-Options)
  - `src/config.rs` — runtime config (addr, static root)
  - `src/error.rs` — typed error handling with HTTP status mapping
- `velvet-ui/src/config.rs` — parses `content/site.md` at compile time via `include_str!`
- `velvet-ui/src/components/` — render that struct; no business logic in components
- `velvet-ui/assets/theme.css` — cinematic layer, hand-written, ≤40 KB
- `content/site.md` — single source of truth for site content
- `deployment/` — staging folder for container (pre-built WASM + Rust server binary)

See [`CLAUDE.md`](CLAUDE.md) for the engineering rulebook.

## License

© Vaelvet. All rights reserved. Brand marks are not licensed.
