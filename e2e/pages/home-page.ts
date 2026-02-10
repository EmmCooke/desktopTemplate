import type { Page, Locator } from "@playwright/test";

export class HomePage {
  readonly page: Page;
  readonly heading: Locator;
  readonly nameInput: Locator;
  readonly greetButton: Locator;
  readonly greetingMessage: Locator;

  constructor(page: Page) {
    this.page = page;
    this.heading = page.getByRole("heading", { name: "Welcome" });
    this.nameInput = page.getByLabel("Name");
    this.greetButton = page.getByRole("button", { name: "Greet" });
    this.greetingMessage = page.getByRole("status");
  }

  async goto() {
    await this.page.goto("/");
  }

  async greet(name: string) {
    await this.nameInput.fill(name);
    await this.greetButton.click();
  }
}
