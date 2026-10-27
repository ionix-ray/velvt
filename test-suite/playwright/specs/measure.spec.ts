import { test, expect } from '@playwright/test';
test('measure padding', async ({ page }) => {
  await page.goto('/achivements');
  const container = page.locator('.v-case-hero .v-container');
  const box = await container.boundingBox();
  const vp = page.viewportSize();
  console.log(`Viewport: ${vp.width}x${vp.height}`);
  console.log(`Container: x=${box.x}, width=${box.width}`);
  
  const layoutContainer = page.locator('.v-case-page > .v-container').last();
  const box2 = await layoutContainer.boundingBox();
  console.log(`Layout Container: x=${box2.x}, width=${box2.width}`);
});
