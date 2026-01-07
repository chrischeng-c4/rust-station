import { test, expect } from './electron.fixture'
import { getAppState } from './test-helpers'

test.describe('Mocked Docker Management', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to Docker view
    await page.getByRole('button', { name: 'Docker' }).click()
    await page.waitForTimeout(500)
  })

  test('should display mocked services', async ({ page }) => {
    // Inject mock services into state
    await page.evaluate(async () => {
      const mockServices = [
        {
          id: 'mock-postgres',
          name: 'Mock PostgreSQL',
          image: 'postgres:16',
          status: 'running',
          port: 5432,
          service_type: 'Database',
          project_group: 'rstn',
          is_rstn_managed: true
        },
        {
          id: 'mock-redis',
          name: 'Mock Redis',
          image: 'redis:alpine',
          status: 'stopped',
          port: 6379,
          service_type: 'Cache',
          project_group: 'other',
          is_rstn_managed: false
        }
      ];

      await (window as any).stateApi.dispatch({
        type: 'SetDockerAvailable',
        payload: { available: true }
      });

      await (window as any).stateApi.dispatch({
        type: 'SetDockerServices',
        payload: { services: mockServices }
      });
    });

    // Check UI for mock services
    await expect(page.getByText('Mock PostgreSQL')).toBeVisible();
    await expect(page.getByText('Mock Redis')).toBeVisible();
    
    // Check status badges
    await expect(page.getByText('Running').first()).toBeVisible();
    await expect(page.getByText('Stopped').first()).toBeVisible();
  });

  test('should show connection string dialog for database', async ({ page }) => {
    // Inject mock services
    await page.evaluate(async () => {
      await (window as any).stateApi.dispatch({
        type: 'SetDockerAvailable',
        payload: { available: true }
      });
      await (window as any).stateApi.dispatch({
        type: 'SetDockerServices',
        payload: { services: [{
          id: 'mock-pg',
          name: 'PostgreSQL',
          image: 'postgres:16',
          status: 'running',
          port: 5432,
          service_type: 'Database',
          project_group: 'rstn',
          is_rstn_managed: true
        }] }
      });
    });

    // Click "Add DB" button
    await page.getByRole('button', { name: 'Add DB' }).click();
    
    // Check dialog
    await expect(page.getByText('Create Database')).toBeVisible();
    await expect(page.getByLabel('Database Name')).toBeVisible();
  });
});
