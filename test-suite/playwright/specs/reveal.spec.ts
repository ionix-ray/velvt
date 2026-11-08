import { expect, test } from "@playwright/test";

/** Effective opacity: every ancestor's opacity multiplied in. A heatmap cell
 * only reads as visible when its `.v-reveal` container is revealed too. */
async function effectiveOpacity(
  locator: ReturnType<import("@playwright/test").Page["locator"]>,
): Promise<number> {
  return locator.evaluate((el) => {
    let node: HTMLElement | null = el as HTMLElement;
    let opacity = 1;
    while (node) {
      opacity *= Number.parseFloat(getComputedStyle(node).opacity);
      node = node.parentElement;
    }
    return opacity;
  });
}

test("reveal: stats container gets the observer class and its cards become visible", async ({
  page,
}) => {
  await page.goto("/#about");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const about = page.locator("#about");
  await about.scrollIntoViewIfNeeded();

  const container = page.locator(".v-heatmap-container");
  await expect(container).toHaveClass(/v-reveal/);
  await container.evaluate((el) => {
    const y = window.scrollY + el.getBoundingClientRect().top - window.innerHeight / 3;
    window.scrollTo(0, y);
  });

  // One mechanism: scroll.rs stamps `is-revealed`, theme.css consumes it.
  await expect
    .poll(async () => (await container.getAttribute("class")) ?? "", {
      timeout: 6000,
    })
    .toContain("is-revealed");

  const statCard = container.locator(".v-about-stat").first();
  await expect
    .poll(async () => effectiveOpacity(statCard), { timeout: 8000 })
    .toBeGreaterThan(0);
});

test("reveal: scroll-in entrance targets are stamped with the observer class", async ({
  page,
}) => {
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const header = page.locator("#ideology .v-panel-header.v-reveal");
  await expect(header).toHaveCount(1);
  await header.scrollIntoViewIfNeeded();

  await expect
    .poll(async () => (await header.getAttribute("class")) ?? "", { timeout: 6000 })
    .toContain("is-revealed");
});

test("reduced motion: stats land visible with animation off, no scroll needed", async ({
  page,
}) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const container = page.locator(".v-heatmap-container");
  expect(await effectiveOpacity(container)).toBeGreaterThan(0);

  const statCard = page.locator(".v-about-stat").first();
  expect(await effectiveOpacity(statCard)).toBeGreaterThan(0);

  // Entrance sections must not be hidden behind a motion gate either.
  const hero = page.locator("#home .v-reveal-left");
  expect(await effectiveOpacity(hero)).toBeGreaterThan(0);
});
