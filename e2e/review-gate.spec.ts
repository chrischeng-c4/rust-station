import { test, expect } from './electron.fixture'
import path from 'path'
import { openProject, getAppState } from './test-helpers'

test.describe('ReviewGate Workflow', () => {
  let testProjectPath: string

  test.beforeEach(async ({ page }) => {
    // Use the actual rustation project
    testProjectPath = path.resolve(__dirname, '..')
    await openProject(page, testProjectPath)

    // Wait for initial load
    await page.waitForTimeout(1000)
  })

  test('should display review content and controls when a session is active', async ({ page }) => {
    // 1. Start a review session by injecting a change with a linked session
    // We need to do this via state manipulation since the StartReview action is internal
    // However, CreateChange -> GenerateProposal -> CompleteProposal triggers StartReview
    // So let's simulate that flow
    
    // Create a change
    await page.evaluate(async () => {
      await (window as any).stateApi.dispatch({
        type: 'CreateChange', 
        payload: { intent: 'Test ReviewGate Flow' }
      })
    })
    
    // Wait for change to be created and selected
    await page.waitForTimeout(1000)

    // Navigate to Change Management to see the change
    await page.getByRole('button', { name: 'Change Management' }).first().click()
    await page.waitForTimeout(500)
    
    await expect(page.getByText('Test ReviewGate Flow')).toBeVisible()
    
    // Get the change ID
    const changeId = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const state = JSON.parse(json)
      const project = state.projects[state.active_project_index]
      const worktree = project.worktrees[project.active_worktree_index]
      return worktree.changes.changes[0].id
    })
    
    // Manually trigger CompleteProposal to start the review
    // This requires simulating the streaming output first
    await page.evaluate(async (id) => {
      await (window as any).stateApi.dispatch({
        type: 'AppendProposalOutput', 
        payload: { 
          change_id: id, 
          content: '# Test Proposal\n\nThis is a test proposal for review.' 
        }
      })
      
      await (window as any).stateApi.dispatch({
        type: 'CompleteProposal', 
        payload: { change_id: id }
      })
    }, changeId)
    
    // Navigate to Change Management to see the change
    await page.getByRole('button', { name: 'Change Management' }).first().click()
    await page.waitForTimeout(1000)

    // Verify we are on Change Management page
    await expect(page.getByRole('heading', { name: 'Change Management' }).first()).toBeVisible()
    
    // Select the change
    await page.getByText('Test ReviewGate Flow').click({ force: true })
    await page.waitForTimeout(1000)

    // Ensure Proposal tab is active
    await expect(page.getByRole('tab', { name: 'Proposal' })).toBeVisible()
    // Check aria-selected on the tab (it might be a button or div if custom styled, but role=tab is standard)
    // If it fails, we might need to check class or other attributes
    // But visibility check passed in previous run? No, it failed "element not found".
    
    // 2. Verify Review UI
    // Should see the proposal content - use a regex to be more flexible
    await expect(page.getByText(/Test Proposal/)).toBeVisible()
    
    // Should see "Review required" inline control
    await expect(page.getByText('Review required')).toBeVisible()
    
    // Should see Approve/Reject buttons
    await expect(page.getByRole('button', { name: 'Approve' })).toBeVisible()
    await expect(page.getByRole('button', { name: 'Reject' })).toBeVisible()
    
    // 3. Test Approval
    // Click Approve
    await page.getByRole('button', { name: 'Approve' }).click()
    
    // Status should change
    // Since we don't have a real backend to process the approval fully (it just updates state),
    // we verify the state update
    
    // Wait for state update
    await page.waitForTimeout(500)
    
    const sessionStatus = await page.evaluate(async () => {
      const json = await (window as any).stateApi.getState()
      const state = JSON.parse(json)
      const project = state.projects[state.active_project_index]
      const worktree = project.worktrees[project.active_worktree_index]
      const sessions = worktree.tasks.review_gate.sessions
      const sessionId = Object.keys(sessions)[0]
      return sessions[sessionId].status
    })
    
    expect(sessionStatus).toBe('approved')
    
    // UI should update (Review required banner should disappear)
    await expect(page.getByText('Review required')).not.toBeVisible()
  })
})
