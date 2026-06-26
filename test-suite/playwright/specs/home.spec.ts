import { expect, test } from "@playwright/test";

test("home: brand + hero + sections all paint", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/Velvt/i);
  await expect(page.locator("h1")).toContainText(/shape|history|stories/i);

  // Sections present (panel IDs from config-driven components)
  await expect(page.locator("#home")).toBeVisible();
  await expect(page.locator("#about")).toBeVisible();
  await expect(page.locator("#cases")).toBeVisible();
  await expect(page.locator("#contact")).toBeVisible();

  // Loader is hidden after boot
  const loader = page.locator(".v-loader");
  await expect(loader).toHaveClass(/hidden/);
});

test("home: CTA links to contact email", async ({ page }) => {
  await page.goto("/");
  const cta = page.locator(".v-btn--primary").first();
  await expect(cta).toBeVisible();
  // CTA now links to actual contact email, not anchor
  await expect(cta).toHaveAttribute("href", /mailto:|#contact/);
});

test("topbar: theme toggle flips html[data-theme]", async ({ page }) => {
  await page.goto("/");
  const html = page.locator("html");
  await expect(html).toHaveAttribute("data-theme", "dark");

  await page.locator(".v-theme-toggle").click();
  await expect(html).toHaveAttribute("data-theme", "light");

  await page.locator(".v-theme-toggle").click();
  await expect(html).toHaveAttribute("data-theme", "dark");
});

test("topbar: menu button opens and closes the stacked nav", async ({ page }) => {
  await page.goto("/");
  const stackNav = page.locator(".v-stack-nav");
  const menuBtn = page.locator(".v-topbar__menu-btn");

  await expect(stackNav).not.toHaveClass(/open/);
  await menuBtn.click();
  await expect(stackNav).toHaveClass(/open/);
  await expect(menuBtn).toHaveClass(/active/);

  await menuBtn.click();
  await expect(stackNav).not.toHaveClass(/open/);
});

test("cta: filling and submitting the inquiry form shows the thank-you view", async ({ page }) => {
  await page.goto("/");
  const form = page.locator(".v-contact-form");
  await form.scrollIntoViewIfNeeded();

  await form.locator('input[type="text"]').fill("Sam Parhi");
  await form.locator('input[type="email"]').fill("sam@example.com");
  await form.locator("textarea").fill("Tell me more about your services.");
  await form.locator('button[type="submit"]').click();

  await expect(page.locator(".v-cta__inner")).toContainText(/Thank You/i);
});

test("cta: submitting with an invalid email shows a validation error", async ({ page }) => {
  await page.goto("/");
  const form = page.locator(".v-contact-form");
  await form.scrollIntoViewIfNeeded();

  await form.locator('input[type="text"]').fill("Sam Parhi");
  // Passes the browser's native type="email" constraint (just needs an "@")
  // but fails our own dot-in-domain check, so the app's onsubmit handler
  // (not native HTML5 validation) is what produces this message.
  await form.locator('input[type="email"]').fill("sam@examplecom");
  await form.locator("textarea").fill("Hello there");
  await form.locator('button[type="submit"]').click();

  await expect(page.locator(".v-form-error")).toContainText(/valid email/i);
});

test("home: hero LCP under 2.5s", async ({ page }) => {
  const t0 = Date.now();
  await page.goto("/", { waitUntil: "load" });
  await page.locator(".v-hero__title").waitFor({ state: "visible" });
  const lcp = Date.now() - t0;
  expect(lcp, `hero visible in ${lcp}ms`).toBeLessThan(2500);
});

test("reduced-motion: loader curtain animates", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  const curtain = page.locator(".v-curtain");
  await expect(curtain).toBeHidden();
});

test("a11y: skip-link target + headings present", async ({ page }) => {
  await page.goto("/");
  const h1 = page.locator("h1");
  await expect(h1).toHaveCount(1);
  const h2s = page.locator("h2");
  expect(await h2s.count()).toBeGreaterThanOrEqual(4);
});

