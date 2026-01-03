import { test, expect } from './electron.fixture'
import path from 'path'
import { openProject, captureConsoleErrors } from './test-helpers'

test.describe('Screenshots - Workflows Page', () => {
  let testProjectPath: string

  test.beforeEach(async ({ page }) => {
    testProjectPath = path.resolve(__dirname, '..')
    await openProject(page, testProjectPath)
    await page.waitForTimeout(1000)
  })

  test('capture empty state screenshots for consistency check', async ({ page }) => {
    // Use a clean project with no existing data
    const cleanProjectPath = '/tmp/test-empty-state'
    await openProject(page, cleanProjectPath)
    await page.waitForTimeout(1000)

    // Create screenshots directory
    const screenshotsDir = path.resolve(__dirname, 'screenshots')

    // Screenshot 1: Constitution Setup Panel (empty state)
    await page.getByText('Constitution Setup').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, 'empty-01-constitution.png'),
      fullPage: true,
    })

    // Screenshot 2: Living Context Panel (empty state)
    await page.getByText('Living Context').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, 'empty-02-context.png'),
      fullPage: true,
    })

    // Screenshot 3: Change Management Panel (empty state)
    await page.getByText('Change Management').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, 'empty-03-change-management.png'),
      fullPage: true,
    })

    console.log('Empty state screenshots saved to:', screenshotsDir)
  })

  test('capture workflows page screenshots', async ({ page }) => {
    // Create screenshots directory
    const screenshotsDir = path.resolve(__dirname, 'screenshots')

    // Screenshot 1: Workflows Page Overview
    await page.screenshot({
      path: path.join(screenshotsDir, '01-workflows-overview.png'),
      fullPage: true,
    })

    // Screenshot 2: Constitution Setup Panel
    await page.getByText('Constitution Setup').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, '02-constitution-panel.png'),
      fullPage: true,
    })

    // Screenshot 3: Living Context Panel
    await page.getByText('Living Context').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, '03-context-panel.png'),
      fullPage: true,
    })

    // Screenshot 4: Change Management Panel
    await page.getByText('Change Management').click()
    await page.waitForTimeout(500)
    await page.screenshot({
      path: path.join(screenshotsDir, '04-change-management.png'),
      fullPage: true,
    })

    // Create a new change to test ChangeDetailView
    const newButton = page.getByRole('button', { name: /New/i })
    if (await newButton.isVisible()) {
      await newButton.click()
      await page.waitForTimeout(300)

      // Screenshot 5: New Change Dialog
      await page.screenshot({
        path: path.join(screenshotsDir, '05-new-change-dialog.png'),
        fullPage: true,
      })

      // Fill in the intent and create
      const intentInput = page.getByPlaceholder(/describe/i).or(page.locator('textarea'))
      if (await intentInput.isVisible()) {
        await intentInput.fill('Test feature for screenshot - add user authentication')
        await page.waitForTimeout(200)

        const createButton = page.getByRole('button', { name: /Create/i })
        if (await createButton.isVisible()) {
          await createButton.click()
          await page.waitForTimeout(1000)

          // Screenshot 6: Change List with new change
          await page.screenshot({
            path: path.join(screenshotsDir, '06-change-list.png'),
            fullPage: true,
          })

          // Click on the change card to select it
          const changeCard = page.locator('.border').filter({ hasText: 'test-feature' }).first()
          if (await changeCard.isVisible()) {
            await changeCard.click()
            await page.waitForTimeout(500)

            // Screenshot 7: Change Detail View
            await page.screenshot({
              path: path.join(screenshotsDir, '07-change-detail-view.png'),
              fullPage: true,
            })

            // Click on Context Files section to expand it
            const contextFilesSection = page.getByText('Context Files')
            if (await contextFilesSection.isVisible()) {
              await contextFilesSection.click()
              await page.waitForTimeout(300)

              // Screenshot 8: Context Files Expanded
              await page.screenshot({
                path: path.join(screenshotsDir, '08-context-files-expanded.png'),
                fullPage: true,
              })
            }
          }
        }
      }
    }

    console.log('Screenshots saved to:', screenshotsDir)
  })
})
