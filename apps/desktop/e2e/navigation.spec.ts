import { test, expect, _electron as electron } from '@playwright/test'
import path from 'path'
import os from 'os'
import fs from 'fs'

let electronApp: Awaited<ReturnType<typeof electron.launch>>
let tempDir: string

test.beforeAll(async () => {
  // Create a temp directory for test projects
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-'))

  // Initialize a git repo to simulate a project
  const projectDir = path.join(tempDir, 'test-project')
  fs.mkdirSync(projectDir, { recursive: true })

  // Build and launch the app
  electronApp = await electron.launch({
    args: [path.join(__dirname, '../out/main/index.js')],
    env: {
      ...process.env,
      // Use temp directory for state to avoid polluting real state
      RSTN_DATA_DIR: tempDir,
    },
  })
})

test.afterAll(async () => {
  await electronApp.close()

  // Cleanup temp directory
  try {
    fs.rmSync(tempDir, { recursive: true, force: true })
  } catch {
    // Ignore cleanup errors
  }
})

test.describe('Navigation', () => {
  test('shows Open Project button initially', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    const openButton = window.getByRole('button', { name: 'Open Project' })
    await expect(openButton.first()).toBeVisible({ timeout: 10000 })
  })

  test('shows project tabs area', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // The top area should have project tabs or open project button
    const projectTabsArea = window.locator('.flex.flex-col.border-b')
    await expect(projectTabsArea).toBeVisible({ timeout: 5000 })
  })

  test('shows no project view content', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Should show the no project view
    const noProjectHeading = window.locator('text=No Project Open')
    await expect(noProjectHeading).toBeVisible({ timeout: 10000 })

    const instructions = window.locator('text=Open a project folder to get started')
    await expect(instructions).toBeVisible()
  })
})
