import { test, expect } from './electron.fixture'

test.describe('Agent Rules Management', () => {
  test('should navigate to Agent Rules page when clicking Agent Rules button', async ({
    page,
  }) => {
    // Wait for app to load
    await page.waitForSelector('[data-testid="project-tabs"]', { timeout: 10000 }).catch(() => {})

    // Look for the Agent Rules button in the project tabs
    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })

    // If no project is open, Agent Rules button won't be visible
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (isVisible) {
      await agentRulesButton.click()

      // Should show Agent Rules heading
      await expect(page.locator('h2', { hasText: /Agent Rules/i })).toBeVisible({
        timeout: 5000,
      })
    } else {
      // No project open - this is expected in clean state
      test.skip(true, 'No project open - Agent Rules button not visible')
    }
  })

  test('should display 3 built-in profiles by default', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Check for built-in profiles in the All Profiles section
    await expect(page.getByText('Rust Expert')).toBeVisible({ timeout: 3000 })
    await expect(page.getByText('TypeScript Expert')).toBeVisible({ timeout: 3000 })
    await expect(page.getByText('Code Reviewer')).toBeVisible({ timeout: 3000 })

    // Verify they have built-in badges
    const rustExpertCard = page.locator('text=Rust Expert').locator('..')
    await expect(rustExpertCard.getByText('Built-in')).toBeVisible()
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

  test('should select a built-in profile', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable agent rules first
    const toggleButton = page.getByRole('button', { name: /Disabled/i })
    const isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    // Click the profile selector
    const selector = page.getByRole('button', { name: /Select a profile/i })
    await selector.click()
    await page.waitForTimeout(300)

    // Select Rust Expert
    await page.getByRole('menuitem', { name: /Rust Expert/i }).click()
    await page.waitForTimeout(500)

    // Verify the profile is selected in state
    const selectedProfile = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      const profileId = parsed.projects?.[0]?.agent_rules_config?.active_profile_id
      const profiles = parsed.projects?.[0]?.agent_rules_config?.profiles || []
      return profiles.find((p: any) => p.id === profileId)
    })

    expect(selectedProfile?.name).toBe('Rust Expert')
    expect(selectedProfile?.is_builtin).toBe(true)

    // Should show profile preview
    await expect(page.getByText('Profile Preview')).toBeVisible()
    await expect(page.getByText('snake_case')).toBeVisible() // Part of Rust Expert prompt
  })

  test('should create a new custom profile', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Click "New Profile" button
    await page.getByRole('button', { name: /New Profile/i }).click()
    await page.waitForTimeout(300)

    // Should open dialog
    await expect(page.getByRole('dialog')).toBeVisible()
    await expect(page.getByText('Create New Profile')).toBeVisible()

    // Fill in the form
    await page.getByLabel(/Profile Name/i).fill('Test Custom Profile')
    await page
      .getByLabel(/System Prompt/i)
      .fill('You are a test expert. Always write comprehensive tests.')

    // Click Create Profile
    await page.getByRole('button', { name: /Create Profile/i }).click()
    await page.waitForTimeout(500)

    // Verify the profile was created
    const customProfile = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      const profiles = parsed.projects?.[0]?.agent_rules_config?.profiles || []
      return profiles.find((p: any) => p.name === 'Test Custom Profile')
    })

    expect(customProfile).toBeDefined()
    expect(customProfile.is_builtin).toBe(false)
    expect(customProfile.prompt).toContain('test expert')

    // Should appear in the profile list
    await expect(page.getByText('Test Custom Profile')).toBeVisible()
  })

  test('should edit a custom profile', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Create a profile first
    await page.getByRole('button', { name: /New Profile/i }).click()
    await page.waitForTimeout(300)
    await page.getByLabel(/Profile Name/i).fill('To Edit')
    await page.getByLabel(/System Prompt/i).fill('Original prompt')
    await page.getByRole('button', { name: /Create Profile/i }).click()
    await page.waitForTimeout(500)

    // Find and click the edit button for the custom profile
    const profileCard = page.locator('text=To Edit').locator('..')
    const editButton = profileCard.getByRole('button').first() // First button is edit
    await editButton.click()
    await page.waitForTimeout(300)

    // Should open dialog with "Edit Profile" title
    await expect(page.getByText('Edit Profile')).toBeVisible()

    // Update the profile
    await page.getByLabel(/Profile Name/i).fill('Edited Profile')
    await page.getByLabel(/System Prompt/i).fill('Updated prompt')
    await page.getByRole('button', { name: /Save Changes/i }).click()
    await page.waitForTimeout(500)

    // Verify the profile was updated
    const updatedProfile = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      const profiles = parsed.projects?.[0]?.agent_rules_config?.profiles || []
      return profiles.find((p: any) => p.name === 'Edited Profile')
    })

    expect(updatedProfile).toBeDefined()
    expect(updatedProfile.prompt).toBe('Updated prompt')
  })

  test('should delete a custom profile', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Create a profile to delete
    await page.getByRole('button', { name: /New Profile/i }).click()
    await page.waitForTimeout(300)
    await page.getByLabel(/Profile Name/i).fill('To Delete')
    await page.getByLabel(/System Prompt/i).fill('Will be deleted')
    await page.getByRole('button', { name: /Create Profile/i }).click()
    await page.waitForTimeout(500)

    // Get profile count before delete
    const beforeCount = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.profiles?.length || 0
    })

    // Setup dialog handler for confirmation
    page.on('dialog', (dialog) => dialog.accept())

    // Find and click the delete button
    const profileCard = page.locator('text=To Delete').locator('..')
    const deleteButton = profileCard.getByRole('button').last() // Last button is delete
    await deleteButton.click()
    await page.waitForTimeout(500)

    // Verify the profile was deleted
    const afterCount = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.profiles?.length || 0
    })

    expect(afterCount).toBe(beforeCount - 1)
    await expect(page.getByText('To Delete')).not.toBeVisible()
  })

  test('should not allow editing built-in profiles', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Built-in profiles should not have edit/delete buttons
    const rustExpertCard = page.locator('text=Rust Expert').locator('..')
    const hasEditButton = await rustExpertCard.getByRole('button').count()

    // Built-in profiles should only have "Built-in" text, no action buttons
    expect(hasEditButton).toBe(0)
  })

  test('should show warning when agent rules are enabled with a profile', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable and select a profile
    const toggleButton = page.getByRole('button', { name: /Disabled/i })
    const isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    const selector = page.getByRole('button', { name: /Select a profile/i })
    await selector.click()
    await page.waitForTimeout(300)
    await page.getByRole('menuitem', { name: /Rust Expert/i }).click()
    await page.waitForTimeout(500)

    // Warning card should be visible
    await expect(page.getByText(/Custom Rules Active/i)).toBeVisible({ timeout: 3000 })
    await expect(page.getByText(/will.*replace.*CLAUDE\.md/i)).toBeVisible()
  })

  test('should disable when active profile is set to None', async ({ page }) => {
    await page.waitForTimeout(2000)

    const agentRulesButton = page.getByRole('button', { name: /Agent Rules/i })
    const isVisible = await agentRulesButton.isVisible().catch(() => false)

    if (!isVisible) {
      test.skip(true, 'No project open')
      return
    }

    await agentRulesButton.click()
    await page.waitForTimeout(500)

    // Enable and select a profile first
    let toggleButton = page.getByRole('button', { name: /Disabled/i })
    let isDisabled = await toggleButton.isVisible().catch(() => false)
    if (isDisabled) {
      await toggleButton.click()
      await page.waitForTimeout(500)
    }

    let selector = page.getByRole('button', { name: /Select a profile/i })
    await selector.click()
    await page.waitForTimeout(300)
    await page.getByRole('menuitem', { name: /Rust Expert/i }).click()
    await page.waitForTimeout(500)

    // Now select "None"
    selector = page.getByRole('button', { name: /Rust Expert/i })
    await selector.click()
    await page.waitForTimeout(300)
    await page.getByRole('menuitem', { name: /None \(use CLAUDE\.md\)/i }).click()
    await page.waitForTimeout(500)

    // Verify active_profile_id is cleared
    const activeProfileId = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const parsed = JSON.parse(json)
      return parsed.projects?.[0]?.agent_rules_config?.active_profile_id
    })

    expect(activeProfileId).toBeUndefined()
  })
})
