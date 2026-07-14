import { test } from '@playwright/test';
test('screenshot achievements', async ({ page }) => {
  await page.goto('http://localhost:8080/achivements');
  await page.screenshot({ path: '/Users/samirparhi-dev/.gemini/antigravity-ide/brain/e7e3c025-cc80-4690-819b-a5b87c1cb557/achivements.png', fullPage: true });
});
