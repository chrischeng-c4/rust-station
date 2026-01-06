import { test, expect, _electron as electron } from '@playwright/test'
import path from 'path'
import os from 'os'
import fs from 'fs'
import { execSync } from 'child_process'

let electronApp: Awaited<ReturnType<typeof electron.launch>>
let tempDir: string
let projectDir: string

test.beforeAll(async () => {
  // Create a temp directory for test projects
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-project-'))
  projectDir = path.join(tempDir, 'test-project')
  fs.mkdirSync(projectDir, { recursive: true })

  // Initialize a git repo
  try {
    execSync('git init', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.email "test@test.com"', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.name "Test"', { cwd: projectDir, stdio: 'ignore' })
    fs.writeFileSync(path.join(projectDir, 'README.md'), '# Test Project\n')
    execSync('git add . && git commit -m "init"', { cwd: projectDir, stdio: 'ignore' })
  } catch {
    // Git initialization might fail in some environments
  }

  // Build and launch the app
  electronApp = await electron.launch({
    args: [path.join(__dirname, '../out/main/index.js')],
    env: {
      ...process.env,
      RSTN_DATA_DIR: tempDir,
    },
  })
})

test.afterAll(async () => {
  await electronApp.close()

  try {
    fs.rmSync(tempDir, { recursive: true, force: true })
  } catch {
    // Ignore cleanup errors
  }
})

test.describe('Project Management', () => {
  test('shows empty state when no project is open', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    const noProjectText = window.locator('text=No Project Open')
    await expect(noProjectText).toBeVisible({ timeout: 10000 })
  })

  test('has Open Project button in empty state', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    const openButton = window.getByRole('button', { name: 'Open Project' })
    await expect(openButton.first()).toBeVisible()
  })

  test.skip('opens project when folder is selected', async () => {
    // Skipped - requires mocking the Electron dialog API
    // The dialog.showOpenDialog cannot be easily mocked in E2E tests
  })

  test.skip('shows project in tabs after opening', async () => {
    // Skipped - requires project to be opened first
  })

  test.skip('can switch between multiple projects', async () => {
    // Skipped - requires multiple projects to be opened
  })

  test.skip('can close a project', async () => {
    // Skipped - requires project to be opened first
  })
})

test.describe('Recent Projects', () => {
  test.skip('shows recent projects in dropdown after opening a project', async () => {
    // Skipped - requires project history
  })

  test.skip('can open recent project from dropdown', async () => {
    // Skipped - requires project history
  })

  test.skip('filters out already open projects from recent list', async () => {
    // Skipped - requires multiple projects in history
  })
})
