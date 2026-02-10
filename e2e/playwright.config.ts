import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./specs",

  fullyParallel: true,

  // Fail CI if test.only is left in source
  forbidOnly: !!process.env.CI,

  retries: process.env.CI ? 2 : 0,

  workers: process.env.CI ? 4 : undefined,

  reporter: process.env.CI
    ? [["html", { open: "never" }], ["github"]]
    : [["html", { open: "on-failure" }], ["line"]],

  timeout: 30_000,

  expect: {
    timeout: 5_000,
  },

  use: {
    baseURL: "http://localhost:5173",
    trace: "on-first-retry",
    screenshot: "only-on-failure",
    video: "on-first-retry",
  },

  // Tauri's WebView2 is Chromium-based — one browser is sufficient
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],

  webServer: {
    command: "npm run dev",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