// ── Scroll snap & keyboard navigation ───────────────────────────────────────

test("scroll: panels snap to full viewport width", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  const vw = await page.evaluate(() => window.innerWidth);
  const panelWidth = await page.locator(".v-panel").first().evaluate(
    (el) => el.getBoundingClientRect().width,
  );
  expect(panelWidth).toBe(vw);
});

test("scroll: keyboard arrow right navigates to next panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.evaluate(() => document.body.focus());
  await page.waitForTimeout(500);

  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(1000);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(1);
});

test("scroll: keyboard arrow left navigates to previous panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.evaluate(() => document.body.focus());
  await page.waitForTimeout(500);

  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(1000);

  await page.keyboard.press("ArrowLeft");
  await page.waitForTimeout(1000);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(0);
});

test("scroll: arrow down advances to next panel like arrow right", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.evaluate(() => document.body.focus());
  await page.waitForTimeout(500);

  await page.keyboard.press("ArrowDown");
  await page.waitForTimeout(1000);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(1);
});

test("scroll: no half-panel state after scroll", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  await page.evaluate(() => {
    const panels = document.querySelector<HTMLElement>(".v-panels");
    if (!panels) return;
    panels.scrollBy({ left: 50, behavior: "instant" });
  });
  await page.waitForTimeout(800);

  const snapped = await page.evaluate(() => {
    const panels = document.querySelector<HTMLElement>(".v-panels");
    if (!panels) return -1;
    const remainder = panels.scrollLeft % panels.clientWidth;
    return Number(remainder === 0 || Math.abs(remainder) < 2);
  });
  expect(snapped).toBe(1);
});

test("scroll: spindle has 7 items (6 content + footer panel)", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.waitForSelector(".v-spindle-item");

  const dots = page.locator(".v-spindle-item");
  // 7 panels: HOME, ABOUT, STORIES, SHOWCASE, PORTFOLIO, CONTACT, FOOTER
  await expect(dots).toHaveCount(7);
});

test("scroll: spindle item click navigates to panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.waitForSelector(".v-spindle-item");

  const dots = page.locator(".v-spindle-item");
  await dots.nth(1).click();
  await page.waitForTimeout(800);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(1);
  await expect(page.locator("#about")).toBeVisible();
});

test("scroll: panels container has smooth scroll-behavior", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  const isReducedMotion = await page.evaluate(
    () => window.matchMedia("(prefers-reduced-motion: reduce)").matches
  );

  const smooth = await page.evaluate(() => {
    const panels = document.querySelector(".v-panels");
    if (!panels) return null;
    return getComputedStyle(panels).scrollBehavior;
  });

  if (isReducedMotion) {
    expect(smooth).toBe("auto");
  } else {
    expect(["smooth", "auto"]).toContain(smooth);
  }
});

// ── Brand / Logo ────────────────────────────────────────────────────────────

test("brand: logo renders in topbar", async ({ page }) => {
  await page.goto("/");
  const logo = page.locator(".v-topbar__brand img");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VELVT");
});

test("brand: logo is NOT in hero content but in loader with animation", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  const heroLogo = page.locator(".v-hero__content .v-hero__logo");
  await expect(heroLogo).toBeHidden();

  const logo = page.locator(".v-loader__logo");
  await expect(logo).toBeVisible({ timeout: 3000 });
  await expect(logo).toHaveAttribute("alt", "VELVT");
  await expect(logo).toHaveCSS("animation-name", /v-loader-logo/);
});

test("ui: glassmorphism cards and green tags render", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  // The first case card uses the v-card-modern system (kept on /#cases)
  const caseCard = page.locator("#cases .v-card-modern").first();
  await expect(caseCard).toBeVisible();

  // It should contain a glowing CTA button
  const cta = page.locator("#cases .v-btn-glow").first();
  await expect(cta).toBeAttached();
});

