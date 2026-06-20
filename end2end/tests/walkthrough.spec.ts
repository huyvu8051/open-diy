import { test, expect } from "@playwright/test";

const BASE_URL = process.env.BASE_URL || "http://localhost:3000";

test.describe("open-diy Webapp E2E Walkthrough", () => {
  test.beforeEach(async ({ page }) => {
    // Listen for console logs and runtime errors
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', err => console.error('PAGE ERROR:', err.message));

    // Abort known external trackers/analytics to ensure stable test environment
    await page.route("**/*", (route) => {
      const url = route.request().url();
      if (
        url.includes("plausible.io") ||
        url.includes("umami") ||
        url.includes("google-analytics") ||
        url.includes("analytics")
      ) {
        return route.abort();
      }
      return route.continue();
    });
  });

  test("should successfully walk through all webapp pages and features", async ({ page }) => {
    // 1. Visit homepage
    await page.goto(BASE_URL);
    await expect(page).toHaveTitle(/open-diy/);
    
    // Wait for WASM/JS hydration
    await page.waitForTimeout(2000);
    
    const heroTitle = page.locator(".hero-title");
    await expect(heroTitle).toBeVisible();

    // 2. Test Theme Toggle
    const themeBtn = page.locator(".theme-toggle-btn");
    await expect(themeBtn).toBeVisible();
    
    const htmlElement = page.locator("html");
    // Assert light/dark toggling works by checking class presence
    await themeBtn.click();
    await expect(htmlElement).not.toHaveClass(/light/); // toggled off light

    await themeBtn.click();
    await expect(htmlElement).toHaveClass(/light/); // toggled back to light

    // 3. Test Language Selector (Desktop)
    const enButton = page.locator('text="EN"');
    const viButton = page.locator('text="VI"');
    
    await expect(enButton).toBeVisible();
    await expect(viButton).toBeVisible();

    // Toggle English
    await enButton.click();
    const shopLinkEn = page.locator('a[href="/shop"]').first();
    await expect(shopLinkEn).toHaveText("Shop");

    // Toggle Vietnamese
    await viButton.click();
    const shopLinkVi = page.locator('a[href="/shop"]').first();
    await expect(shopLinkVi).toHaveText("Cửa hàng");

    // 4. Navigate to Catalog Page
    await shopLinkVi.click();
    await expect(page).toHaveURL(new RegExp("/shop"));
    
    const sectionTitle = page.locator(".section-title");
    await expect(sectionTitle).toBeVisible();

    // 5. Navigate to Product Detail Page (first product card)
    const firstProduct = page.locator(".product-card a").first();
    await firstProduct.click();
    await expect(page).toHaveURL(new RegExp("/product/"));

    // Verify detail page elements
    const productTitle = page.locator("h1.gradient-text");
    await expect(productTitle).toBeVisible();

    // Verify shoppee button exists on product detail
    const shopeeBtn = page.locator('a[href^="https://shopee.vn"]').first();
    await expect(shopeeBtn).toBeVisible();

    // 6. Test AI Customer Service Assistant
    const messengerBtn = page.locator(".messenger-float-btn");
    await expect(messengerBtn).toBeVisible();
    await messengerBtn.click();

    // Verify AI Chat Window open
    const chatWindow = page.locator(".ai-chat-window");
    await expect(chatWindow).toBeVisible();

    // Input message
    const chatInput = chatWindow.locator("input[type=text]");
    await expect(chatInput).toBeVisible();
    await chatInput.fill("corne");
    await chatInput.press("Enter");

    // Wait for messages to populate and check for Corne text in the chat window contents
    const messagesContainer = chatWindow.locator("#ai-chat-messages > div");
    await expect(messagesContainer.first()).toBeVisible();
    // Expect 3 messages: 1 (initial welcome) + 1 (user message) + 1 (AI response)
    await expect(messagesContainer).toHaveCount(3, { timeout: 10000 });
    const chatText = await chatWindow.innerText();
    expect(chatText.toLowerCase()).toContain("corne");

    // Close AI Chat Window
    await messengerBtn.click();
    await expect(chatWindow).not.toBeVisible();

    // 7. Navigate to About Page
    const aboutLink = page.locator('a[href="/about"]').first();
    await aboutLink.click();
    await expect(page).toHaveURL(new RegExp("/about"));

    // Verify About Page Content
    const aboutHeader = page.locator("h1");
    await expect(aboutHeader.first()).toBeVisible();
  });
});
