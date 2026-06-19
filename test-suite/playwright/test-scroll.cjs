const { chromium } = require('playwright');
(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  
  // Expose a function to get scroll
  await page.exposeFunction('logScroll', (s) => console.log('Scroll:', s));

  await page.goto('http://localhost:8080');
  await page.waitForSelector('.v-panels');
  
  // Wait for loader to disappear
  await page.waitForSelector('.v-loader', { state: 'hidden' });

  // Focus body
  await page.evaluate(() => document.body.focus());
  
  console.log('Pressing ArrowRight...');
  await page.keyboard.press('ArrowRight');
  
  // Wait 1 second
  await page.waitForTimeout(1000);
  
  const scrollLeft = await page.evaluate(() => {
    const p = document.querySelector('.v-panels');
    return p ? p.scrollLeft : null;
  });
  console.log('scrollLeft after ArrowRight:', scrollLeft);
  
  // Let's also check if current_panel state updated
  const activeDot = await page.evaluate(() => {
    const dot = document.querySelector('.v-spindle-item.active');
    return dot ? dot.textContent : null;
  });
  console.log('Active spindle item text:', activeDot);

  await browser.close();
})();