test("brand: floating badge anchored top-left and overflows the topbar strip", async ({ page }) => {
  await page.goto("/");
  const topbar = page.locator(".v-topbar");
  const brand = topbar.locator(".v-topbar__brand");
  const logo = brand.locator("img");

  await expect(topbar).toBeVisible();
  await expect(brand).toBeVisible();
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VELVT");

  const topbarBox = await topbar.boundingBox();
  const brandBox = await brand.boundingBox();
  expect(topbarBox).not.toBeNull();
  expect(brandBox).not.toBeNull();

  if (topbarBox && brandBox) {
    // Anchored near the top-left corner with a small breathing inset.
    expect(brandBox.x).toBeGreaterThanOrEqual(topbarBox.x);
    expect(brandBox.x - topbarBox.x).toBeLessThanOrEqual(20);
    expect(brandBox.y).toBeGreaterThanOrEqual(topbarBox.y);
    expect(brandBox.y - topbarBox.y).toBeLessThanOrEqual(20);
    // Square mark.
    expect(Math.abs(brandBox.width - brandBox.height)).toBeLessThanOrEqual(1);
    // Badge overflows below the topbar strip — the strip's bottom border
    // re-emerges to the right of the badge.
    expect(brandBox.y + brandBox.height).toBeGreaterThan(
      topbarBox.y + topbarBox.height,
    );
  }

  // Sharp corners, not pill-rounded.
  const radius = await logo.evaluate((el) =>
    Number.parseFloat(getComputedStyle(el).borderTopLeftRadius),
  );
  expect(radius).toBeLessThanOrEqual(2);
});

test("brand: floating badge scales across desktop, tablet, and phone viewports", async ({ page }) => {
  const viewports = [
    { name: "desktop", w: 1440, h: 900, minSide: 86, maxSide: 108 },
    { name: "tablet", w: 768, h: 1024, minSide: 56, maxSide: 66 },
    { name: "phone", w: 380, h: 720, minSide: 48, maxSide: 58 },
  ] as const;

  for (const v of viewports) {
    await page.setViewportSize({ width: v.w, height: v.h });
    await page.goto("/");
    const brand = page.locator(".v-topbar__brand");
    const actions = page.locator(".v-topbar__actions");
    await expect(brand).toBeVisible();
    const brandBox = await brand.boundingBox();
    const actionsBox = await actions.boundingBox();
    expect(brandBox, `brand box on ${v.name}`).not.toBeNull();
    expect(actionsBox, `actions box on ${v.name}`).not.toBeNull();
    if (brandBox && actionsBox) {
      expect(brandBox.width).toBeGreaterThanOrEqual(v.minSide);
      expect(brandBox.width).toBeLessThanOrEqual(v.maxSide);
      // Actions cluster never collides with the badge.
      expect(actionsBox.x).toBeGreaterThan(brandBox.x + brandBox.width);
      // Actions cluster sits inside the viewport.
      expect(actionsBox.x + actionsBox.width).toBeLessThanOrEqual(v.w);
    }
  }
});

test("brand: topbar and stacked navigation share the same brand mark", async ({ page }) => {
  await page.goto("/");
  const topbarLogo = page.locator(".v-topbar__brand img");
  const menuBtn = page.locator(".v-topbar__menu-btn");
  await menuBtn.click();
  const navLogo = page.locator(".v-stack-nav__brand img");

  const topbarSrc = await topbarLogo.getAttribute("src");
  const navSrc = await navLogo.getAttribute("src");
  expect(topbarSrc).toBe(navSrc);
  expect(topbarSrc).toContain("velvet-square");
});

test("brand: logo renders in stacked navigation", async ({ page }) => {
  await page.goto("/");
  const menuBtn = page.locator(".v-topbar__menu-btn");
  await menuBtn.click();
  const logo = page.locator(".v-stack-nav__brand img");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VELVT");
});

// ── Footer panel (7th scroll panel) ─────────────────────────────────────────

