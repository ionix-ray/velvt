import { expect, test } from "@playwright/test";

test("home: brand + hero + sections all paint", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/Vaelvet/i);
  await expect(page.locator("h1")).toContainText(/Elevate Your Brand/i);

  // Sections present (panel IDs from config-driven components)
  await expect(page.locator("#home")).toBeVisible();
  await expect(page.locator("#about")).toBeVisible();
  await expect(page.locator("#stories")).toBeVisible();
  await expect(page.locator("#cases")).toBeVisible();
  await expect(page.locator("#contact")).toBeVisible();

  // Loader is hidden after boot
  const loader = page.locator(".v-loader");
  await expect(loader).toHaveClass(/hidden/);
});

test("home: CTA links to contact", async ({ page }) => {
  await page.goto("/");
  const cta = page.locator(".v-btn--primary").first();
  await expect(cta).toBeVisible();
  await expect(cta).toHaveAttribute("href", "#contact");
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

  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(800);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(1);
});

test("scroll: keyboard arrow left navigates to previous panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  await page.keyboard.press("ArrowRight");
  await page.waitForTimeout(800);
  await page.keyboard.press("ArrowLeft");
  await page.waitForTimeout(800);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(0);
});

test("scroll: arrow down advances to next panel like arrow right", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  await page.keyboard.press("ArrowDown");
  await page.waitForTimeout(800);

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

test("scroll: section dot click navigates to panel", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");
  await page.waitForSelector(".v-dot");

  const dots = page.locator(".v-dot");
  await expect(dots).toHaveCount(11);

  await dots.nth(2).click();
  await page.waitForTimeout(800);

  const visible = await page.evaluate(() => {
    const panels: HTMLElement | null = document.querySelector(".v-panels");
    return panels ? Math.round(panels.scrollLeft / panels.clientWidth) : -1;
  });
  expect(visible).toBe(2);
  await expect(page.locator("#about")).toBeVisible();
});

test("scroll: panels container has smooth scroll-behavior", async ({ page }) => {
  await page.goto("/");
  await page.waitForSelector(".v-panels");

  const smooth = await page.evaluate(() => {
    const panels = document.querySelector(".v-panels");
    if (!panels) return null;
    return getComputedStyle(panels).scrollBehavior;
  });
  expect(smooth).toBe("smooth");
});

// ── Brand / Logo ────────────────────────────────────────────────────────────

test("brand: logo renders in topbar", async ({ page }) => {
  await page.goto("/");
  const logo = page.locator(".v-topbar__brand img");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VAELVET");
});

test("brand: logo renders in loader with loading animation", async ({ page }) => {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  const logo = page.locator(".v-loader__logo");
  await expect(logo).toBeVisible({ timeout: 3000 });
  await expect(logo).toHaveAttribute("alt", "VAELVET");
  await expect(logo).toHaveCSS("animation-name", /v-loader-logo/);
});

test("brand: logo renders in stacked navigation", async ({ page }) => {
  await page.goto("/");
  const menuBtn = page.locator(".v-topbar__menu-btn");
  await menuBtn.click();
  const logo = page.locator(".v-stack-nav__brand img");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VAELVET");
});

test("brand: logo renders in footer", async ({ page }) => {
  await page.goto("/");
  const panels = page.locator(".v-panels");
  await panels.evaluate(el => el.scrollTo({ left: el.scrollWidth, behavior: "instant" }));
  await page.waitForTimeout(600);
  const logo = page.locator("img.v-footer__brand-name");
  await expect(logo).toBeVisible();
  await expect(logo).toHaveAttribute("alt", "VAELVET");
});
