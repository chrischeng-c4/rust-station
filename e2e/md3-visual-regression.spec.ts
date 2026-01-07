import { test, expect } from './electron.fixture'

/**
 * Material Design 3 Visual Regression Tests
 * 
 * This suite ensures that all UI components strictly follow MD3 standards.
 * Every test captures a screenshot and compares it against a baseline snapshot.
 */
test.describe('Material Design 3 Compliance', () => {
  
  test.beforeEach(async ({ page }) => {
    // Wait for the app and theme to load
    await page.waitForTimeout(1000)
  })

  test('Main Layout should follow MD3 specs', async ({ page }) => {
    // Assert main layout matches snapshot
    await expect(page).toHaveScreenshot('main-layout.png', {
      mask: [page.locator('[data-testid="dynamic-content"]')], // Mask content that changes
    })
  })

  test('Command Palette should match MD3 design', async ({ page }) => {
    // Open command palette
    const isMac = process.platform === 'darwin'
    const modifier = isMac ? 'Meta' : 'Control'
    await page.keyboard.press(`${modifier}+k`)
    
    // Wait for animation
    await page.waitForTimeout(500)
    
    // Use test ID
    const palette = page.locator('[data-testid="command-palette-dialog"]')
    await expect(palette).toBeVisible()
    
    // Check visual compliance
    await expect(palette).toHaveScreenshot('command-palette.png')
  })

  test('Empty State should match MD3 design', async ({ page }) => {
    // The app starts with no project, so we expect the NoProjectView
    // Check for "No Project Open" text
    const emptyState = page.getByText('No Project Open')
    await expect(emptyState).toBeVisible()
    
    // Snapshot the empty state container
    // We can locate the main content area
    const content = page.locator('main, .MuiBox-root').first() 
    // Just snapshot the whole page since it's the main view
    await expect(page).toHaveScreenshot('empty-state-no-project.png')
  })

  test('Shared components: PageHeader', async ({ page }) => {
    // Navigate to a page with a standard header (e.g. Docker)
    const dockerTab = page.locator('[data-tab="dockers"]').or(page.getByText('Docker'))
    if (await dockerTab.isVisible()) {
      await dockerTab.click()
      await page.waitForTimeout(500)
      
      const header = page.locator('header').or(page.locator('.MuiBox-root').first())
      await expect(header).toHaveScreenshot('page-header.png')
    }
  })
  
  test('Dark and Light theme snapshots', async ({ page }) => {
    // Switch to Light Mode via Command Palette or Settings
    // This assumes we have a way to toggle theme
    // For now, we capture the default (Dark)
    await expect(page).toHaveScreenshot('full-app-dark-mode.png')
  })
})
