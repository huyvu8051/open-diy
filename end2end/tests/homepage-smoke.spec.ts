import { test, expect } from "@playwright/test";

const BASE_URL = process.env.BASE_URL || "https://shop.opendiy.vn";

test.describe("Homepage Smoke Tour", () => {
  test.beforeEach(async ({ page }) => {
    // 3) Use page.route to ignore known tracker/analytics hosts and unhandled unknown URLs so the run stays stable
    await page.route("**/*", (route) => {
      const url = route.request().url();
      
      // Ignore/abort known tracker/analytics hosts
      if (
        url.includes("plausible.io") ||
        url.includes("umami") ||
        url.includes("plausible") ||
        url.includes("google-analytics") ||
        url.includes("analytics")
      ) {
        return route.abort();
      }

      // Allow the live application URL, local environments, data URIs, and expected asset CDNs
      if (
        url.startsWith(BASE_URL) ||
        url.startsWith("data:") ||
        url.includes("localhost") ||
        url.includes("fal.media") ||
        url.includes("fonts.googleapis.com") ||
        url.includes("fonts.gstatic.com")
      ) {
        return route.continue();
      }

      // Abort other unhandled unknown URLs to keep the run stable
      return route.abort();
    });
  });

  test("tours the live app without breaking on trackers", async ({ page }) => {
    // 1) Visit homepage
    await page.goto(BASE_URL);

    // 4) Assert existing class name on homepage: .hero-title
    const heroTitle = page.locator(".hero-title");
    await expect(heroTitle).toBeVisible();

    // 2) Go to /shop
    const shopLink = page.locator('a[href="/shop"]').first();
    await shopLink.click();
    await expect(page).toHaveURL(new RegExp("/shop"));

    // Assert existing class name on shop: .section-title
    const sectionTitle = page.locator(".section-title");
    await expect(sectionTitle).toBeVisible();

    // Click the first product
    const firstProduct = page.locator(".product-card a").first();
    await firstProduct.click();
    await expect(page).toHaveURL(new RegExp("/product/"));

    // Open the AI chat bubble
    const messengerBtn = page.locator(".messenger-float-btn");
    await expect(messengerBtn).toBeVisible();
    await messengerBtn.click();

    // Assert AI chat window class: .ai-chat-window
    const chatWindow = page.locator(".ai-chat-window");
    await expect(chatWindow).toBeVisible();

    // Find input[type=text] inside the chat window/footer and send 'Xin chào'
    const chatInput = chatWindow.locator("input[type=text]");
    await expect(chatInput).toBeVisible();
    await chatInput.fill("Xin chào");
    await chatInput.press("Enter");

    // Close the bubble
    await messengerBtn.click();
    await expect(chatWindow).not.toBeVisible();

    // Visit /about
    const aboutLink = page.locator('a[href="/about"]').first();
    await aboutLink.click();
    await expect(page).toHaveURL(new RegExp("/about"));

    // Assert the footer link https://www.youtube.com/@opendiyvn is present
    const footerYoutubeLink = page.locator('a[href="https://www.youtube.com/@opendiyvn"]');
    await expect(footerYoutubeLink).toBeVisible();
  });
});
