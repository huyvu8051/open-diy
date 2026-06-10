import { test, expect } from "@playwright/test";

test.describe("open-diy E2E Test Suite", () => {
  
  test("homepage loads with correct title and header", async ({ page }) => {
    await page.goto("http://localhost:3000/");

    // Check document title
    await expect(page).toHaveTitle("open-diy | Premium 3D Printed Keyboards");

    // Check main hero title text
    const heroTitle = page.locator(".hero-title");
    await expect(heroTitle).toContainText("Fully Custom");
    await expect(heroTitle).toContainText("3D Printed Keyboards");
  });

  test("navigation works between pages", async ({ page }) => {
    await page.goto("http://localhost:3000/");

    // Click "Shop" in the navbar
    await page.click("text=Shop");
    await expect(page).toHaveURL("http://localhost:3000/shop");
    await expect(page.locator(".section-title")).toContainText("Our Curated Builds");

    // Click "About" in the navbar
    await page.click("text=About");
    await expect(page).toHaveURL("http://localhost:3000/about");
    await expect(page.locator(".about-header h1")).toHaveText("Open-Source Keyboards");
  });

  test("interactive keyboard customizer updates pricing and adds to cart", async ({ page }) => {
    await page.goto("http://localhost:3000/builder");

    // Check default preview spec summary
    await expect(page.locator(".preview-overlay-specs")).toContainText("Layout: dactyl");
    
    // Default price check (Base $219 + Default Case $0 + Default Switch $0 + Keycap Addon $20 = $239)
    const totalCost = page.locator(".summary-row.total span").last();
    await expect(totalCost).toHaveText("$239.00");

    // Change layout to Frosted 60% (Base $109)
    await page.click("text=Frosted 60% Compact");
    
    // Change case color to Frosted Glass (+$15)
    await page.click("div[title='Frosted Glass']");

    // Change switch to Custom Silent (+$20)
    await page.click("text=Custom Silent");

    // Change keycaps to Pastel Gradient (+$25)
    await page.click("text=Pastel Gradient");

    // Total should be Base $109 + Color $15 + Switch $20 + Keycap $25 = $169.00
    await expect(totalCost).toHaveText("$169.00");

    // Add to cart
    await page.click("text=Add Custom Build");

    // Cart drawer should be visible (has open class)
    const cartDrawer = page.locator(".cart-drawer");
    await expect(cartDrawer).toHaveClass(/open/);

    // Verify item in cart
    const cartItem = page.locator(".cart-item");
    await expect(cartItem).toBeVisible();
    await expect(cartItem.locator(".cart-item-name")).toHaveText("open-diy Frosted 60%");
    await expect(cartItem.locator(".cart-item-specs")).toContainText("Layout: Frosted 60%");
    await expect(cartItem.locator(".cart-item-specs")).toContainText("Color: Frosted Glass");
    await expect(cartItem.locator(".cart-item-specs")).toContainText("Switches: Custom Silent");

    // Checkout
    await page.click("text=Checkout Order");

    // Verify redirect to success page
    await expect(page).toHaveURL("http://localhost:3000/checkout-success");
    await expect(page.locator(".checkout-page h1")).toHaveText("Order Received!");
  });
});
