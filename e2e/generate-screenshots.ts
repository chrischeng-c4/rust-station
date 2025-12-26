/**
 * Generate documentation screenshots using Playwright
 *
 * Run with: pnpm test:screenshots
 * Prerequisites: pnpm build:debug (build the Electron app)
 */

import { test, expect } from './electron.fixture'
import path from 'path'
import fs from 'fs'

const SCREENSHOT_DIR = path.join(__dirname, '../docs/public/screenshots')

// Ensure screenshot directory exists
test.beforeAll(async () => {
  if (!fs.existsSync(SCREENSHOT_DIR)) {
    fs.mkdirSync(SCREENSHOT_DIR, { recursive: true })
  }
})

test.describe('Documentation Screenshots', () => {
  test('capture main interface', async ({ page }) => {
    // Wait for app to fully load
    await page.waitForTimeout(2000)

    // Full app screenshot
    await page.screenshot({
      path: path.join(SCREENSHOT_DIR, 'main-interface.png'),
      fullPage: false,
    })
  })

  test('capture Tasks tab', async ({ page }) => {
    await page.waitForTimeout(1000)

    // Click Tasks tab in sidebar
    const tasksTab = page.locator('[data-tab="tasks"]').or(page.getByText('Tasks'))
    if (await tasksTab.isVisible()) {
      await tasksTab.click()
      await page.waitForTimeout(500)

      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, 'tasks-tab.png'),
      })
    }
  })

  test('capture Docker tab', async ({ page }) => {
    await page.waitForTimeout(1000)

    // Click Docker tab in sidebar
    const dockerTab = page.locator('[data-tab="docker"]').or(page.getByText('Docker'))
    if (await dockerTab.isVisible()) {
      await dockerTab.click()
      await page.waitForTimeout(500)

      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, 'docker-tab.png'),
      })
    }
  })

  test('capture Settings tab', async ({ page }) => {
    await page.waitForTimeout(1000)

    // Click Settings tab in sidebar
    const settingsTab = page
      .locator('[data-tab="settings"]')
      .or(page.getByText('Settings'))
    if (await settingsTab.isVisible()) {
      await settingsTab.click()
      await page.waitForTimeout(500)

      await page.screenshot({
        path: path.join(SCREENSHOT_DIR, 'settings-tab.png'),
      })
    }
  })

  test('capture Add Worktree dialog', async ({ page }) => {
    await page.waitForTimeout(1000)

    // Find and click the add worktree button
    const addWorktreeButton = page.locator('[data-testid="add-worktree-button"]').or(
      page.locator('button').filter({ hasText: '+' }).first()
    )

    if (await addWorktreeButton.isVisible()) {
      await addWorktreeButton.click()
      await page.waitForTimeout(500)

      // Check if dialog opened
      const dialog = page.locator('[role="dialog"]')
      if (await dialog.isVisible()) {
        await page.screenshot({
          path: path.join(SCREENSHOT_DIR, 'add-worktree-dialog.png'),
        })

        // Close dialog
        await page.keyboard.press('Escape')
      }
    }
  })

  test('capture project tabs', async ({ page }) => {
    await page.waitForTimeout(1000)

    // Screenshot of just the top tab bar area
    const tabBar = page.locator('.tab-bar').or(page.locator('[role="tablist"]').first())
    if (await tabBar.isVisible()) {
      await tabBar.screenshot({
        path: path.join(SCREENSHOT_DIR, 'project-tabs.png'),
      })
    }
  })
})
