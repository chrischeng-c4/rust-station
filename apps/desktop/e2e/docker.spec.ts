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
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-docker-'))
  projectDir = path.join(tempDir, 'test-project')
  fs.mkdirSync(projectDir, { recursive: true })

  // Initialize a git repo to simulate a project
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

test.describe('Docker Tab', () => {
  test.skip('shows Docker tab in sidebar when project is open', async () => {
    // This test is skipped because it requires a project to be open
    // and the dialog API mocking is complex in Electron E2E tests
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to open a project first
    // Then verify Docker tab is visible
    const dockerTab = window.locator('text=Docker')
    await expect(dockerTab).toBeVisible()
  })

  test.skip('shows Docker unavailable message when Docker is not running', async () => {
    // This test is skipped because it depends on Docker state
    // and requires project to be open first
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to:
    // 1. Open a project
    // 2. Navigate to Docker tab
    // 3. Verify Docker unavailable message if Docker is not running
  })

  test.skip('shows Docker services when Docker is available', async () => {
    // This test is skipped because it requires actual Docker to be running
    // with rstn services
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to:
    // 1. Open a project
    // 2. Navigate to Docker tab
    // 3. Verify service list is visible
  })
})

test.describe('Docker Service Interactions', () => {
  test.skip('can refresh Docker services', async () => {
    // Skipped - requires project open and Docker available
  })

  test.skip('can view service logs', async () => {
    // Skipped - requires project open and Docker available
  })

  test.skip('can start/stop services', async () => {
    // Skipped - requires project open and Docker available with actual services
  })
})
