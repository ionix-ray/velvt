const { test, expect } = require('@playwright/test');

const BASE_URL = process.env.BASE_URL || 'http://localhost:8080';

test.describe('Velvet PR Agency - Core Navigation', () => {
  test('home page loads with hero section', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page).toHaveTitle(/Velvet/);
    await expect(page.locator('h1')).toContainText('Velvet');
    await expect(page.locator('#hero')).toBeVisible();
  });

  test('navbar is visible on all pages', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('nav[aria-label="Main navigation"]')).toBeVisible();
    await expect(page.locator('.navbar-logo')).toHaveText('Velvet');
  });

  test('navbar links navigate correctly', async ({ page }) => {
    await page.goto(BASE_URL);
    await page.click('a[href="/services"]');
    await expect(page).toHaveURL(/.*services/);
    await expect(page.locator('h2')).toContainText('Our Services');

    await page.click('a[href="/talent"]');
    await expect(page).toHaveURL(/.*talent/);
    await expect(page.locator('h2')).toContainText('Talent');

    await page.click('a[href="/portfolio"]');
    await expect(page).toHaveURL(/.*portfolio/);
    await expect(page.locator('h2')).toContainText('Portfolio');

    await page.click('a[href="/podcast"]');
    await expect(page).toHaveURL(/.*podcast/);
    await expect(page.locator('h2')).toContainText('Podcast');

    await page.click('a[href="/contact"]');
    await expect(page).toHaveURL(/.*contact/);
    await expect(page.locator('h2')).toContainText('Get in Touch');
  });

  test('footer is visible with navigation links', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('footer')).toBeVisible();
    await expect(page.locator('footer')).toContainText('2026 Velvet PR Agency');
    await expect(page.locator('footer a[href="/services"]')).toBeVisible();
    await expect(page.locator('footer a[href="/contact"]')).toBeVisible();
  });
});

test.describe('Velvet PR Agency - Services Page', () => {
  test('services page renders three cards', async ({ page }) => {
    await page.goto(`${BASE_URL}/services`);
    await expect(page.locator('h2')).toContainText('Our Services');
    await expect(page.locator('.card')).toHaveCount(3);
    await expect(page.locator('.card').first()).toContainText('Public Relations');
    await expect(page.locator('.card').nth(1)).toContainText('Media Relations');
    await expect(page.locator('.card').nth(2)).toContainText('Crisis Communications');
  });
});

test.describe('Velvet PR Agency - Talent Page', () => {
  test('talent page renders roster cards', async ({ page }) => {
    await page.goto(`${BASE_URL}/talent`);
    await expect(page.locator('h2')).toContainText('Talent');
    await expect(page.locator('.card')).toHaveCount(3);
    await expect(page.locator('.card').first()).toContainText('Alexandra Chen');
    await expect(page.locator('.card').nth(1)).toContainText('Marcus Rivera');
    await expect(page.locator('.card').nth(2)).toContainText('Sophie Laurent');
  });
});

test.describe('Velvet PR Agency - Portfolio Page', () => {
  test('portfolio page renders case studies', async ({ page }) => {
    await page.goto(`${BASE_URL}/portfolio`);
    await expect(page.locator('h2')).toContainText('Portfolio');
    await expect(page.locator('.card')).toHaveCount(3);
    await expect(page.locator('.card').first()).toContainText('Global Product Launch');
    await expect(page.locator('.card').nth(1)).toContainText('Crisis Turnaround');
    await expect(page.locator('.card').nth(2)).toContainText('Executive Visibility');
  });
});

test.describe('Velvet PR Agency - Podcast Page', () => {
  test('podcast page shows coming soon', async ({ page }) => {
    await page.goto(`${BASE_URL}/podcast`);
    await expect(page.locator('h2')).toContainText('Podcast');
    await expect(page.locator('.podcast-badge')).toContainText('Coming Soon');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('button')).toContainText('Notify Me');
  });
});

test.describe('Velvet PR Agency - Contact Page', () => {
  test('contact page renders form', async ({ page }) => {
    await page.goto(`${BASE_URL}/contact`);
    await expect(page.locator('h2')).toContainText('Get in Touch');
    await expect(page.locator('input[type="text"]')).toBeVisible();
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('textarea')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toContainText('Send Message');
  });

  test('contact page shows office info', async ({ page }) => {
    await page.goto(`${BASE_URL}/contact`);
    await expect(page.locator('.contact-info')).toContainText('123 Media Lane');
    await expect(page.locator('.contact-info')).toContainText('hello@velvet.pr');
    await expect(page.locator('.social-links')).toContainText('Twitter');
    await expect(page.locator('.social-links')).toContainText('LinkedIn');
  });
});

test.describe('Velvet PR Agency - Accessibility', () => {
  test('page is keyboard navigable', async ({ page }) => {
    await page.goto(BASE_URL);
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    const focused = page.locator(':focus');
    await expect(focused).toBeVisible();
  });

  test('navigation has aria labels', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('nav[aria-label="Main navigation"]')).toBeVisible();
  });

  test('form inputs have labels', async ({ page }) => {
    await page.goto(`${BASE_URL}/contact`);
    await expect(page.locator('label').first()).toBeVisible();
  });
});

test.describe('Velvet PR Agency - Responsive', () => {
  test('mobile viewport (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(BASE_URL);
    await expect(page.locator('h1')).toContainText('Velvet');
    await expect(page.locator('#hero')).toBeVisible();
  });

  test('tablet viewport (768px)', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto(BASE_URL);
    await expect(page.locator('h1')).toContainText('Velvet');
    await expect(page.locator('.services-grid')).toBeVisible();
  });

  test('desktop viewport (1440px)', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(BASE_URL);
    await expect(page.locator('h1')).toContainText('Velvet');
    await expect(page.locator('.services-grid')).toBeVisible();
  });
});

test.describe('Velvet PR Agency - SEO', () => {
  test('page has meta description', async ({ page }) => {
    await page.goto(BASE_URL);
    const metaDescription = page.locator('meta[name="description"]');
    await expect(metaDescription).toHaveAttribute('content', /Premium public relations/);
  });

  test('page has OpenGraph tags', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', /Velvet/);
    await expect(page.locator('meta[property="og:type"]')).toHaveAttribute('content', 'website');
  });

  test('page has Twitter Card tags', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
  });

  test('page has canonical link', async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', /velvet\.pr/);
  });

  test('JSON-LD is present', async ({ page }) => {
    await page.goto(BASE_URL);
    const jsonLd = page.locator('script[type="application/ld+json"]');
    await expect(jsonLd).toHaveCount(1);
    const content = await jsonLd.textContent();
    expect(content).toContain('Organization');
    expect(content).toContain('Velvet PR Agency');
  });
});