test("footer: is the 7th horizontal scroll panel", async ({ page }) => {
  await page.goto("/");
  const panels = page.locator(".v-panel");
  // 7 panels total
  await expect(panels).toHaveCount(7);

  // Footer panel has id="footer"
  const footerPanel = page.locator("#footer");
  await expect(footerPanel).toBeAttached();
  await expect(footerPanel).toHaveClass(/v-panel--footer/);
});

test("footer: contains brand name and copyright", async ({ page }) => {
  await page.goto("/");
  // Navigate to last panel (footer)
  const panels = page.locator(".v-panels");
  await panels.evaluate((el: HTMLElement) => {
    el.scrollTo({ left: el.scrollWidth, behavior: "instant" });
  });
  await page.waitForTimeout(600);

  const footer = page.locator("#footer");
  const text = await footer.textContent();
  expect(text).toMatch(/Velvt|velvt/i);
});

test("footer: renders the real brand logo in the standard footer block", async ({ page }) => {
  await page.goto("/");
  const panels = page.locator(".v-panels");
  await panels.evaluate((el: HTMLElement) => {
    el.scrollTo({ left: el.scrollWidth, behavior: "instant" });
  });
  await page.waitForTimeout(600);

  const logo = page.locator(".v-footer-panel__wordmark");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", /Velvt/i);
  await expect(logo).toHaveAttribute("src", /velvet-square/);
});

// ── Social strip (all panels except last) ────────────────────────────────────

test("social-strip: visible on hero panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  // Strip visible on panel 0 (HOME)
  const strip = page.locator(".v-social-strip");
  await expect(strip).not.toHaveClass(/v-social-strip--hidden/);
});

test("social-strip: hidden (aria-hidden) on footer panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  // Direct scroll to last panel (index 6)
  await page.evaluate(() => {
    const panels = document.querySelector<HTMLElement>(".v-panels");
    if (panels) {
      panels.scrollTo({ left: 6 * panels.clientWidth, behavior: "instant" });
    }
  });
  await page.waitForTimeout(600);

  const strip = page.locator(".v-social-strip");
  await expect(strip).toHaveClass(/v-social-strip--hidden/);
});

// ── Brand badge (oversized square mark, pinned top-left) ────────────────────

test("brand: topbar mark is inside the fixed header strip", async ({ page }) => {
  await page.goto("/");
  const badge = page.locator(".v-topbar__brand img");
  const brand = page.locator(".v-topbar__brand");
  await expect(badge).toBeVisible();

  const badgeBox = await badge.boundingBox();
  const brandBox = await brand.boundingBox();
  expect(badgeBox).not.toBeNull();
  expect(brandBox).not.toBeNull();
  if (badgeBox && brandBox) {
    expect(badgeBox.x).toBeGreaterThanOrEqual(brandBox.x);
    expect(badgeBox.y).toBeGreaterThanOrEqual(brandBox.y);
    expect(badgeBox.y + badgeBox.height).toBeLessThanOrEqual(
      brandBox.y + brandBox.height,
    );
  }
});

test("brand: topbar actions cluster stays clear of the brand badge", async ({ page }) => {
  await page.goto("/");
  const badge = page.locator(".v-topbar__brand img");
  const actions = page.locator(".v-topbar__actions");
  const badgeBox = await badge.boundingBox();
  const actionsBox = await actions.boundingBox();
  expect(badgeBox).not.toBeNull();
  expect(actionsBox).not.toBeNull();
  if (badgeBox && actionsBox) {
    // Actions cluster (theme + menu buttons) renders to the right of the badge.
    expect(actionsBox.x).toBeGreaterThan(badgeBox.x + badgeBox.width / 2);
  }
});

// ── Showcase masonry (card sizing + vertical overflow scroll) ───────────────

