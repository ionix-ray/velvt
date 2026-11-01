import { test } from '@playwright/test';
test('screenshot achievements', async ({ page }) => {
  await page.goto('/achivements');
  await page.screenshot({ path: 'test-results/achivements.png', fullPage: true });
});
