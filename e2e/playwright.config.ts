import { defineConfig } from "@playwright/test";
import { devices } from "@playwright/test";

export default defineConfig({
  testDir: "tests",
  timeout: (process.env.CI ? 60 : 30) * 1000,
  expect: { timeout: 5000 },
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["html", { open: "never" }]],
  use: { actionTimeout: 0, trace: "on-first-retry" },
  projects: [
    { name: "chromium", use: devices["Desktop Chrome"] },
    { name: "firefox", use: devices["Desktop Firefox"] },
    { name: "webkit", use: devices["Desktop Safari"] },
  ],
  webServer: {
    command: "just serve-release",
    port: 3333,
    timeout: 1000 * 60 * 10,
  },
});