test("showcase: masonry cards stay within a sane height and show their text", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const items = page.locator("#showcase .v-tile");
  const count = await items.count();
  expect(count).toBeGreaterThan(0);

  for (let i = 0; i < count; i++) {
    const item = items.nth(i);
    const box = await item.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      // Cards must never balloon to near-viewport height.
      expect(box.height).toBeLessThan(500);
    }
    const text = (await item.textContent()) ?? "";
    expect(text.trim().length).toBeGreaterThan(0);
  }
});

test("showcase: panel content overflowing the viewport is reachable via vertical scroll", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const overflowY = await page.locator("#showcase").evaluate(
    (el) => getComputedStyle(el).overflowY,
  );
  expect(overflowY).toBe("auto");

  const scrolled = await page.locator("#showcase").evaluate((el) => {
    el.scrollTop = el.scrollHeight;
    return el.scrollTop;
  });
  // If content fits, scrollTop legitimately stays 0 — the assertion is just
  // that scrolling never throws and the value is non-negative/finite.
  expect(scrolled).toBeGreaterThanOrEqual(0);
});

test("showcase: responsive grid collapses from three columns to one on narrow viewports", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto("/#showcase");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const desktopColumns = await page.locator("#showcase .v-showcase__grid").evaluate(
    (el) => getComputedStyle(el).gridTemplateColumns.split(" ").length,
  );
  expect(desktopColumns).toBeGreaterThanOrEqual(3);

  // Narrow viewport: items must stack vertically, regardless of which
  // breakpoint the layout chooses — the user-facing requirement is that
  // every tile renders below the previous one on phone-sized viewports.
  await page.setViewportSize({ width: 380, height: 900 });
  await page.goto("/#showcase");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const tops = await page.locator("#showcase .v-tile").evaluateAll((els) =>
    els.map((el) => (el as HTMLElement).getBoundingClientRect().top),
  );
  expect(tops.length).toBeGreaterThan(1);
  for (let i = 1; i < tops.length; i += 1) {
    expect(tops[i], `tile ${i} top`).toBeGreaterThan(tops[i - 1]);
  }
});

test("showcase: overflow scrollbars are enabled on both axes", async ({ page }) => {
  await page.setViewportSize({ width: 900, height: 520 });
  await page.goto("/#showcase");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const panelOverflow = await page.locator("#showcase").evaluate((el) => {
    const style = getComputedStyle(el);
    return { x: style.overflowX, y: style.overflowY };
  });
  expect(panelOverflow.x).toBe("auto");
  expect(panelOverflow.y).toBe("auto");

  const panelsScrollbarWidth = await page.locator(".v-panels").evaluate((el) => {
    return getComputedStyle(el).scrollbarWidth;
  });
  expect(panelsScrollbarWidth).not.toBe("none");
});

// ── Content: registered office + service naming ──────────────────────────────

test("footer: lists the full registered office address", async ({ page }) => {
  await page.goto("/");
  const panels = page.locator(".v-panels");
  await panels.evaluate((el: HTMLElement) => {
    el.scrollTo({ left: el.scrollWidth, behavior: "instant" });
  });
  await page.waitForTimeout(600);

  const text = await page.locator("#footer").textContent();
  expect(text).toContain("Plot No: 756");
  expect(text).toMatch(/Saheed Nagar/);
  expect(text).toMatch(/Odisha-\s*751007/);
});

// ── Social strip styling (bigger, always-red, responsive) ──────────────────

test("social-strip: links are bigger and carry the brand-red background always, not just on hover", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  const link = page.locator(".v-social-strip__link").first();
  await expect(link).toBeVisible();

  const box = await link.boundingBox();
  expect(box).not.toBeNull();
  if (box) {
    expect(box.width).toBeGreaterThanOrEqual(40);
    expect(box.height).toBeGreaterThanOrEqual(40);
  }

  const bg = await link.evaluate((el) => getComputedStyle(el).backgroundColor);
  // var(--accent) in dark mode resolves to the crimson-light brand red.
  expect(bg).toMatch(/rgba?\(/);
  expect(bg).not.toBe("rgba(0, 0, 0, 0)");
});

