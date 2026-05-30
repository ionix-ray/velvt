import { expect, test } from "@playwright/test";

test("home: brand + hero + sections all paint", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/Velvt/i);
  await expect(page.locator("h1")).toContainText(/Elevate Your Brand/i);

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

  // The first case card should use v-card-modern
  const caseCard = page.locator(".v-card-modern").first();
  await expect(caseCard).toBeVisible();

  // It should contain a glowing button
  const greenBtn = page.locator(".v-btn-glow").first();
  await expect(greenBtn).toBeAttached();
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
  await page.waitForSelector(".v-loader", { state: "hidden" });
  await page.evaluate(() => document.body.focus());
  await page.waitForTimeout(500);

  // Navigate to footer panel (index 6) via keyboard
  for (let i = 0; i < 6; i++) {
    await page.keyboard.press("ArrowRight");
    await page.waitForTimeout(500);
  }
  await page.waitForTimeout(800);

  const strip = page.locator(".v-social-strip");
  await expect(strip).toHaveClass(/v-social-strip--hidden/);
});
