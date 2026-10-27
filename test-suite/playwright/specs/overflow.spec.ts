import { test, expect } from '@playwright/test';
test('check horizontal overflow', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 800 });
  await page.goto('/achivements');
  
  const scrollW = await page.evaluate(() => document.documentElement.scrollWidth);
  const viewportW = await page.evaluate(() => window.innerWidth);
  console.log(`scrollW: ${scrollW}, viewportW: ${viewportW}`);
  expect(scrollW).toBeLessThanOrEqual(viewportW);
});
