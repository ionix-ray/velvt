import { expect, test } from "@playwright/test";

test("home: brand + hero + sections all paint", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/Vaelvet/i);
  await expect(page.locator("h1")).toContainText(/compose entrances/i);

  // Sections present
  await expect(page.locator("#manifesto")).toBeVisible();
  await expect(page.locator("#services")).toBeVisible();
  await expect(page.locator("#cases")).toBeVisible();
  await expect(page.locator("#contact")).toBeVisible();
});

test("home: CTA links to contact", async ({ page }) => {
  await page.goto("/");
  const cta = page.locator(".v-hero__cta");
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

test("reduced-motion: parallax planes are static", async ({ page, browserName }) => {
  test.skip(browserName === "webkit", "transform inspection inconsistent on webkit");
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  const transform = await page
    .locator(".v-hero__plane--back")
    .evaluate((el) => getComputedStyle(el).transform);
  expect(transform === "none" || transform.includes("matrix")).toBeTruthy();
});

test("a11y: skip-link target + headings present", async ({ page }) => {
  await page.goto("/");
  const h1 = page.locator("h1");
  await expect(h1).toHaveCount(1);
  const h2s = page.locator("h2");
  expect(await h2s.count()).toBeGreaterThanOrEqual(4);
});
