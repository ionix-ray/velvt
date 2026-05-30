const { chromium } = require('playwright');
const fs = require('fs');
(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(2000);
  const html = await page.content();
  fs.writeFileSync('page-dump.html', html);
  await browser.close();
})();
