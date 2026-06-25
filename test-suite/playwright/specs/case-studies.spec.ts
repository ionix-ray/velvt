import { expect, test } from "@playwright/test";

test("cases panel: View Case Study links to the internal case study page", async ({ page }) => {
  await page.goto("/");
  const cases = page.locator("#cases");
  await cases.scrollIntoViewIfNeeded();

  const firstLink = cases.locator(".v-btn-glow").first();
  await expect(firstLink).toHaveAttribute("href", /^\/cases\/[a-z0-9-]+$/);

  await firstLink.click();
  await expect(page).toHaveURL(/\/cases\/technova-full-funnel-growth$/);
});

test("case study detail: renders frontmatter and markdown body", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");

  await expect(page.locator("h1")).toContainText(/full-funnel growth/i);
  await expect(page.locator(".v-case-hero__metric")).toContainText("+240%");
  await expect(page.locator(".v-prose h2").first()).toContainText(/The brief/i);
  await expect(page.locator(".v-prose table")).toBeVisible();

  const tagLink = page.locator(".v-tags .v-tag--green").first();
  await expect(tagLink).toHaveAttribute("href", /^\/cases\/tag\//);
});

test("case study detail: unknown slug shows a not-found state, not a crash", async ({ page }) => {
  await page.goto("/cases/no-such-case-study");
  await expect(page.locator("h1")).toContainText(/not found/i);
  await expect(page.locator("a.v-btn-glow")).toHaveAttribute("href", "/");
});

test("case studies index: lists all sample studies and filters by tag", async ({ page }) => {
  await page.goto("/cases");
  await expect(page.locator(".v-card-modern__client")).toHaveCount(3);

  await page.goto("/cases/tag/Beauty");
  await expect(page.locator(".v-card-modern__client")).toHaveCount(1);
  await expect(page.locator(".v-card-modern__client").first()).toContainText("Luxe Beauty");
});

test("case page header: shows the brand mark image, not text", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");
  const brandImg = page.locator(".v-topbar__brand img");
  await expect(brandImg).toBeVisible();
  await expect(brandImg).toHaveAttribute("alt", /.+/);
});

test("case page header: shares the home page's floating-badge topbar layout", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");
  const topbar = page.locator(".v-topbar");
  const brand = topbar.locator(".v-topbar__brand");
  const actions = topbar.locator(".v-topbar__actions");
  const themeBtn = actions.locator(".v-theme-toggle");
  await expect(topbar).toBeVisible();
  await expect(brand).toBeVisible();
  await expect(themeBtn).toBeVisible();

  const topbarBox = await topbar.boundingBox();
  const brandBox = await brand.boundingBox();
  if (topbarBox && brandBox) {
    // Same floating-badge pattern as Home: square, anchored top-left,
    // overflows the topbar strip downward.
    expect(Math.abs(brandBox.width - brandBox.height)).toBeLessThanOrEqual(1);
    expect(brandBox.y + brandBox.height).toBeGreaterThan(
      topbarBox.y + topbarBox.height,
    );
  }
});

test("case studies index: ships the home footer at the bottom of the page", async ({ page }) => {
  await page.goto("/cases");
  const footer = page.locator(".v-panel--footer");
  await footer.scrollIntoViewIfNeeded();
  await expect(footer).toBeVisible();
  await expect(footer).toContainText(/Celebrity Management/i);
});

test("case study detail: also ships the home footer at the bottom of the page", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");
  const footer = page.locator(".v-panel--footer");
  await footer.scrollIntoViewIfNeeded();
  await expect(footer).toBeVisible();
});

test("case page theme toggle: flips html[data-theme] between dark and light", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");
  const html = page.locator("html");
  await expect(html).toHaveAttribute("data-theme", "dark");

  await page.locator(".v-theme-toggle").click();
  await expect(html).toHaveAttribute("data-theme", "light");

  await page.locator(".v-theme-toggle").click();
  await expect(html).toHaveAttribute("data-theme", "dark");
});

test("case detail sidebar: shows Published date and Topics tag links", async ({ page }) => {
  await page.goto("/cases/technova-full-funnel-growth");
  const sidebar = page.locator(".v-case-layout__sidebar");
  await expect(sidebar).toBeVisible();
  await expect(sidebar).toContainText("Published");
  await expect(sidebar).toContainText("Topics");

  const tagLink = sidebar.locator("a.v-tag--green").first();
  await expect(tagLink).toHaveAttribute("href", /^\/cases\/tag\//);
});

test("case detail layout: sidebar and main content stack vertically on narrow viewports", async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 800 });
  await page.goto("/cases/technova-full-funnel-growth");

  const sidebar = page.locator(".v-case-layout__sidebar");
  const main = page.locator(".v-case-layout__main");
  await expect(sidebar).toBeVisible();
  await expect(main).toBeVisible();

  const sidebarBox = await sidebar.boundingBox();
  const mainBox = await main.boundingBox();
  if (sidebarBox && mainBox) {
    // Stacked layout: main content starts below the sidebar, not beside it.
    expect(mainBox.y).toBeGreaterThanOrEqual(sidebarBox.y + sidebarBox.height - 1);
  }
});

test("case study detail: article body scrolls vertically when content overflows", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto("/cases/technova-full-funnel-growth");
  await expect(page.locator(".v-case-article")).toBeVisible();

  // Body must not be locked to 100vh/overflow:hidden the way Home is.
  const bodyOverflow = await page.evaluate(
    () => getComputedStyle(document.body).overflowY,
  );
  expect(["auto", "visible", "scroll"]).toContain(bodyOverflow);

  // Scroll height genuinely exceeds the viewport on a real case study.
  const scrollH = await page.evaluate(() => document.documentElement.scrollHeight);
  const viewportH = await page.evaluate(() => window.innerHeight);
  expect(scrollH).toBeGreaterThan(viewportH);

  // The user can actually scroll the window down to read the rest of the
  // article — the inline "Back to case studies" link sits below the article
  // body and is reachable. (The header back link is a separate copy at the
  // top of the page; pick the in-flow trailing one.)
  const backLink = page.locator(".v-case-layout__main .v-case-page__back");
  await backLink.scrollIntoViewIfNeeded();
  await expect(backLink).toBeInViewport();
  const scrolled = await page.evaluate(() => window.scrollY);
  expect(scrolled).toBeGreaterThan(100);
});
