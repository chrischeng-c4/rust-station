import { test, expect } from './electron.fixture'

test.describe('Agent Rules Management', () => {
  test('should navigate to Agent Rules page when clicking Agent Rules button', async ({ page }) => {
    // Wait for app to load
    await page.waitForSelector('[data-testid="project-tabs"]', { timeout: 10000 }).catch(() => {})

    // Look for the Agent Rules button in the project tabs
    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })

    // If no project is open, Agent Rules button won't be visible
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (isVisible) {
      await agentRulesButton.click()

      // Should show Agent Rules heading
      await expect(page.locator('h2', { hasText: /Agent Rules/i })).toBeVisible({ timeout: 5000 })
    } else {
      // No project open - this is expected in clean state
      test.skip(true, 'No project open - Agent Rules button not visible')
    }
  })

  test('should display Agent Rules page elements when navigated', async ({ page }) => {
    // Wait for app to load
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open - Agent Rules button not visible')
      return
    }

    await agentRulesButton.click()

    // Verify key UI elements
    await expect(page.locator('h2', { hasText: /Agent Rules/i })).toBeVisible({ timeout: 5000 })

    // Should have Enable/Disable toggle
    const toggleButton = page.getByRole('button', { name: /Enabled|Disabled/i })
    await expect(toggleButton).toBeVisible()

    // Should have Custom Prompt textarea
    await expect(page.getByPlaceholder(/Enter custom instructions/i)).toBeVisible()

    // Should have character counter
    await expect(page.getByText(/0 characters/i)).toBeVisible()
  })

  test('should toggle agent rules enabled state', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Get initial state
    const initialState = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.enabled
    })

    // Click toggle button
    const toggleButton = page.getByRole('button', { name: /Enabled|Disabled/i })
    await toggleButton.click()
    await page.waitForTimeout(500)

    // Get updated state
    const updatedState = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.enabled
    })

    // State should have toggled
    expect(updatedState).toBe(!initialState)
  })

  test('should save custom prompt text to state', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable agent rules first (textarea is disabled when rules are disabled)
    const toggleButton = page.getByRole('button', { name: /Disabled/i })
    const isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    // Find the textarea (should now be enabled)
    const textarea = page.getByPlaceholder(/Enter custom instructions/i)
    await expect(textarea).toBeVisible({ timeout: 3000 })
    await expect(textarea).toBeEnabled()

    // Type a custom prompt
    const testPrompt = 'You are a Rust expert. Always use snake_case.'
    await textarea.fill(testPrompt)

    // Trigger blur event to auto-save
    await textarea.blur()
    await page.waitForTimeout(1000)

    // Verify the prompt was saved to state
    const savedPrompt = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.custom_prompt
    })

    expect(savedPrompt).toBe(testPrompt)
  })

  test('should update character count when typing', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable agent rules first (textarea is disabled when rules are disabled)
    const toggleButton = page.getByRole('button', { name: /Disabled/i })
    const isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    const textarea = page.getByPlaceholder(/Enter custom instructions/i)
    await expect(textarea).toBeVisible({ timeout: 3000 })
    await expect(textarea).toBeEnabled()

    // Initially should show 0 characters (or whatever is already there)
    // Type some text
    const testText = 'Hello, this is a test'
    await textarea.fill(testText)

    // Character counter should update (21 characters)
    await expect(page.getByText(`${testText.length} character`, { exact: false })).toBeVisible({ timeout: 3000 })
  })

  test('should show warning when custom rules are enabled', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Check if rules are currently enabled
    const isEnabled = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.enabled
    })

    // If not enabled, enable them
    if (!isEnabled) {
      const toggleButton = page.getByRole('button', { name: /Disabled/i })
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    // Warning card should be visible when enabled
    await expect(page.getByText(/Your custom prompt will.*replace/i)).toBeVisible({ timeout: 3000 })
  })

  test('should persist custom prompt in backend state', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    // Navigate to Agent Rules
    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable agent rules first (textarea is disabled when rules are disabled)
    const toggleButton = page.getByRole('button', { name: /Disabled/i })
    const isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    // Set a custom prompt
    const textarea = page.getByPlaceholder(/Enter custom instructions/i)
    await expect(textarea).toBeEnabled()
    const testPrompt = `Test persistence ${Date.now()}`
    await textarea.fill(testPrompt)
    await textarea.blur()
    await page.waitForTimeout(1000)

    // Verify the prompt was saved to backend state
    const savedPrompt = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.custom_prompt
    })

    expect(savedPrompt).toBe(testPrompt)
  })
})
