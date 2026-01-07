import { test, expect } from './electron.fixture'
import path from 'path'
import { openProject, getAppState } from './test-helpers'

test.describe('Change Management Workflow', () => {
  let testProjectPath: string

  test.beforeEach(async ({ page }) => {
    // Use the actual rustation project
    testProjectPath = path.resolve(__dirname, '..')
    await openProject(page, testProjectPath)

    // Navigate to Workflows -> Change Management
    await page.getByText('Change Management').click()
    await page.waitForTimeout(1000)
  })

  test('should display Change Management heading', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Change Management' }).first()).toBeVisible()
  })

  test('should show empty state when no changes exist', async ({ page }) => {
    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]
    
    if (worktree?.changes?.changes.length === 0) {
      await expect(page.getByText('No Active Changes')).toBeVisible()
      await expect(page.getByRole('button', { name: 'New Change' })).toBeVisible()
    }
  })

  test('should open New Change dialog', async ({ page }) => {
    await page.getByRole('button', { name: 'New Change' }).click()
    await expect(page.getByText('Propose New Change')).toBeVisible()
    await expect(page.getByPlaceholder('e.g., Add user authentication system')).toBeVisible()
  })

  test('should validate input in New Change dialog', async ({ page }) => {
    await page.getByRole('button', { name: 'New Change' }).click()
    
    const createButton = page.getByRole('button', { name: 'Propose Change' })
    await expect(createButton).toBeDisabled()
    
    await page.getByPlaceholder('e.g., Add user authentication system').fill('Test intent')
    await expect(createButton).toBeEnabled()
  })
})
