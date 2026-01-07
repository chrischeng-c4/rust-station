import '@testing-library/jest-dom'
import { vi } from 'vitest'

// Mock window.stateApi (state-first API)
const mockStateApi = {
  dispatch: vi.fn().mockResolvedValue(undefined),
  getState: vi.fn().mockResolvedValue('{}'),
  onStateUpdate: vi.fn().mockReturnValue(() => {}),
}

// Mock window.dialogApi
const mockDialogApi = {
  openFolder: vi.fn().mockResolvedValue(null),
}

// Mock clipboard API
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn().mockResolvedValue(undefined),
  },
})

// Assign mocks to window
Object.defineProperty(window, 'stateApi', {
  value: mockStateApi,
  writable: true,
})

Object.defineProperty(window, 'dialogApi', {
  value: mockDialogApi,
  writable: true,
})

// Export mocks for test files to access
export { mockStateApi, mockDialogApi }
