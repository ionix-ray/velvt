# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S1 — First production build
**Current task**: S1-01/S1-02 — containerization, security headers, JS inline
**Last action**: Container builds at 11.5 MB distroless; all security headers verified; JS inlined (no external JS); WASM optimized via wasm-opt; config via sws.toml
**Next action**: Sprint S1 items from TASKS.md (IntersectionObserver, animated V-mark, Carbon grid, etc.)
**Files touched**: justfile, Containerfile, sws.toml, scripts/inline-js.sh, config.rs (OUT_DIR fix), build.rs removed
**Open questions**: none
**Rollback**: `git reset --hard v0.1.0-pre-vaelvet`
