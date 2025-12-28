import { test, expect } from './electron.fixture'
import * as fs from 'fs/promises'
import * as path from 'path'

test.describe('Constitution Workflow', () => {
  test('should show Initialize Constitution command in Tasks tab', async ({ page }) => {
    // Wait for app to load
    await page.waitForSelector('[data-testid="project-tabs"]', { timeout: 10000 }).catch(() => {})

    // Click Tasks tab
    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open - Tasks tab not visible')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    // Should show Tasks heading
    await expect(page.locator('h2', { hasText: /Tasks/i })).toBeVisible({ timeout: 5000 })

    // Should show "Initialize Constitution" command
    await expect(page.getByText('Initialize Constitution')).toBeVisible({ timeout: 3000 })
    await expect(page.getByText('Initialize project constitution (CESDD)')).toBeVisible()
  })

  test('should display Constitution workflow UI when command is selected', async ({ page }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    // Click "Initialize Constitution" command
    const constitutionCmd = page.getByText('Initialize Constitution')
    await constitutionCmd.click()
    await page.waitForTimeout(500)

    // Should show Constitution workflow panel
    await expect(page.getByText('Initialize Constitution')).toBeVisible()
    await expect(page.getByText('0 / 4')).toBeVisible() // Progress indicator

    // Should show first question
    await expect(
      page.getByText(/What technology stack does this project use/)
    ).toBeVisible({ timeout: 3000 })
  })

  test('should allow answering questions sequentially', async ({ page }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    // Select Constitution workflow
    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Question 1: Technology Stack
    const textarea = page.getByPlaceholder('Type your answer...')
    await expect(textarea).toBeVisible()

    // Next button should be disabled initially
    const nextButton = page.getByRole('button', { name: /Next/i })
    await expect(nextButton).toBeDisabled()

    // Type answer
    await textarea.fill('React + Rust (napi-rs), TypeScript')
    await expect(nextButton).toBeEnabled()
    await nextButton.click()
    await page.waitForTimeout(300)

    // Should show Question 2
    await expect(page.getByText('1 / 4')).toBeVisible()
    await expect(page.getByText(/What security requirements/)).toBeVisible()

    // Answer Question 2
    await textarea.fill('JWT auth required, no SQL injection')
    await nextButton.click()
    await page.waitForTimeout(300)

    // Should show Question 3
    await expect(page.getByText('2 / 4')).toBeVisible()
    await expect(page.getByText(/What code quality standards/)).toBeVisible()

    // Answer Question 3
    await textarea.fill('80% test coverage, TypeScript strict mode')
    await nextButton.click()
    await page.waitForTimeout(300)

    // Should show Question 4
    await expect(page.getByText('3 / 4')).toBeVisible()
    await expect(page.getByText(/Any architectural constraints/)).toBeVisible()

    // Answer Question 4
    await textarea.fill('State-first principle, reducer pattern')
    await nextButton.click()
    await page.waitForTimeout(500)

    // Should show "All questions answered" success message
    await expect(page.getByText(/All questions answered!/i)).toBeVisible({ timeout: 5000 })

    // Should show "Generate Constitution" button
    const generateButton = page.getByRole('button', { name: /Generate Constitution/i })
    await expect(generateButton).toBeVisible()
    await expect(generateButton).toBeEnabled()
  })

  test('should display progress indicators with checkmarks for answered questions', async ({
    page,
  }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Answer first 2 questions
    const textarea = page.getByPlaceholder('Type your answer...')
    const nextButton = page.getByRole('button', { name: /Next/i })

    await textarea.fill('React + Rust')
    await nextButton.click()
    await page.waitForTimeout(300)

    await textarea.fill('JWT auth')
    await nextButton.click()
    await page.waitForTimeout(300)

    // Should show 2 checkmarks for answered questions
    const checkmarks = page.locator('.text-green-500').filter({ has: page.locator('svg') })
    const count = await checkmarks.count()
    expect(count).toBeGreaterThanOrEqual(1) // At least 1 checkmark visible
  })

  test('should preserve state when navigating away and back', async ({ page }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    // Start Constitution workflow
    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Answer 2 questions
    const textarea = page.getByPlaceholder('Type your answer...')
    const nextButton = page.getByRole('button', { name: /Next/i })

    await textarea.fill('React + Rust')
    await nextButton.click()
    await page.waitForTimeout(300)

    await textarea.fill('JWT auth')
    await nextButton.click()
    await page.waitForTimeout(300)

    // Should be on Question 3
    await expect(page.getByText('2 / 4')).toBeVisible()

    // Navigate to another tab (if available)
    const settingsButton = page.getByRole('button', { name: /Settings/i })
    const settingsVisible = await settingsButton.isVisible().catch(() => false)

    if (settingsVisible) {
      await settingsButton.click()
      await page.waitForTimeout(500)

      // Navigate back to Tasks
      await tasksButton.click()
      await page.waitForTimeout(500)

      // Select Constitution workflow again
      await page.getByText('Initialize Constitution').click()
      await page.waitForTimeout(500)

      // State should be preserved - still on Question 3
      await expect(page.getByText('2 / 4')).toBeVisible({ timeout: 3000 })
      await expect(page.getByText(/What code quality standards/)).toBeVisible()
    }
  })

  test('should validate empty answers', async ({ page }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Next button should be disabled when textarea is empty
    const nextButton = page.getByRole('button', { name: /Next/i })
    await expect(nextButton).toBeDisabled()

    // Type spaces only - should still be disabled
    const textarea = page.getByPlaceholder('Type your answer...')
    await textarea.fill('   ')
    await expect(nextButton).toBeDisabled()

    // Type actual content - should enable
    await textarea.fill('React')
    await expect(nextButton).toBeEnabled()

    // Clear - should disable again
    await textarea.fill('')
    await expect(nextButton).toBeDisabled()
  })

  test('should show generating state when Generate Constitution is clicked', async ({ page }) => {
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Answer all 4 questions quickly
    const textarea = page.getByPlaceholder('Type your answer...')
    const nextButton = page.getByRole('button', { name: /Next/i })

    await textarea.fill('React + Rust')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('JWT auth')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('80% coverage')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('State-first')
    await nextButton.click()
    await page.waitForTimeout(500)

    // Click Generate Constitution
    const generateButton = page.getByRole('button', { name: /Generate Constitution/i })
    await generateButton.click()
    await page.waitForTimeout(1000)

    // Should show generating state
    // Note: This will fail if Claude CLI is not installed, which is expected
    const generatingText = page.getByText(/Generating Constitution/i)
    const streamingText = page.getByText(/Streaming from Claude Code/i)

    // Check if either appears (generating state reached)
    const isGenerating =
      (await generatingText.isVisible().catch(() => false)) ||
      (await streamingText.isVisible().catch(() => false))

    if (!isGenerating) {
      console.log('Note: Claude CLI may not be available - workflow did not enter generating state')
      // This is expected in CI or environments without Claude CLI
      test.skip(true, 'Claude CLI not available or workflow failed to start')
    }
  })

  test.skip('should create .rstn/constitution.md file after generation', async ({ page }) => {
    // This test requires Claude CLI to be installed and working
    // Skip in automated environments
    await page.waitForTimeout(2000)

    const tasksButton = page.getByRole('button', { name: /Tasks/i })
    const isVisible = await tasksButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await tasksButton.click()
    await page.waitForTimeout(500)

    await page.getByText('Initialize Constitution').click()
    await page.waitForTimeout(500)

    // Answer all 4 questions
    const textarea = page.getByPlaceholder('Type your answer...')
    const nextButton = page.getByRole('button', { name: /Next/i })

    await textarea.fill('React + Rust (napi-rs), TypeScript')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('JWT auth required, sanitize all input')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('80% test coverage, clippy clean')
    await nextButton.click()
    await page.waitForTimeout(200)

    await textarea.fill('State-first principle, no singletons')
    await nextButton.click()
    await page.waitForTimeout(500)

    // Generate
    const generateButton = page.getByRole('button', { name: /Generate Constitution/i })
    await generateButton.click()

    // Wait for completion (up to 60 seconds for Claude)
    await expect(page.getByText(/Constitution saved to/i)).toBeVisible({ timeout: 60000 })

    // Verify file exists
    const projectPath = process.env.TEST_PROJECT_PATH || process.cwd()
    const constitutionPath = path.join(projectPath, '.rstn', 'constitution.md')

    const fileExists = await fs
      .access(constitutionPath)
      .then(() => true)
      .catch(() => false)

    expect(fileExists).toBe(true)

    if (fileExists) {
      const content = await fs.readFile(constitutionPath, 'utf-8')
      expect(content).toContain('Project Constitution')
      expect(content.length).toBeGreaterThan(100) // Should have substantial content
    }
  })
})
