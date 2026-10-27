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

test("reveal: heatmap container gets the observer class and its cells become visible", async ({
  page,
}) => {
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const container = page.locator(".v-heatmap-container");
  await expect(container).toHaveClass(/v-reveal/);
  await container.scrollIntoViewIfNeeded();

  // One mechanism: scroll.rs stamps `is-revealed`, theme.css consumes it.
  await expect
    .poll(async () => (await container.getAttribute("class")) ?? "", {
      timeout: 6000,
    })
    .toContain("is-revealed");

  const cell = container.locator(".v-heatmap-cell").first();
  await expect
    .poll(async () => effectiveOpacity(cell), { timeout: 8000 })
    .toBeGreaterThan(0);

  // Active (VELVT letter) cells land on the crimson brand colour.
  const active = container.locator(".v-heatmap-cell--active").first();
  await expect
    .poll(async () => active.evaluate((el) => getComputedStyle(el).backgroundColor), {
      timeout: 8000,
    })
    .toMatch(/181,\s*42,\s*42|212,\s*62,\s*62/);
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

test("reduced motion: heatmap lands visible with animation off, no scroll needed", async ({
  page,
}) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  await page.waitForSelector(".v-loader", { state: "hidden" });

  const cell = page.locator(".v-heatmap-cell").first();
  expect(await effectiveOpacity(cell)).toBeGreaterThan(0);

  const active = page.locator(".v-heatmap-cell--active").first();
  await expect(active).toHaveCSS("animation-name", "none");

  // Entrance sections must not be hidden behind a motion gate either.
  const hero = page.locator("#home .v-reveal-left");
  expect(await effectiveOpacity(hero)).toBeGreaterThan(0);
});