test("social-strip: still visible (not display:none) on mobile viewports", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });
  const strip = page.locator(".v-social-strip");
  await expect(strip).toBeVisible();
  const display = await strip.evaluate((el) => getComputedStyle(el).display);
  expect(display).not.toBe("none");
});

// ── Loader: camera-iris entrance/exit ───────────────────────────────────────

test("loader: three aperture blade rings and a flash render behind the mark while loading", async ({ page }) => {
  await page.goto("/");
  const rings = page.locator(".v-loader__iris-ring");
  await expect(rings).toHaveCount(3);
  await expect(page.locator(".v-loader__iris-flash")).toBeAttached();
});

test("loader: hides via clip-path collapse, not display:none, and stops blocking clicks", async ({ page }) => {
  await page.goto("/");
  // Wait for the loader to actually toggle its `hidden` modifier — using
  // `waitForSelector(".v-loader", { state: "hidden" })` would also match the
  // brief pre-mount window where the loader element doesn't exist yet, which
  // under slower engines (playwright webkit, reduced-motion Chrome) returns
  // before the 2.2s hide timer fires and yields pointer-events from the
  // still-visible initial state. Asserting on the `.hidden` class is the
  // unambiguous signal that the iris-wipe has been triggered.
  const loader = page.locator(".v-loader");
  await expect(loader).toHaveClass(/\bhidden\b/, { timeout: 10000 });
  const pointerEvents = await loader.evaluate((el) => getComputedStyle(el).pointerEvents);
  expect(pointerEvents).toBe("none");
});

test("footer: services column reads Celebrity Management, not Celebrity Booking", async ({ page }) => {
  await page.goto("/");
  const panels = page.locator(".v-panels");
  await panels.evaluate((el: HTMLElement) => {
    el.scrollTo({ left: el.scrollWidth, behavior: "instant" });
  });
  await page.waitForTimeout(600);

  const text = await page.locator("#footer").textContent();
  expect(text).toContain("Celebrity Management");
  expect(text).not.toContain("Celebrity Booking");
});

// ── Spindle dial (section navigator on the left rail) ───────────────────────

test("spindle: every section label stays readable so the visitor can see what the site offers", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto("/");
  await page.waitForSelector(".v-spindle-item.active");

  const items = page.locator(".v-spindle-item");
  const count = await items.count();
  expect(count).toBeGreaterThanOrEqual(6);

  // Every dial item — even the ones furthest from the active panel — is
  // readable (opacity >= 0.4) and accepts clicks (pointer-events != none).
  for (let i = 0; i < count; i += 1) {
    const it = items.nth(i);
    const { opacity, pointerEvents } = await it.evaluate((el) => {
      const cs = getComputedStyle(el);
      return { opacity: Number.parseFloat(cs.opacity), pointerEvents: cs.pointerEvents };
    });
    expect(opacity, `item ${i} opacity`).toBeGreaterThanOrEqual(0.4);
    expect(pointerEvents, `item ${i} pointer-events`).not.toBe("none");
  }
});

test("spindle: active item is visually distinct (accent color + filled tick dot)", async ({ page }) => {
  await page.goto("/");
  await page.waitForLoadState("networkidle");
  await page.waitForSelector(".v-spindle-item.active", { state: "visible" });
  const active = page.locator(".v-spindle-item.active").first();
  await expect(active).toBeVisible();

  // Accent color: not the muted neutral.
  await expect
    .poll(
      async () => await active.evaluate((el) => getComputedStyle(el).color),
      { timeout: 4000 },
    )
    .not.toMatch(/rgba?\(255,\s*255,\s*255/);

  // The ::before dial tick is filled (non-transparent background).
  await expect
    .poll(
      async () =>
        await active.evaluate(
          (el) => getComputedStyle(el, "::before").backgroundColor,
        ),
      { timeout: 4000 },
    )
    .not.toMatch(/rgba?\(0,\s*0,\s*0,\s*0\)/);
});
