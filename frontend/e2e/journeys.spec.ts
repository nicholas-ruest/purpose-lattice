import { test, expect } from "@playwright/test";
test("safe plan reaches real backend and renders result", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("Support case corpus")).toBeVisible();
  await page.getByRole("button", { name: "Evaluate safe plan" }).click();
  await expect(
    page.getByText("Plan allowed and appended to RVF"),
  ).toBeVisible();
  await expect(page.locator(".result")).toContainText("allow");
  await page.screenshot({
    path: "../docs/assets/desktop-evaluation.png",
    fullPage: true,
  });
});
test("policy denial has no forbidden effect and corrected plan recovers", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Try forbidden export" }).click();
  await expect(page.getByRole("alert")).toContainText("policy denied");
  await expect(
    page.getByText("Policy denial confirmed; no plan was appended"),
  ).toBeVisible();
  await page.getByRole("button", { name: "Evaluate safe plan" }).click();
  await expect(
    page.getByText("Plan allowed and appended to RVF"),
  ).toBeVisible();
});
test("audit survives backend restart", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Audit" }).click();
  await expect(page.getByTestId("audit-json")).toContainText("covenant");
  await page.reload();
  await page.getByRole("button", { name: "Audit" }).click();
  await expect(page.getByTestId("audit-json")).toContainText("evaluation");
  await page.setViewportSize({ width: 390, height: 844 });
  await page.screenshot({
    path: "../docs/assets/mobile-audit.png",
    fullPage: true,
  });
});
