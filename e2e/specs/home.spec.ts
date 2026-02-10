import { test, expect } from "../fixtures";

test.describe("Home Page", () => {
  test.beforeEach(async ({ homePage }) => {
    await homePage.goto();
  });

  test("displays the welcome heading", async ({ homePage }) => {
    await expect(homePage.heading).toBeVisible();
  });

  test("shows the greet form", async ({ homePage }) => {
    await expect(homePage.nameInput).toBeVisible();
    await expect(homePage.greetButton).toBeVisible();
  });
});
