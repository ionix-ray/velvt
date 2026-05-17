# DEPLOY.md — Velvet PR Agency Deployment Guide

## Build Commands
```bash
just build        # Production WASM build → target/dx/velvet-ui/release/web/public/
just static       # Generate static output in dist/
just dev          # Local dev server (port 8080-8100)
just test         # Run all tests
just lint         # fmt + clippy
just audit        # Security audit
```

## Build Output
- Location: `target/dx/velvet-ui/release/web/public/`
- Contents: `index.html`, `assets/` (WASM, CSS, JS)
- WASM size: ~425KB (well under 1.5MB budget)

## Deployment Options

### Vercel
```bash
# 1. Build
just build

# 2. Copy output to dist/
cp -r target/dx/velvet-ui/release/web/public/* dist/

# 3. Deploy
vercel dist --prod
```

### Netlify
```bash
# 1. Build
just build

# 2. Deploy
netlify deploy --prod --dir=target/dx/velvet-ui/release/web/public
```

### Cloudflare Pages
```bash
# 1. Build
just build

# 2. Deploy via Wrangler
wrangler pages deploy target/dx/velvet-ui/release/web/public --project-name=velvet
```

### GitHub Pages
```bash
# 1. Build with base path
dx build --release

# 2. Push to gh-pages branch
git subtree push --prefix target/dx/velvet-ui/release/web/public origin gh-pages
```

### Static File Server (Self-hosted)
```bash
# Using Python
cd target/dx/velvet-ui/release/web/public && python3 -m http.server 8080

# Using Caddy
caddy file-server --root target/dx/velvet-ui/release/web/public --listen :8080

# Using NGINX
# server { listen 80; root /path/to/public; index index.html; }
```

## Post-Deployment Checklist
- [ ] Verify all routes render correctly
- [ ] Check Lighthouse scores (Perf ≥90, A11y ≥95, Best Practices ≥95, SEO ≥95)
- [ ] Validate JSON-LD with Google Rich Results Test
- [ ] Confirm sitemap.xml accessible at /sitemap.xml
- [ ] Verify robots.txt at /robots.txt
- [ ] Test responsive breakpoints (320px, 768px, 1024px, 1440px)
- [ ] Verify keyboard navigation
- [ ] Confirm CSP headers present

## CDN Configuration
- Set Cache-Control: `public, max-age=31536000, immutable` for WASM/CSS/JS
- Set Cache-Control: `no-cache` for index.html
- Enable Brotli compression
- Enable HTTP/2 or HTTP/3
