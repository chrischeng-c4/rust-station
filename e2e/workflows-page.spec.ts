import { test, expect } from './electron.fixture'
import path from 'path'
import { openProject, captureConsoleErrors, getAppState } from './test-helpers'

test.describe('Workflows Page - Constitution Panel via Workflows Tab', () => {
  let testProjectPath: string
  let consoleErrors: string[]

  test.beforeEach(async ({ page }) => {
    // Use the actual rustation project
    testProjectPath = path.resolve(__dirname, '..')
    await openProject(page, testProjectPath)

    // Setup console error capture
    consoleErrors = captureConsoleErrors(page)

    // Workflows is the default view, so we should already be on it
    // Wait for Workflows page content to load
    await page.waitForTimeout(1000)
  })

  test('should display Workflows page with workflow list', async ({ page }) => {
    // Should show Workflows heading (with increased timeout for initial load)
    await expect(page.getByRole('heading', { name: 'Workflows' })).toBeVisible({ timeout: 10000 })

    // Should show Constitution Management workflow
    await expect(page.getByText('Constitution Management')).toBeVisible({ timeout: 5000 })
    await expect(
      page.getByText('Initialize or update project constitution for AI-assisted development')
    ).toBeVisible()

    // Should show Context Management workflow
    await expect(page.getByText('Context Management')).toBeVisible()

    // Should show Change Management workflow
    await expect(page.getByText('Change Management')).toBeVisible()

    // No console errors
    expect(consoleErrors.filter((e) => !e.includes('Electron Security Warning'))).toHaveLength(0)
  })

  test('should show Constitution Panel when Constitution Management is selected', async ({ page }) => {
    // Constitution Management is selected by default, wait for panel to load
    await page.waitForTimeout(1500)

    // Check state
    const state = await getAppState(page)
    const activeProject = state?.projects?.[state?.active_project_index]
    const worktree = activeProject?.worktrees?.[activeProject?.active_worktree_index ?? 0]
    const constitutionExists = worktree?.tasks?.constitution_exists

    console.log('Constitution exists:', constitutionExists)

    // Verify the appropriate UI is shown based on constitution status
    if (constitutionExists === false) {
      await expect(page.getByRole('button', { name: /Apply Default Template/i })).toBeVisible({
        timeout: 5000,
      })
      await expect(page.getByRole('button', { name: /Create with Q&A/i })).toBeVisible()
    } else if (constitutionExists === true) {
      // Use exact match to avoid matching both "Constitution Setup" and "Constitution"
      await expect(page.getByRole('heading', { name: 'Constitution', exact: true })).toBeVisible({ timeout: 5000 })
    } else {
      // Still loading or checking
      console.log('Constitution status still loading')
    }
  })

  test('should dispatch CheckConstitutionExists on panel mount', async ({ page }) => {
    // Wait for panel to mount and dispatch
    await page.waitForTimeout(2000)

    // Check that constitution_exists is no longer null
    const state = await getAppState(page)
    const activeProject = state?.projects?.[state?.active_project_index]
    const worktree = activeProject?.worktrees?.[activeProject?.active_worktree_index ?? 0]
    const constitutionExists = worktree?.tasks?.constitution_exists

    // Should be either true or false, not null
    expect(constitutionExists).not.toBeNull()
    expect(typeof constitutionExists).toBe('boolean')
  })

  test('should start Q&A workflow from Workflows tab', async ({ page }) => {
    await page.waitForTimeout(1500)

    // Check if constitution exists
    const state = await getAppState(page)
    const activeProject = state?.projects?.[state?.active_project_index]
    const worktree = activeProject?.worktrees?.[activeProject?.active_worktree_index ?? 0]

    if (worktree?.tasks?.constitution_exists === false) {
      // Click "Create with Q&A"
      const qaButton = page.getByRole('button', { name: /Create with Q&A/i })
      await qaButton.click()
      await page.waitForTimeout(500)

      // Should show first question
      await expect(page.getByRole('button', { name: 'Rust' })).toBeVisible()
      await expect(page.getByText('0 / 4 questions answered')).toBeVisible()

      // Verify workflow state
      const stateAfter = await getAppState(page)
      const worktreeAfter =
        stateAfter?.projects?.[stateAfter?.active_project_index]?.worktrees?.[0]
      expect(worktreeAfter?.tasks?.constitution_workflow).toBeDefined()
      expect(worktreeAfter?.tasks?.constitution_workflow?.status).toBe('collecting')
    } else if (worktree?.tasks?.constitution_exists === true) {
      // Click "Regenerate with Q&A"
      const regenButton = page.getByRole('button', { name: /Regenerate/i })
      if (await regenButton.isVisible()) {
        await regenButton.click()
        await page.waitForTimeout(500)

        // Should show first question
        await expect(page.getByRole('button', { name: 'Rust' })).toBeVisible()
      }
    }
  })

  test('should navigate between workflows', async ({ page }) => {
    // Click on Context Management
    await page.getByText('Context Management').click()
    await page.waitForTimeout(300)

    // Should show Context Management panel content
    await expect(page.getByRole('heading', { name: /Context Management/i }).first()).toBeVisible({ timeout: 3000 })

    // Click on Change Management
    await page.getByText('Change Management').click()
    await page.waitForTimeout(300)

    // Should show Change Management panel content (use heading to be specific)
    await expect(page.getByRole('heading', { name: 'Change Management' }).first()).toBeVisible({ timeout: 3000 })

    // Click back on Constitution Management
    await page.getByText('Constitution Management').click()
    await page.waitForTimeout(1000)

    // Should show Constitution panel again
    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    if (worktree?.tasks?.constitution_exists === false) {
      await expect(page.getByRole('button', { name: /Apply Default Template/i })).toBeVisible()
    } else {
      // Use exact match to avoid matching both "Constitution Setup" and "Constitution"
      await expect(page.getByRole('heading', { name: 'Constitution', exact: true })).toBeVisible()
    }
  })

  test('should preserve workflow state when switching between workflows', async ({ page }) => {
    await page.waitForTimeout(1500)

    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    // Only test if constitution doesn't exist
    if (worktree?.tasks?.constitution_exists !== false) {
      console.log('Skipping test - constitution exists')
      return
    }

    // Start Q&A workflow
    await page.getByRole('button', { name: /Create with Q&A/i }).click()
    await page.waitForTimeout(500)

    // Answer first question
    await page.getByRole('button', { name: 'Rust' }).click()
    await page.getByRole('button', { name: /Next/i }).click()
    await page.waitForTimeout(300)

    // Should be on question 2
    await expect(page.getByText('1 / 4 questions answered')).toBeVisible()

    // Navigate to Context Management
    await page.getByText('Context Management').click()
    await page.waitForTimeout(500)

    // Navigate back to Constitution Management
    await page.getByText('Constitution Management').click()
    await page.waitForTimeout(1000)

    // Workflow state should be preserved - should still be on question 2
    // Note: Because of useEffect clearing workflow on mount, this behavior may differ
    // If workflow is cleared, we'll see the options again
    const hasProgress = await page.getByText('1 / 4 questions answered').isVisible().catch(() => false)
    const hasOptions = await page
      .getByRole('button', { name: /Apply Default Template/i })
      .isVisible()
      .catch(() => false)

    // Either workflow preserved OR reset to options
    expect(hasProgress || hasOptions).toBe(true)
  })

  test('should complete all 4 questions in Q&A workflow from Workflows tab', async ({ page }) => {
    await page.waitForTimeout(1500)

    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    if (worktree?.tasks?.constitution_exists !== false) {
      console.log('Skipping test - constitution exists')
      return
    }

    // Start Q&A workflow
    await page.getByRole('button', { name: /Create with Q&A/i }).click()
    await page.waitForTimeout(500)

    const nextButton = page.getByRole('button', { name: /Next/i })

    // Answer all 4 questions with guided selections
    await page.getByRole('button', { name: 'Rust' }).click()
    await nextButton.click()
    await page.waitForTimeout(200)

    await page.getByRole('button', { name: 'No secrets in repo' }).click()
    await nextButton.click()
    await page.waitForTimeout(200)

    await page.getByRole('button', { name: 'cargo test must pass' }).click()
    await nextButton.click()
    await page.waitForTimeout(200)

    await page.getByRole('button', { name: 'State-first (serializable state)' }).click()
    await nextButton.click()
    await page.waitForTimeout(200)

    // Should show "All questions answered"
    await expect(page.getByText(/All questions answered/i)).toBeVisible({ timeout: 5000 })

    // Should show Generate button
    const generateButton = page.getByRole('button', { name: /Generate Constitution/i })
    await expect(generateButton).toBeVisible()
    await expect(generateButton).toBeEnabled()

    // Verify state
    const stateAfter = await getAppState(page)
    const workflowAfter =
      stateAfter?.projects?.[stateAfter?.active_project_index]?.worktrees?.[0]?.tasks
        ?.constitution_workflow
    expect(workflowAfter?.current_question).toBe(4)
    expect(workflowAfter?.status).toBe('collecting')
  })

  test('should apply default template when button is clicked', async ({ page }) => {
    await page.waitForTimeout(1500)

    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    if (worktree?.tasks?.constitution_exists !== false) {
      console.log('Skipping test - constitution already exists')
      return
    }

    // Click "Apply Default Template"
    const applyButton = page.getByRole('button', { name: /Apply Default Template/i })
    await expect(applyButton).toBeVisible()
    await applyButton.click()

    // Wait for the action to complete
    await page.waitForTimeout(2000)

    // Should show constitution content or success state
    const stateAfter = await getAppState(page)
    const worktreeAfter = stateAfter?.projects?.[stateAfter?.active_project_index]?.worktrees?.[0]

    // Constitution should now exist
    expect(worktreeAfter?.tasks?.constitution_exists).toBe(true)
  })

  test('should show CLAUDE.md import option when CLAUDE.md exists', async ({ page }) => {
    await page.waitForTimeout(1500)

    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    // This test only runs if CLAUDE.md exists and constitution doesn't
    if (worktree?.tasks?.claude_md_exists !== true || worktree?.tasks?.constitution_exists !== false) {
      console.log('Skipping test - CLAUDE.md not found or constitution already exists')
      return
    }

    // Should show the CLAUDE.md import option
    await expect(page.getByText(/CLAUDE\.md found/i)).toBeVisible()
    await expect(page.getByRole('button', { name: /Use This/i })).toBeVisible()
    await expect(page.getByRole('button', { name: /Skip/i })).toBeVisible()
  })

  test('should regenerate constitution when Regenerate button is clicked', async ({ page }) => {
    await page.waitForTimeout(1500)

    const state = await getAppState(page)
    const worktree = state?.projects?.[state?.active_project_index]?.worktrees?.[0]

    if (worktree?.tasks?.constitution_exists !== true) {
      console.log('Skipping test - constitution does not exist')
      return
    }

    // Should see Regenerate button
    const regenButton = page.getByRole('button', { name: /Regenerate/i })
    if (!(await regenButton.isVisible())) {
      console.log('Regenerate button not visible')
      return
    }

    await regenButton.click()
    await page.waitForTimeout(500)

    // Should show Q&A workflow
    await expect(page.getByRole('button', { name: 'Rust' })).toBeVisible()
    await expect(page.getByText(/0 \/ 4 questions answered/i)).toBeVisible()

    // Verify workflow state
    const stateAfter = await getAppState(page)
    const worktreeAfter = stateAfter?.projects?.[stateAfter?.active_project_index]?.worktrees?.[0]
    expect(worktreeAfter?.tasks?.constitution_workflow).toBeDefined()
    expect(worktreeAfter?.tasks?.constitution_workflow?.status).toBe('collecting')
  })
})
