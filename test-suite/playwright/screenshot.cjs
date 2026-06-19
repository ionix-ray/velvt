const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  
  await page.goto('http://localhost:8081', { waitUntil: 'domcontentloaded' });
  await page.waitForTimeout(2500); // wait for loader to disappear
  
  // Screenshot Home Panel
  await page.screenshot({ path: '/Users/samirparhi-dev/.gemini/antigravity-ide/brain/4fd8148e-08df-4607-8c23-0621c7169dcf/home-panel.png' });
  
  // Navigate to About Panel (Index 1)
  const dots = page.locator('.v-spindle-item');
  await dots.nth(1).click();
  await page.waitForTimeout(800);
  
  // Screenshot About Panel
  await page.screenshot({ path: '/Users/samirparhi-dev/.gemini/antigravity-ide/brain/4fd8148e-08df-4607-8c23-0621c7169dcf/about-panel.png' });
  
  await browser.close();
})();
