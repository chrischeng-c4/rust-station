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
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-worktree-'))
  projectDir = path.join(tempDir, 'test-project')
  fs.mkdirSync(projectDir, { recursive: true })

  // Initialize a git repo with multiple worktrees
  try {
    execSync('git init', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.email "test@test.com"', { cwd: projectDir, stdio: 'ignore' })
    execSync('git config user.name "Test"', { cwd: projectDir, stdio: 'ignore' })
    fs.writeFileSync(path.join(projectDir, 'README.md'), '# Test Project\n')
    execSync('git add . && git commit -m "init"', { cwd: projectDir, stdio: 'ignore' })

    // Create a branch for worktree testing
    execSync('git branch feature-branch', { cwd: projectDir, stdio: 'ignore' })
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

test.describe('Worktree Management', () => {
  test.skip('shows worktree tabs when project has multiple worktrees', async () => {
    // Skipped - requires project with multiple git worktrees
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Would need to:
    // 1. Open a project with multiple worktrees
    // 2. Verify worktree tabs are visible
  })

  test.skip('hides worktree tabs when project has single worktree', async () => {
    // Skipped - requires project with single worktree
  })

  test.skip('can switch between worktrees', async () => {
    // Skipped - requires project with multiple worktrees
  })

  test.skip('shows current branch name in worktree tab', async () => {
    // Skipped - requires project to be opened
  })

  test.skip('shows modified indicator for worktrees with uncommitted changes', async () => {
    // Skipped - requires project with uncommitted changes
  })
})

test.describe('Worktree State', () => {
  test.skip('preserves active tab per worktree', async () => {
    // Skipped - requires multiple worktrees
    // Each worktree should remember which tab (Tasks/Docker/Settings) was last active
  })

  test.skip('preserves selected service per worktree in Docker tab', async () => {
    // Skipped - requires multiple worktrees
  })

  test.skip('preserves task output per worktree in Tasks tab', async () => {
    // Skipped - requires multiple worktrees
  })
})
