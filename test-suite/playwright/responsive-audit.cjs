// One-off responsive audit: load the page at several viewport widths,
// report horizontal overflow (the thing that causes the dreaded
// "everything scrolls sideways on mobile" bug) and grab a screenshot.
const { chromium } = require("@playwright/test");

const URL = process.env.VAELVET_URL || "http://localhost:8080";
const WIDTHS = [320, 375, 414, 768, 1024, 1280, 1440];

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  let anyOverflow = false;

  for (const width of WIDTHS) {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(URL, { waitUntil: "networkidle" });
    await page.waitForSelector(".v-loader.hidden", { timeout: 15000 }).catch(() => {});
    await page.waitForTimeout(500); // let curtain transition finish

    const result = await page.evaluate(() => {
      const docWidth = document.documentElement.scrollWidth;
      const viewWidth = document.documentElement.clientWidth;
      const overflowing = [];
      if (docWidth > viewWidth + 1) {
        for (const el of document.querySelectorAll("*")) {
          if (el.scrollWidth > viewWidth + 1) {
            overflowing.push({
              tag: el.tagName,
              cls: el.className && el.className.toString().slice(0, 60),
              scrollWidth: el.scrollWidth,
            });
          }
        }
      }
      return { docWidth, viewWidth, overflowing: overflowing.slice(0, 8) };
    });

    const overflow = result.docWidth > result.viewWidth + 1;
    anyOverflow = anyOverflow || overflow;
    console.log(
      `[${width}px] doc=${result.docWidth} view=${result.viewWidth} ${overflow ? "OVERFLOW" : "ok"}`
    );
    if (overflow) {
      for (const el of result.overflowing) {
        console.log(`    -> <${el.tag} class="${el.cls}"> scrollWidth=${el.scrollWidth}`);
      }
    }
    await page.screenshot({ path: `/tmp/velvet-container-${width}.png`, fullPage: false });
  }

  await browser.close();
  process.exit(anyOverflow ? 1 : 0);
})();
