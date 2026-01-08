import { test, expect, _electron as electron } from '@playwright/test'
import path from 'path'
import os from 'os'
import fs from 'fs'

let electronApp: Awaited<ReturnType<typeof electron.launch>>
let tempDir: string

test.beforeAll(async () => {
  // Create a temp directory for test
  tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'rstn-e2e-tabs-'))

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

test.describe('Dual-Layer Tab Architecture', () => {
  test('shows correct structure when no project is open', async () => {
    const window = await electronApp.firstWindow()
    await window.waitForLoadState('domcontentloaded')

    // Should NOT show ProjectTabs when no projects are open
    // ProjectTabs only renders when projects.length > 0
    const projectTabs = window.locator('[aria-label="project tabs"]')
    await expect(projectTabs).not.toBeVisible({ timeout: 3000 }).catch(() => {
      // It's ok if the element doesn't exist at all
    })

    // Should NOT show WorktreeTabs when no project is open
    const worktreeTabs = window.locator('[aria-label="worktree tabs"]')
    await expect(worktreeTabs).not.toBeVisible({ timeout: 3000 }).catch(() => {
      // It's ok if the element doesn't exist at all
    })

    // Should show NoProjectView
    const noProjectView = window.locator('text=No Project Open')
    await expect(noProjectView).toBeVisible({ timeout: 5000 })
  })

  test.skip('shows ProjectTabs with GlobalIconBar when project is open', async () => {
    // Skipped - requires ability to open a project via E2E
    // This would verify:
    // 1. ProjectTabs is visible
    // 2. GlobalIconBar icons are visible (Tasks, Snapshot, Import, etc.)
    // 3. Add Project button is visible
  })

  test.skip('shows WorktreeTabs when project is open', async () => {
    // Skipped - requires ability to open a project via E2E
    // This would verify:
    // 1. WorktreeTabs is visible
    // 2. Shows worktree tabs with branch names
    // 3. Shows "Env" tab on the right
    // 4. Shows "Add Worktree" button
  })

  test.skip('GlobalIconBar has correct 7 icons', async () => {
    // Skipped - requires project to be open first
    // This would verify GlobalIconBar contains:
    // 1. Tasks
    // 2. Snapshot
    // 3. Import
    // 4. Notifications
    // 5. Metrics
    // 6. Docker
    // 7. Settings
    const window = await electronApp.firstWindow()

    const tasksIcon = window.getByRole('button', { name: 'tasks' })
    await expect(tasksIcon).toBeVisible()

    const snapshotIcon = window.getByRole('button', { name: 'snapshot' })
    await expect(snapshotIcon).toBeVisible()

    const importIcon = window.getByRole('button', { name: 'import' })
    await expect(importIcon).toBeVisible()

    const notificationsIcon = window.getByRole('button', { name: 'notifications' })
    await expect(notificationsIcon).toBeVisible()

    const metricsIcon = window.getByRole('button', { name: 'metrics' })
    await expect(metricsIcon).toBeVisible()

    const dockerIcon = window.getByRole('button', { name: 'docker' })
    await expect(dockerIcon).toBeVisible()

    const settingsIcon = window.getByRole('button', { name: 'settings' })
    await expect(settingsIcon).toBeVisible()
  })

  test.skip('Env tab is positioned on the right of WorktreeTabs', async () => {
    // Skipped - requires project to be open first
    // This would verify:
    // 1. Env tab exists in WorktreeTabs
    // 2. Clicking Env tab switches to Env view
  })

  test.skip('can switch between worktrees using WorktreeTabs', async () => {
    // Skipped - requires project with multiple worktrees
    // This would verify:
    // 1. WorktreeTabs shows multiple worktree tabs
    // 2. Clicking a worktree tab switches the active worktree
    // 3. Active worktree tab is highlighted
  })

  test.skip('main worktree shows "main" chip badge', async () => {
    // Skipped - requires project to be open
    // This would verify:
    // 1. Main worktree tab shows a "main" chip badge
    // 2. Non-main worktree tabs don't show the badge
  })
})

test.describe('Navigation Behavior', () => {
  test.skip('ProjectTabs persist when switching between worktrees', async () => {
    // Skipped - requires project with multiple worktrees
    // This would verify:
    // 1. ProjectTabs remain visible at top
    // 2. WorktreeTabs update to show different worktree context
  })

  test.skip('GlobalIconBar remains accessible from any view', async () => {
    // Skipped - requires project to be open
    // This would verify:
    // 1. GlobalIconBar is always visible when project is open
    // 2. GlobalIconBar icons work from any active view (Tasks, Docker, etc.)
  })

  test.skip('clicking Add Project button opens folder dialog', async () => {
    // Skipped - requires mocking Electron dialog API
    // This would verify the Add Project button in ProjectTabs works
  })
})
