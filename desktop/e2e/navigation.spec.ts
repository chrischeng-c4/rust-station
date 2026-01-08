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
  try {
    electronApp = await electron.launch({
      args: [path.join(__dirname, '../out/main/index.js')],
      env: {
        ...process.env,
        // Use temp directory for state to avoid polluting real state
        RSTN_DATA_DIR: tempDir,
      },
    })
  } catch (error) {
    console.error('Failed to launch Electron app:', error)
    throw error
  }
})

test.afterAll(async () => {
  if (electronApp) {
    await electronApp.close()
  }

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

  test('shows app structure with navigation layers', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Should show "No Project Open" content initially
    const noProjectView = window.locator('text=No Project Open')
    await expect(noProjectView).toBeVisible({ timeout: 5000 })

    // Should have Open Project button available
    const openButton = window.getByRole('button', { name: 'Open Project' })
    await expect(openButton.first()).toBeVisible()
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
