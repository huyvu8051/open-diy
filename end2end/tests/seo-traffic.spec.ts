import { test, expect } from "@playwright/test";

// List of target production domains for traffic generation
const targets = [
  "http://open-diy.unghotoi.asia",
  "http://open-diy.unghotui.vn"
];

for (const baseUrl of targets) {
  test.describe(`Simulate SEO User Traffic - ${baseUrl}`, () => {
    
    test("simulate standard user shopping journey", async ({ page }) => {
      // 1. User enters the homepage
      await page.goto(`${baseUrl}/`);
      await expect(page).toHaveTitle(/open-diy/);
      
      // Scroll down to simulate reading the homepage features
      await page.evaluate(() => window.scrollBy(0, 800));
      await page.waitForTimeout(1500);
      
      // 2. User goes to the About page to read the philosophy
      await page.click("text=About");
      await expect(page).toHaveURL(`${baseUrl}/about`);
      await page.evaluate(() => window.scrollBy(0, 600));
      await page.waitForTimeout(2000);

      // 3. User navigates to the Shop catalog page
      await page.click("text=Shop");
      await expect(page).toHaveURL(`${baseUrl}/shop`);
      await page.waitForTimeout(1000);

      // 4. User selects the Dactyl Split keyboard to customize
      // Clicking on the first "Customize" button (Dactyl)
      const customizeButtons = page.locator("text=Customize");
      await customizeButtons.first().click();
      await expect(page).toHaveURL(new RegExp(`${baseUrl}/builder\\?preset=dactyl`));
      await page.waitForTimeout(1500);

      // 5. User customizes the keyboard options
      // Select Forest Green color
      await page.click("div[title='Forest Green']");
      await page.waitForTimeout(800);
      
      // Select Tactile Brown switches
      await page.click("text=Tactile Brown");
      await page.waitForTimeout(800);

      // Select Pastel Gradient keycaps
      await page.click("text=Pastel Gradient");
      await page.waitForTimeout(1000);

      // Increase quantity to 2
      await page.click("text=+");
      await page.waitForTimeout(500);

      // 6. User adds the custom build to their cart
      await page.click("text=Add Custom Build");
      
      // Verify that the cart drawer is open and contains the custom build
      const cartDrawer = page.locator(".cart-drawer");
      await expect(cartDrawer).toHaveClass(/open/);
      await page.waitForTimeout(2000);

      // 7. User proceeds to checkout
      await page.click("text=Checkout Order");
      await expect(page).toHaveURL(`${baseUrl}/checkout-success`);
      await page.waitForTimeout(3000); // Spend some time on the success page

      // 8. Go back to Home
      await page.click("text=Back to Home Page");
      await expect(page).toHaveURL(`${baseUrl}/`);
    });

    test("simulate quick browse and exit", async ({ page }) => {
      // User lands on homepage, goes to Shop, clicks customize on second option, and leaves
      await page.goto(`${baseUrl}/`);
      await page.waitForTimeout(1000);

      await page.click("text=Shop");
      await page.waitForTimeout(1000);

      const customizeButtons = page.locator("text=Customize");
      await customizeButtons.nth(1).click(); // Frosted 60%
      await expect(page).toHaveURL(new RegExp(`${baseUrl}/builder\\?preset=frosted`));
      await page.waitForTimeout(2000);
    });

  });
}
