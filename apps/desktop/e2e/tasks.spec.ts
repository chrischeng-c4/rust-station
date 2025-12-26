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
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-tasks-'))
  projectDir = path.join(tempDir, 'test-project')
  fs.mkdirSync(projectDir, { recursive: true })

  // Initialize a git repo with a justfile
  try {
    execSync('git init', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.email "test@test.com"', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.name "Test"', { cwd: projectDir, stdio: 'ignore' })

    // Create a justfile with some commands
    const justfileContent = `# Test justfile

# Build the project
build:
    echo "Building..."

# Run tests
test:
    echo "Testing..."

# Clean build artifacts
clean:
    echo "Cleaning..."
`
    fs.writeFileSync(path.join(projectDir, 'justfile'), justfileContent)
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

test.describe('Tasks Tab', () => {
  test.skip('shows Tasks tab in sidebar when project is open', async () => {
    // This test is skipped because it requires opening a project
    // which needs dialog API mocking
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to open project first
    const tasksTab = window.locator('text=Tasks')
    await expect(tasksTab).toBeVisible()
  })

  test.skip('shows justfile commands when project has justfile', async () => {
    // Skipped - requires project to be opened
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to:
    // 1. Open project with justfile
    // 2. Navigate to Tasks tab
    // 3. Verify commands are listed
  })

  test.skip('shows no justfile message when project lacks justfile', async () => {
    // Skipped - requires project to be opened
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to:
    // 1. Open project without justfile
    // 2. Navigate to Tasks tab
    // 3. Verify "No justfile found" message
  })
})

test.describe('Task Execution', () => {
  test.skip('can run a task and see output', async () => {
    // Skipped - requires project to be opened and just to be installed
  })

  test.skip('shows task status while running', async () => {
    // Skipped - requires project to be opened and just to be installed
  })

  test.skip('shows success indicator when task completes', async () => {
    // Skipped - requires project to be opened and just to be installed
  })

  test.skip('shows error indicator when task fails', async () => {
    // Skipped - requires project to be opened and just to be installed
  })
})
